use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let diameter = 50.;
    let ratio = 2;
    let result = D2::circle_d(diameter)
        .scale_x(ratio)
        .linear_extrude(8)
        ;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

