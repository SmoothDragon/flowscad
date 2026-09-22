use flowscad::*;
use anyhow::Result;


fn nested_circles(r: f32, w_line: f32) -> D2 {
    let epsilon: f32 = 0.001;
    D2::circle_r(4.*r)
        .sub(D2::circle_r(2.*r+2.*w_line).translate_x(-2.*r + 4.*w_line))
        .sub(D2::rectangle( (2.*r, epsilon) ).translate_x(-5.*r))
        .add(D2::circle_r(r).translate_x(-r+6.*w_line))
        .add(D2::rectangle( (2.*r, 2.*w_line) ).translate((-r,-0.5*w_line)))
}


fn main() -> Result<()> {
    let side = 8.;
    let result = nested_circles(side, 0.4)
        .linear_extrude(10.)
        ;
    println!("$fn=256;\n{}", &result);
    Ok(())
}
