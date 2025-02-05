use flowscad::*;
use anyhow::Result;


fn heart(side: f32) -> D2 {
    D2::square(side).center()
        .add(D2::circle_d(side).translate_x(side/2.0))
        .add(D2::circle_d(side).translate_y(side/2.0))
        .translate( (side/2.0, side/2.0) )
        .rotate(45.0)
}

fn heart_align_left(obj: D2, side: f32) -> D2 {
    obj.translate( (side * 3.414 /4., -side * 3. * 1.414 / 4.) )
}

fn heart_left(side:f32) -> D2 {
    heart_align_left(heart(side), side)
}


fn three_heart(side: f32, w_line: f32) -> D2 {
    heart_left(4.*side).translate_x(-2.*w_line)
        .sub(heart_left(2.*side))
        .sub(D2::rectangle( (side,0.001) ).center())
        .add(heart_left(side).translate_x(1.707*side))
        .add(D2::rectangle( (side, 2.*w_line) ).center().translate_x(3.*side))
}


fn main() -> Result<()> {
    let side = 8.;
    let result = three_heart(side, 0.4)
        .linear_extrude(10.)
        ;
    println!("$fn=256;\n{}", &result);
    Ok(())
}
