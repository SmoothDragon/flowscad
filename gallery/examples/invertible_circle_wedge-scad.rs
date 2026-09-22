use anyhow::Result;
use flowscad::*;


fn main() -> Result<()> {
    let theta = 2.0 * PI - (1.0 + PI - (1.0 + PI * PI).sqrt());
    let result = D2::circle_r(1)
        .add(D2::sector(1.0 + theta, theta * 180.0 / PI))
        .scale(10)
        .linear_extrude(10)
         ;
    println!("$fn=256;\n{}", &result);

    Ok(())
}

