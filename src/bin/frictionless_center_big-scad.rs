use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let or_hex = X(28.5)/2;
    let d_peg = X(5.5);
    let h_base = X(3.0);
    let h_peg = X(3.0);
    let inner_spread = X(44.1);
    // let base = D2::regular_polygon(6, or_hex)
    let base = D3::chamfer_regular_polygon_prism(6, h_base, 14.4, 1)
        .rotate_z(30)
        // .linear_extrude(h_base)
        .add(D3::cylinder_d(h_base, d_peg))
        .translate_x((inner_spread+d_peg)/2)
        .iter_rotate((0,0,60), 6)
        .union()
        .add(D3::chamfer_regular_polygon_prism(6, h_base, 14.4, 1)
             .rotate_z(30))
        ;
    let leg = D3::chamfer_cylinder_d(h_base+h_peg, d_peg, 1.2)
        .translate_x((inner_spread+d_peg)/2)
        .iter_rotate((0,0,60), 6)
        .union()
        ;
    let center = D3::chamfer_regular_polygon_prism(6, h_base+h_peg, 14.2, 1)
        .rotate((0,0,30))
        ;
    let counts = (1..=6)
        .map(|x| D2::text(x.to_string())
             .rotate(30)
             .translate_y(-inner_spread/1.8)
             .rotate(30+x*60))
        .union()
        .linear_extrude(10)
        .translate_z(-h_peg/2)
        .rotate_y(180)
        ;
    // let base = D2::regular_polygon(6, or_hex)
        // .linear_extrude(h_base)
        // .add(center_peg)
        // ;
    let result = base + leg + center - counts;
    println!("$fn=64;\n{}", &result);
    Ok(())
}
