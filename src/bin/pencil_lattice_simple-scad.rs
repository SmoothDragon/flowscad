use flowscad::*;
use anyhow::Result;



fn main() -> Result<()> {
    let or_hex = X(5.);
    let h = X(120.);
    // let chamfer = X(1.0);
    let gap = X(0.2);
    let base = D2::hexagon(or_hex - gap).linear_extrude(2.8)
    // let base = D3::chamfer_regular_polygon_prism(6, 2.8, or_hex-gap/2, 1)
        .translate_x(4*or_hex)
        .translate( (-2*or_hex, -2*or_hex*3_f32.sqrt(), 0) )
        // .translate( (-6*or_hex, -6*or_hex*3_f32.sqrt(), 0) )
        .iter_rotate((0,0,60), 6)
        .pairs()
        .map(|(a,b)| (a+b).hull())
        .union()
        ;
    let legs = D2::hexagon(or_hex - gap).linear_extrude(h*0.8)
        .add(D2::hexagon(or_hex/6 - gap).linear_extrude(h))
        .hull()
        // .add_map(|x| x.translate_x(4*or_hex))
        .translate_x(4*or_hex)
        .translate( (-2*or_hex, -2*or_hex*3_f32.sqrt(), 0) )
        .iter_rotate((0,0,60), 6)
        .union()
        ;
    // let result = legs;
    let result = base + legs;
    println!("$fn=64;\n{}", &result);
    Ok(())
}
