use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    // let id_hex = X(24.75);
    // let or_hex = X(28.5)/2;
    // let id_hex = or_hex * 3.0_f32.sqrt();
    let id_hex = X(26.);
    let or_hex = id_hex / 3.0_f32.sqrt();
    let d_peg = id_hex/3;
    let d_wall = X(1.0);
    let h_wall = X(13.0);
    let gap = X(0.2);

    let h_base = X(3.0);
    let h_peg = X(3.0);
    // let center_peg = D3::chamfer_cylinder_d(h_base+h_peg, d_peg, 0.5)
    let center_peg = D3::chamfer_regular_polygon_prism(6, h_base+h_peg, d_peg-2.*gap, 0.5)
        .translate_z(-h_base)
        ;
    let base = D2::regular_polygon(6, or_hex)
        .linear_extrude(h_base)
        .add(center_peg)
        ;
    let wall = D3::chamfer_regular_polygon_prism(6, h_wall, d_wall, 0.5)
        .add_map(|x| x.translate_x(or_hex))
        .hull()
        .translate( (-or_hex/2, id_hex/2, 0) )
        ;
    let wall1 = wall.clone() + wall.clone().rotate_z(60) + wall.rotate_z(180);
    let wall2 = wall.clone() + wall.clone().rotate_z(60) + wall.rotate_z(120);
    let wall3 = wall.clone() + wall.clone().rotate_z(120) + wall.rotate_z(180);
    let wall4 = wall.clone() + wall.clone().rotate_z(120) + wall.rotate_z(240);
    let result1 = base.clone() + wall1;
    let result2 = base.clone() + wall2;
    let result3 = base.clone() + wall3;
    let result4 = base.clone() + wall4;
    let shift_x = 2.1*or_hex;
    // let result = result1.translate_x(-2*shift_x)
        // + result2.translate_x(-1*shift_x)
        // + result3.translate_x(1*shift_x)
        // + result4.translate_x(2*shift_x)
        // ;
    // let end_result = (0..6)
        // .map(|x| result.clone()
             // .translate_y((x-3)*shift_x)
             // )
        // .union()
        // ;
    let end_result = result1;
    println!("$fn=64;\n{}", &end_result);
    Ok(())
}
