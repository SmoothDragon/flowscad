use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let unit = X(5.);
    let outer_r = X(10.) * unit;
    let bottom = XY(10.,1.) * unit;
    let vert = XY(1.,7.) * unit;
    let mid_cross = XY(4.,1.) * unit;
    let inner = (
        D2::rectangle(vert).translate_x(7*unit)
        + D2::rectangle(mid_cross).translate( v2(4,6)*unit )
        )
        .translate( v2(-5,-5) * unit)
        ;
    let outer = D2::circle_r(outer_r)
        .sub(D2::circle_r(outer_r-unit))
        .translate_y(5*unit)
        .add(inner)
        .intersection(D2::circle_r(outer_r).translate_y(5*unit))
        .intersection(D2::square(10.*unit).center())
        .add_map(|x| x.rotate(180))
        .linear_extrude(unit)
        .translate( v2(5,5) * unit)
        ;
    let side_string = D2::square(0.6)
        .center()
        .linear_extrude(unit * 10)
        .rotate_x(-90)
        .translate( v3(1,0,0.5) * unit)
        // .translate( v2(-5,-5) * unit)
        ;

    let mid_string = D2::square(0.4)
        .center()
        .linear_extrude(unit * 4)
        .rotate_x(-90)
        .translate( v3(5,3,0.5) * unit)
        ;

    let piece = outer
        + side_string.clone()
        + side_string.translate_x(8*unit)
        + mid_string
        ;
    let result = piece.iter_translate( v3(0,0,1)*unit, 10)
        .union()
        .translate( v3(-5,-5,0) * unit)
        .intersection(D3::cylinder_r(10*unit,outer_r)
                      .translate_y(5*unit))
        .intersection(D3::cylinder_r(10*unit,outer_r)
                      .translate_y(-5*unit))
        ;
    println!("$fn=256;\n{}", &result);
    Ok(())
}
