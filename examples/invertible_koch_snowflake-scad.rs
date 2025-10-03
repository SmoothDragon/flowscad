use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let r = 50.;
    let result = D2::hexagram(r)
        .rotate(90)
        .repeat_map(2, |x| x.add_map(|y|
            y.scale(1./3.)
            .translate_y(2.*r/3.)
            .iter_rotate_equal(6)
            .union()
            )
        )
        .linear_extrude(10)
        ;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

