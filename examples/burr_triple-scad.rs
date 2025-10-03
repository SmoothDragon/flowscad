use flowscad::*;
use anyhow::Result;


/// Design to hook around the handle of a Pillar work cubicle to make it easier
/// to access a backpack with a loop strap on top.

fn main() -> Result<()> {
    let s = X(10.);
    let h = X(10.);
    let bevel = X(1.);
    let w = 3*h;
    let l = 3*w;
    let gap = X(0.1);
    let gap2 = gap/2;

    let piece1 = D3::beveled_cube_block( (9,1,1), s, bevel, gap)
        + D3::beveled_cube_block( (9,1,1), s, bevel, gap).translate_y(2*s)
        + D3::beveled_cube_block( (3,3,1), s, bevel, gap)
        + D3::beveled_cube_block( (3,3,1), s, bevel, gap).translate_x(6*s)
        ;
    let piece2 = D3::beveled_cube_block( (9,1,1), s, bevel, gap).translate_y(2*s)
        + D3::beveled_cube_block( (4,1,1), s, bevel, gap)
        + D3::beveled_cube_block( (4,1,1), s, bevel, gap).translate_x(5*s)
        + D3::beveled_cube_block( (3,3,1), s, bevel, gap)
        + D3::beveled_cube_block( (3,3,1), s, bevel, gap).translate_x(6*s)
        ;

    let piece3 = D3::beveled_cube_block( (9,1,1), s, bevel, gap).translate_y(2*s)
        + D3::beveled_cube_block( (4,3,1), s, bevel, gap)
        + D3::beveled_cube_block( (4,1,1), s, bevel, gap).translate_x(5*s)
        + D3::beveled_cube_block( (3,3,1), s, bevel, gap)
        + D3::beveled_cube_block( (3,3,1), s, bevel, gap).translate_x(6*s)
        ;

    // let piece3 = D3::beveled_box( v3(l,h-gap2,h), bevel).translate_y(2*h+gap2)
        // .add(D3::beveled_box( v3(w+h-gap2,w,h), bevel))
        // .add(D3::beveled_box( v3(w+h-gap2,h-gap2,h), bevel).translate_x(w+2*h-gap2))
        // .add(D3::beveled_box( v3(w-gap2,w,h), bevel))
        // .add(D3::beveled_box( v3(w-gap2,w,h), bevel).translate_x(2*w+gap2))
        // ;

    let result = piece1.translate_y(9*s)
        + piece2.translate_y(4*s)
        + piece3
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

