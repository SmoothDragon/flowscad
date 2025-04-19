use flowscad::*;
use anyhow::Result;


/// Design a 3-piece burr puzzle where the elements look like guitars.

fn main() -> Result<()> {
    let r = X(40.);
    let s = X(10.);
    let d_big = X(30.);
    let d_small = X(23.);
    let w_neck = d_small/3.5;
    let h = X(5.);
    let bevel = X(1.);
    // let w = 3*h;
    // let l = 3*w;
    let gap = X(0.1);
    // let gap2 = gap/2;

    let egg = D2::egg(r)
        .and(D2::square(4*r).translate_y(-2*r))
        .rotate_extrude(360)
        ;

    let piece1 = egg.clone().and(D3::cube(2*r).center().translate_z(-r-s/2))
        ;

    /*
    let piece3 = D3::beveled_cube_block( (9,1,1), s, bevel, gap).translate_y(2*s)
        + D3::beveled_cube_block( (4,3,1), s, bevel, gap)
        + D3::beveled_cube_block( (4,1,1), s, bevel, gap).translate_x(5*s)
        + D3::beveled_cube_block( (3,3,1), s, bevel, gap)
        + D3::beveled_cube_block( (3,3,1), s, bevel, gap).translate_x(6*s)
        ;
    */

    let result = piece1.translate_y(0)
        // + piece2.translate_x(1.1*d_big)
        // + piece3.translate_x(2.2*d_big)
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

