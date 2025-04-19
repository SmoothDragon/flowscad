use flowscad::*;
use anyhow::Result;

fn version1() -> D3 {
    let id_hex = X(26.);
    let or_hex = id_hex / 3.0_f32.sqrt();
    // let or_hex = X(28.5)/2;
    let d_peg = or_hex/3;
    let d_wall = X(1.0);
    let h_wall = X(13.0);
    let gap = X(0.4);
    // let d_peg = X(5.5);
    let h_base = X(3.0);
    let h_peg = X(3.0);
    let inner_spread = X(44.1);
    // let base = D2::regular_polygon(6, or_hex)
    let base = D3::chamfer_regular_polygon_prism(6, h_base, or_hex, 1)
        .rotate_z(30)
        // .linear_extrude(h_base)
        .add(D3::cylinder_d(h_base, d_peg))
        .translate_x((inner_spread+d_peg)/2)
        .iter_rotate((0,0,60), 6)
        .union()
        .add(D3::chamfer_regular_polygon_prism(6, h_base, or_hex, 1)
             .rotate_z(30))
        ;
    let leg = D3::chamfer_regular_polygon_prism(6, h_base+h_peg, d_peg-2*gap, 1.2)
        .rotate_z(30)
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
    return base + leg - counts;
}

fn version2() -> D3 {
    let id_hex = X(26.);
    let or_hex = id_hex / 3.0_f32.sqrt();
    // let or_hex = X(28.5)/2;
    let d_peg = or_hex/3;
    let d_wall = X(1.0);
    let h_wall = X(13.0);
    let gap = X(0.4);
    // let d_peg = X(5.5);
    let h_base = X(3.0);
    let h_peg = X(3.0);
    let inner_spread = X(47.);
    // let base = D2::regular_polygon(6, or_hex)
    let base = D3::chamfer_regular_polygon_prism(6, h_base, or_hex, 1)
        .rotate_z(30)
        // .linear_extrude(h_base)
        .add(D3::cylinder_d(h_base, d_peg))
        .translate_x((inner_spread+d_peg)/2)
        .iter_rotate((0,0,60), 6)
        .union()
        .add(D3::chamfer_regular_polygon_prism(6, h_base, or_hex, 1)
             .rotate_z(30))
        ;
    let leg = D3::chamfer_regular_polygon_prism(6, h_base+h_peg, d_peg-2*gap, 1.2)
        .rotate_z(30)
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
    return base + leg - counts;
}


fn main() -> Result<()> {
    let id_hex = X(6.);
    let or_hex = id_hex / 3.0_f32.sqrt();
    let h = X(150.);
    let chamfer = X(1.0);
    let gap = X(0.1);
    let base = D3::chamfer_regular_polygon_prism(6, 2.8, or_hex-gap/2, 1)
        .translate( (-6*or_hex, -6*or_hex*3_f32.sqrt(), 0) )
        .iter_rotate((0,0,60), 6)
        .pairs()
        .map(|(a,b)| (a+b).hull())
        .union()
        ;
    let legs = D3::chamfer_regular_polygon_prism(6, h, or_hex-gap/2, 1)
        .iter_translate( (4*or_hex, 0, 0), 4)
        .union()
        .translate( (-6*or_hex, -6*or_hex*3_f32.sqrt(), 0) )
        .iter_rotate((0,0,60), 6)
        .union()
        ;
    // let counts = (1..=6)
        // .map(|x| D2::text(x.to_string())
             // .rotate(30)
             // .translate_y(-inner_spread/1.8)
             // .rotate(30+x*60))
        // .union()
        // .linear_extrude(10)
        // .translate_z(-h_peg/2)
        // .rotate_y(180)
        // ;
    // let base = D2::regular_polygon(6, or_hex)
        // .linear_extrude(h_base)
        // .add(center_peg)
        // ;
    let result = base + legs;
    // let result = version2();
    println!("$fn=64;\n{}", &result);
    Ok(())
}
