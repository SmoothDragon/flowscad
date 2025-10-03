use flowscad::*;
use anyhow::Result;

fn circle_bar(d: f32, xy0: XY, xy1: XY) -> D2 {
    let dot = D2::circle_d(d);
    (dot.translate(xy0) + dot.translate(xy1)).hull()
}


fn main() -> Result<()> {
    let d = 2.8;
    let result = circle_bar(d, XY(0., 0.), XY(-d-6., 0.))
        .add(circle_bar(d, XY(-d-6., 0.), XY(-d-6., -3.)))
        .add(circle_bar(d, XY(0., 0.), XY(0., -d-27.)))
        .add(circle_bar(d, XY(-d-6., -d-27.), XY(0., -d-27.)))
        .add(circle_bar(d, XY(10., -d-37.), XY(0., -d-27.)))
        .add(circle_bar(d, XY(-d-6., -d-27.), XY(-d-6., -d-25.)))
        .add(circle_bar(d, XY(0., 0.), XY(40., 40.)))
        .add(circle_bar(d, XY(160., 40.), XY(40., 40.)))
        .add(circle_bar(d, XY(160., 40.), XY(160., 30.)))
        .add(circle_bar(d, XY(180., 45.), XY(160., 30.)))
        .linear_extrude(20)
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

