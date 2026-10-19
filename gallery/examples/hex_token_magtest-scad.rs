/** 
 * Caturledas 20x10x3 mm Rare Earth Neodymium Magnets, 50 Pack
 * 20261001: $13.99
 *
 * The measured magnets were roughly 19x9x2.5mm.
 *
 * This code prints a variety of hole sizes to find the best friction slide fit.
 *
 * Instructions:
 * Find the right width and the first pass.
 * Set the correct width and find the depth on the second pass
 *
 * Resultant code for hole use:
    let h_magnet = X(19.0);
    let w_magnet = X(9.2);
    let d_magnet = X(2.6);
    let hole_mag = D3::cuboid( (w_magnet, d_magnet, h_magnet) );
 **/

use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let w_wall = X(1.2);
    let w_wall_inner = X(0.4);

    let h_magnet = X(19.0);
    let w_magnet = X(9.0);
    let d_magnet = X(2.5);
    let n_magnet = 6;
    let step = X(0.1);

    let holes = (-n_magnet/2..n_magnet/2)
        .map(|ii| {
            let xx = X(ii as f32);
            let shift = xx * step;
            // D3::cuboid( (w_magnet+shift, d_magnet+shift, h_magnet) )  // Initial range
            D3::cuboid( (w_magnet+2*step, d_magnet+shift, h_magnet) )  // Set width
                .translate_y((xx * 2 + (n_magnet + 1))* d_magnet)
                .translate_x(d_magnet)
        })
        .union()
        ;


    let end_result = D3::cuboid((
            2*d_magnet + w_magnet, 
            (2*n_magnet+1)*d_magnet, 
            h_magnet/2
            ))
        - holes
        ;
    println!("$fn=64;\n{}", &end_result);
    Ok(())
}
