use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let d_peg = X(5.6);
    let h_base = X(3.0);
    let h_peg = X(3.0);
    let inner_spread = X(29.4);
    let base = D3::cylinder_d(h_base, d_peg)
        .translate_x((inner_spread+d_peg)/2)
        .iter_rotate((0,0,60), 6)
        .hull()
        ;
    let leg = D3::chamfer_cylinder_d(h_base+h_peg, d_peg, 1)
        .translate_x((inner_spread+d_peg)/2)
        .iter_rotate((0,0,60), 6)
        .union()
        ;
    let center = D3::chamfer_regular_polygon_prism(6, h_base+h_peg, 10, 1)
        .rotate((0,0,30))
        ;
    let counts = (1..=6)
        .map(|x| D2::text(x.to_string())
             .translate_y(-inner_spread/1.6)
             .rotate(30+x*60))
        .union()
        .linear_extrude(10)
        .translate_z(-h_peg/2)
        .rotate_y(180)
        ;
    let result = base + leg + center - counts;
    println!("$fn=64;\n{}", &result);
    Ok(())
}
