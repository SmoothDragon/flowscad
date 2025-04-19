use flowscad::*;
use anyhow::Result;


/// Design a 3-piece burr puzzle where the elements look like guitars.

fn main() -> Result<()> {
    let d_big = X(30.);
    let d_small = X(23.);
    let w_neck = d_small/3.5;
    let h = X(5.);
    let bevel = X(1.);
    // let w = 3*h;
    // let l = 3*w;
    let gap = X(0.1);
    // let gap2 = gap/2;

    let piece1 = D3::chamfer_cylinder_d(h, d_big, bevel)
        + D3::chamfer_cylinder_d(h, d_small, bevel).translate_y(0.6*d_big)
        + D3::beveled_box( v3(w_neck,2*d_big,h), bevel).translate_x(-w_neck/2)
        + D3::beveled_box( v3(2*w_neck,3*bevel,h), bevel)
            .translate( (-w_neck, 2*d_big-4*bevel, 0) )
        + D3::beveled_box( v3(2*w_neck,3*bevel,h), bevel)
            .translate( (-w_neck, 2*d_big-6*bevel, 0) )
        + D3::beveled_box( v3(2*w_neck,3*bevel,h), bevel)
            .translate( (-w_neck, 2*d_big-8*bevel, 0) )
        - D3::beveled_box( v3(h+2*gap, d_big+2*gap, 10*h), bevel )
            .translate( (-h/2-gap, -d_big/2-gap+d_big*0.19, -5*h) )
        - D3::cylinder_d(10*h, 0.4*d_small).center().translate_y(d_big*0.55)
        - D3::beveled_box( v3(2.4*h, 2.1*bevel, 10*h), bevel )
            .translate( (-1.2*h, -bevel-d_big*0.1, -5*h) )
        ;

    let piece2 = piece1.clone()
        // .translate_z(-d_big*0.19+h/2)
        - D3::cuboid( v3(d_big, h+2*gap, 10*h) ).translate((0, 0.5*h,-5*h))
        ;

    let piece3 = (piece1.clone()
        // + D3::cuboid( v3(d_big/2, 2*h, h) ).translate((-d_big/4,-2*h,0))
        // - D3::beveled_box( v3(2.4*h, 2.1*bevel, 10*h), bevel )
            // .translate( (-1.2*h, -bevel-d_big*0.1, -5*h) )
        )
        - D3::cuboid( v3(d_big, h+2*gap, 10*h) ).translate((0, 0.5*h,-5*h)) 
        - D3::cuboid( v3(d_big, h+2*gap, 10*h) ).translate((-d_big-1.5*h, 0.5*h,-5*h)) 
        // .translate_z(-d_big*0.19+h/2)
        // - D3::cuboid( v3(d_big, h, 10*h) ).translate_z(-5*h)
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
        + piece2.translate_x(1.1*d_big)
        + piece3.translate_x(2.2*d_big)
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

