use flowscad::*;
use anyhow::Result;


/// Design a desicate holder for the top of my wheat grain jar.

fn main() -> Result<()> {
    let s = X(10.);  // Corner square side length
    let w = X(78.);  // Inside square side length
    let h = X(10.);  // Height of square ring

    let angle = X(25.);
    let bevel = X(1.);
    let gap = X(0.1);

    let plate_xyz: XYZ = (w,w,h).into();
    // let plate = D3::beveled_box(plate_xyz, bevel).translate(-plate_xyz/2);
    let half_plate = D3::beveled_box((s,w,h), bevel).translate((w/2,-w/2,-s/2))
        .add(D3::beveled_box((s,s,h), bevel).translate((w/2,w/2,-s/2)))
        .iter_rotate((0,0,90), 2)
        .union()
        ;
    let plate = half_plate.clone().add_map(|x| x.rotate_z(180));
    let half_plate = half_plate
        .sub(D3::cuboid((s/2,s,s/2)).center().translate((w/2+s/2,-w/2,0)))
        .sub(D3::cuboid((s/2,s,s/2)).center().translate((-w/2-s/2,w/2,0)))
        .rotate_z(55)
        .add_map(|x| x.rotate_z(180).translate((w/2,0.64*w,0)))
        .translate((0.3*(w+s),0.83*(w+s),0))
        ;
    let join = D3::beveled_box((s/2-gap,s-gap,s/2-gap), 0.2).translate_z(-s/2)
        + D3::beveled_box((s/2-2*gap,s-4*gap,s/2-2*gap), 0.2)
            .translate((s, 0, -s/2))
        + D3::beveled_box((s/2-3*gap,s-4*gap,s/2-3*gap), 0.2)
            .translate((2*s, 0, -s/2))
        + D3::beveled_box((s/2-4*gap,s-4*gap,s/2-4*gap), 0.2)
            .translate((3*s, 0, -s/2))
        ;
    let result = plate.clone()
        + plate.translate_x(w+2*s+2)
        + half_plate.clone()
        + join.clone()
        + join.translate_y(1.5*s)
        ;
    /*
    let half = plate.clone().and(D3::cube(4*w).center().translate_x(-2*w));

    let notch_out = D2::polygon(vec![v2(0,-2), v2(0,2), v2(3,2.5), v2(3,-2.5)])
        .linear_extrude(h)
        ;
    let notch_in = D2::polygon(vec![v2(0,-2), v2(0,2), v2(3,2.5), v2(3,-2.5)])
        .offset_radius(0.05)
        .mirror((1,0))
        .linear_extrude(2*h)
        .translate_z(-0.5*h)
        ;
    let result = half
        -notch_in.clone().translate_y(-w/2+bar/2)
        +notch_out.clone().translate_y(w/2-bar/2)
        ;
    let result = result.clone() + result.rotate_z(180.).translate_x(10.);
    */

    println!("$fn=64;\n{}", &result);
    Ok(())
}

