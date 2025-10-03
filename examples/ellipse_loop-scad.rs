use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let diameter = 60.;
    let base = D2::circle_d(diameter);
    let n_layers = 70;
    let layer = 0.1;

    let result = (0..n_layers)
        .map(|x| {let xx: X = x*X(1.0); 
             base.clone()
                 .scale_xy( XY((diameter-0.05*xx.0)/diameter, (diameter+0.05*xx.0)/diameter) )
                 .linear_extrude(layer)
                 .translate_z(layer*xx)
            })
        .union()
        .add_map(|x| x.mirror( (0,0,1) ).rotate_z(90))
        ;

    println!("$fn=64;\n{}", &result);
    Ok(())
}

