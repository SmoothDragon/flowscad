use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let r_oct = 10.;
    let r_face = 0.5*r_oct;
    let text_depth = 0.8;
    let base = D3::octahedron(r_oct)
        .intersection(D3::sphere_r(0.707*r_oct))  // corner edges between faces
        // .intersection(D3::sphere_r(0.68*r_oct))
        .rotate_z(45)
        .rotate_y(109.47/2.0)
        ;
    let side1 = D2::text("1".to_string())
        .scale(0.5)
        .linear_extrude(text_depth)
        .translate_z(r_face)
        .add(D2::text("8".to_string())
            .scale(0.5)
            .linear_extrude(text_depth)
            .rotate_y(180)
            .translate_z(-r_face)
            )
        .rotate_y(-180.0+109.47)
        ;
    let side2 = D2::text("2".to_string())
        .scale(0.5)
        .linear_extrude(text_depth)
        .translate_z(r_face)
        .add(D2::text("7".to_string())
            .scale(0.5)
            .linear_extrude(text_depth)
            .rotate_y(180)
            .translate_z(-r_face)
            )
        .rotate( (0, -180.0+109.47, 120) )
        ;
    let side3 = D2::text("3".to_string())
        .scale(0.5)
        .linear_extrude(text_depth)
        .translate_z(r_face)
        .add(D2::text("6".to_string())
            .scale(0.5)
            .linear_extrude(text_depth)
            .rotate_y(180)
            .translate_z(-r_face)
            )
        .rotate( (0, -180.0+109.47, 240) )
        ;
    let side4 = D2::text("4".to_string())
        .scale(0.5)
        .linear_extrude(text_depth)
        .translate_z(r_face)
        .add(D2::text("5".to_string())
            .scale(0.5)
            .linear_extrude(text_depth)
            .rotate_y(180)
            .translate_z(-r_face)
            )
        ;


    let result = base - (side1 +side2 + side3 + side4);
    println!("$fn=128;\n{}", &result);
    Ok(())
}
