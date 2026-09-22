use flowscad::*;
use anyhow::Result;


/// Design a desicate holder for the top of my wheat grain jar.

fn main() -> Result<()> {
    let h = X(15.);
    let id = X(68.);
    // let od = X(70.);
    let od = X(71.);
    let w_offset = X(2.);
    let l_offset = X(5.);
    let angle = X(45.);
    let fins = 30;
    let w_hole = X(5.);

    let result = D3::frustum_d(h, od, id)
        .sub(D3::cuboid((od, w_offset, 3*h))
            .translate((0,w_offset/2,-1.5*h))
            .rotate_z(angle)
            .translate_x(od/2-l_offset)
            .iter_rotate((0,0,X(360.)/fins), fins)
            .union()
        )
        .sub(D3::cube(w_hole).center()
            .iter_translate((1.5*w_hole,0,0),7)
            .union()
            .translate_x(-4.5*w_hole)
            .iter_translate((0,1.5*w_hole,0),7)
            .union()
            .translate_y(-4.5*w_hole)
            .and(D3::cylinder_d(2*w_hole, id*0.8).center())
        )
        ;


    println!("$fn=64;\n{}", &result);
    Ok(())
}

