use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    // let r_square = 2.0_f64.powf(0.5) * l_edge;  // height of truncated octahedron between square faces
    let r_oct = 10.;
    // let base = D3::truncated_octahedron(l_edge)
    let base = D3::octahedron(r_oct)
        .intersection(D3::sphere_r(0.707*r_oct))
        .rotate_z(45)
        .rotate_y(109.47/2.0)
        // .add(D3::cylinder_d(40, 0.707*r_oct))
        ;
    let stamp = D2::regular_polygon(6, 0.35*r_oct)
        .linear_extrude(40)
        ;
    let l_edge = r_oct/4.0;
    let id_hex = l_edge * 3.0_f64.sqrt() / 2.0;
    let target = D3::chamfer_regular_polygon_prism(6, 1, 0.5, 0.25)
        .add_map(|x| x.translate_x(l_edge))
        .hull()
        .translate( (-l_edge/2.0, id_hex, 0) )
        .translate_z(0.5*r_oct)
        .add_map(|x| x.rotate( (180,0,0) ))
        ;
    let side1 = target.clone()
        .iter_rotate( (0,0,60), 3 )
        .union()
        .rotate_y(-180.0+109.47)
        ;
    let side2 = target.clone()
        .iter_rotate( (0,0,120), 3 )
        .union()
        .rotate( (0, -180.0+109.47, 120) )
        ;
    let side3 = (target.clone() + target.clone().rotate_z(60) + target.clone().rotate_z(180))
        .rotate( (0, -180.0+109.47, 240) )
        ;

    let side4 = target.clone() + target.clone().rotate_z(60) + target.clone().rotate_z(240)
        ;

    let result = base - (side1 +side2 + side3 + side4);
    println!("$fn=128;\n{}", &result);
    Ok(())
}
