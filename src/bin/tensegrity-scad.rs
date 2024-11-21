use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let unit = X(5.);
    let bottom = XY(10.,1.) * unit;
    let vert = XY(1.,7.) * unit;
    let mid_cross = XY(4.,1.) * unit;
    let side = (
        D2::rectangle(bottom)
        + D2::rectangle(vert).translate_x(7*unit)
        + D2::rectangle(mid_cross).translate( v2(4,6)*unit )
        )
        .translate( v2(-5,-5) * unit)
        .add_map(|x| x.rotate(180))
        .linear_extrude(unit)
        .translate( v2(5,5) * unit)
        ;
    let side_string_hole = D2::rectangle( (0.6, 0.2) )
        .center()
        .linear_extrude(unit * 10 - 0.8)
        .translate_z(0.4)
        .rotate_x(-90)
        .translate( v3(1,0,0.5) * unit)
        ;
    let side_string = D2::rectangle( (0.4, 0.2) )
        .center()
        .linear_extrude(unit * 10)
        .rotate_x(-90)
        .translate( v3(1,0,0.5) * unit)
        ;

    let mid_string_hole = D2::rectangle( (0.6, 0.2) )
        .center()
        .linear_extrude(unit * 4 - 0.8)
        .translate_z(0.4)
        .rotate_x(-90)
        .translate( v3(5,3,0.5) * unit)
        ;
    let mid_string = D2::rectangle( (0.4, 0.2) )
        .center()
        .linear_extrude(unit * 4)
        .rotate_x(-90)
        .translate( v3(5,3,0.5) * unit)
        ;

    let piece = side 
        - side_string_hole.clone()
        - side_string_hole.translate_x(8*unit)
        - mid_string_hole
        + side_string.clone()
        + side_string.translate_x(8*unit)
        + mid_string
        ;
    let result = piece.iter_translate( v3(0,0,1)*unit, 10)
        .union()
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}
