use flowscad::*;
use anyhow::Result;


/// Design a 3-piece burr puzzle where the elements look like guitars.

fn main() -> Result<()> {
    let h = X(10.);
    let bevel = X(1.);
    let bevel = X(0.);
    let w: X= 3*h;
    let l: X = 3*w;
    let gap = X(0.1);
    let r = X(22.);
    let s = X(10.);
    let gap2 = gap/2;
    let bgap = X(0.1);

    let egg = D2::egg(r)
        .and(D2::square(4*r).translate_y(-2*r))
        .rotate_extrude(360)
        ;

    let piece1 = D3::beveled_cuboid_offset( (1,1,9), (-0.5,-1.5,-4.5), h, bevel, bgap)
        .add(D3::beveled_cuboid_offset( (1,1,9), (-0.5,0.5,-4.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (3,3,3), (-2.5,-1.5,-4.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (1,3,3), (-0.5,-1.5,1.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (3,1,3), (-2.5,0.5,-3.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (3,1,3), (-2.5,-1.5,-3.5), h, bevel, bgap))
        .color(ColorEnum::Red)
        // .and(egg.clone())
        ;

    let piece2 = D3::beveled_cuboid_offset( (1,9,1), (0.5,-4.5,-0.5), h, bevel, bgap)
        .add(D3::beveled_cuboid_offset( (2,9,2), (0.5,-4.5,1.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (2,9,2), (-2.5,-4.5,1.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (2,4,4), (-2.5,-4.5,-0.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (2,4,4), (-2.5,0.5,-0.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (2,4,4), (0.5,-4.5,-0.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (2,4,4), (0.5,0.5,-0.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (3,3,4), (-1.5,-4.5,-0.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (3,3,4), (-1.5,1.5,-0.5), h, bevel, bgap))
        .color(ColorEnum::Cyan)
        // .and(egg.clone())
        ;

    let piece3 = D3::beveled_cuboid_offset( (9,1,1), (-4.5,-0.5,-1.5), h, bevel, bgap)
        .add(D3::beveled_cuboid_offset( (4,1,1), (-4.5,-0.5,0.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (4,1,1), (0.5,-0.5,0.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (3,1,3), (1.5,-0.5,-1.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (3,1,3), (-3.5,-0.5,-1.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (3,5,2), (0.5,-2.5,-2.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (3,1,1), (-1.5,-2.5,-1.5), h, bevel, bgap))
        .add(D3::beveled_cuboid_offset( (3,1,1), (-1.5,1.5,-1.5), h, bevel, bgap))
        .color(ColorEnum::Green)
        // .and(egg.clone())
        // .translate_x(h)
        ;

    let result = piece1
        // +piece2
        +piece3
        ;

    // let result = result & egg;

    println!("$fn=64;\n{}", &result);
    Ok(())
}

