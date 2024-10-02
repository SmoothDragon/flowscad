use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let diameter = X(50.);
    let base = D2::circle_d(diameter)
        .add_map(|x| x.translate_x(diameter*0.75))
        ;
    let n_layers = 70;
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

