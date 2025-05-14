use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let d = 50.;
    let l_edge = d * 2.0_f32.powf(-1.5);
    let h_hex = d*3.0_f32.sqrt()/2.0;  // Height between hexagons
    let h_square = 1.155*d*3.0_f32.sqrt()/2.0;  // Height between squares
    let angle_sh = 125.26439;  // Angle between square and hexagon
    let angle_hh = 109.47122;  // Angle between hexagons
    let diangle_tetra = 70.529;  // Dihedral angle of tetrhedron
    let angle_tetra = 109.5;  // Interior angle of tetrhedron
    /// to xy-plane and an edge length of 2*sqrt(6)/3.

    let koch = D2::koch_snowflake(l_edge, 3)
        .linear_extrude(h_hex)
        // .translate_z(0.1*h_hex)
        .add_map(|x| 
            x.rotate_y(-angle_tetra)
            + x.rotate_y(-angle_hh).rotate_z(120)
            + x.rotate_y(-angle_hh).rotate_z(240)
        )
        .rotate_x(180)
        ;

    // let result = koch;
    // let result = koch + square_koch;
    // let result = square_koch;
    // let result = koch & D3::tetrahedron().scale(50.0/2.*3./6.0_f32.sqrt());
    let result = koch & D3::tetrahedron().scale(32.5);

    println!("$fn=128;\n{}", &result);
    Ok(())
}

