use flowscad::*;
use anyhow::Result;


/// Design a desicate holder for the top of my wheat grain jar.

fn main() -> Result<()> {
    let h = X(5.);
    let w = X(100.);
    let angle = X(35.);
    let bevel = X(1.);

    let plate_xyz: XYZ = (w,w,h).into();
    let plate = D3::beveled_box(plate_xyz, bevel).translate(-plate_xyz/2);
    let result = plate.clone().rotate_x(angle)
        .add(plate.clone().rotate_y(90+angle))
        .add(plate.clone().rotate((0,90,90)).rotate_z(angle))
        // .add_map(|x| x.clone().rotate_y(90) + x.rotate( (0,90,90) ))
        ;


    println!("$fn=64;\n{}", &result);
    Ok(())
}

