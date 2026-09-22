use flowscad::*;
use anyhow::Result;
use ::bitperm::*;

fn main() -> Result<()> {
    let h = 142.5;
    let w = 220.0;
    let d = 27.5;
    let gap = 0.2;
    let w_wall = 0.8;
    let od = d + 2.0 * w_wall;
    let outer_shape = D2::circle_d(od)
        .add(D2::circle_d(od).translate_x(h-od))
        .hull()
        ;
    let inner_shape = D2::circle_d(d)
        .add(D2::circle_d(d).translate_x(h-d- 2.0*w_wall))
        .hull()
        .add(D2::rectangle( (h-d-2.0*w_wall -1.0*d, od) )
            .translate_x(0.25*d)
            )
        ;
    let result = (outer_shape - inner_shape)
        .linear_extrude(w);
    println!("$fn=128;\n{}", &result);
    Ok(())
}

