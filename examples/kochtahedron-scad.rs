use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let d = 50.;
    let l_edge = d * 2.0_f32.powf(-1.5);
    let h_hex = d*3.0_f32.sqrt()/2.0;  // Height between hexagons
    let h_square = 1.155*d*3.0_f32.sqrt()/2.0;  // Height between squares
    let angle_sh = 125.26439;  // Angle between square and hexagon
    let angle_hh = 109.47122;  // Angle between hexagons

    let koch = D2::koch_snowflake(l_edge, 2)
        .linear_extrude(h_hex)
        .translate_z(-h_hex*0.5)
        .render()
        .add_map(|x| 
            x.rotate_y(-angle_hh)
            + x.rotate_y(-angle_hh).rotate_z(120)
            + x.rotate_y(-angle_hh).rotate_z(240)
        )
        .render()
        ;

    let square_koch = D2::koch_snowflake(l_edge, 2)
        // .translate_x(-l_edge*0.5)
        .translate_x(-0.75_f32.sqrt()*l_edge)
        .and(D2::square(0.5_f32.sqrt()*l_edge).center().rotate(45).scale_x(1.1))
        .translate_x(0.25_f32.sqrt()*l_edge)
        .iter_rotate_equal(4)
        .union()
        .linear_extrude(h_square)
        .translate_z(-h_square/2.0)
        .rotate_y(angle_sh)
        // .render()
        .iter_rotate((0,0,120), 3)
        .union()
        // .render()
        .add(D3::sphere_r(l_edge))
        ;

    let result = D3::troc_d(d)
        .rotate_z(45)
        // .rotate_x((-1.0_f32/3.0).acos()*180./PI)
        .rotate_y(angle_sh)
        .translate_y(d)
        + koch.clone()
        ;

    // let result = koch;
    let result = koch + square_koch;
    // let result = square_koch;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

