use anyhow::Result;
use flowscad::*;


fn main() -> Result<()> {
    let theta = 2*PI - (1 + PI - (1 + PI*PI).sqrt());
    let result = D2::circle_r(1)
        .add(D2::sector(1 + theta, theta*180/PI))
         ;
    println!("$fn=256;\n{}", &result);

    Ok(())
}

