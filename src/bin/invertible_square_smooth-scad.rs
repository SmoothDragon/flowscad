use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let diameter = 60.;
    let base = D2::square(diameter);
    let n_layers = 140;
    let layer = X(0.1);

    let result = (0..n_layers)
        .map(|x| {let xx: X = x*X(1.0); 
             base.clone()
                 .offset_radius(-x*layer/3)
                 .linear_extrude(layer)
                 .translate_z(layer*xx)
            })
        .union()
        ;

    println!("$fn=64;\n{}", &result);
    Ok(())
}

