use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let d = 50.;
    let l_edge = d * 2.0_f32.powf(-1.5);
    let h = d*3.0_f32.sqrt()/2.0;  // Height between hexagons
    let angle_sh = 125.26439;  // Angle between square and hexagon
    let angle_hh = 109.47122;  // Angle between hexagons

    let koch = D2::koch_snowflake(l_edge, 2)
        .linear_extrude(h)
        .translate_z(-h*0.5)
        .add_map(|x| 
            x.rotate_y(-angle_hh)
            + x.rotate_y(-angle_hh).rotate_z(120)
            + x.rotate_y(-angle_hh).rotate_z(240)
        )
        ;

    let result = D3::troc_d(d)
        .rotate_z(45)
        // .rotate_x((-1.0_f32/3.0).acos()*180./PI)
        .rotate_y(angle_sh)
        .translate_y(d)
        + koch.clone()
        ;

    let result = koch;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

