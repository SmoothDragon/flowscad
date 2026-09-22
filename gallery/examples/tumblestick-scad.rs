use flowscad::*;
use anyhow::Result;
use ::bitperm::*;

fn main() -> Result<()> {
    let w_stick = 25.0;
    let h_stick = 0.125 * w_stick;
    let l_stick = 18.0 * w_stick;
    let d_arc = 2.5 * w_stick;
    
    let base = D2::rectangle( [ w_stick, h_stick ] )
        .translate_x(-0.5 * w_stick)
        .and(
            D2::circle_d(d_arc)
            .translate_y(0.5 * d_arc)
        )
        ;
    let result = base
        .linear_extrude(0.5 * l_stick)
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

