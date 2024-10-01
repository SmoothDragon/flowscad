use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let r = X(33.0);
    // let wall = X(1.9);
    let wall = X(2.4);
    let shift = X(1.2);

    let side = D2::circle_r(r)
        .difference(D2::circle_r(r-wall))
        .intersection(D2::square(r))
        .rotate(-45)
        .rotate_extrude(180)
        ;
    let notch = D3::octahedron(1.).translate_z(5)
        .add(D3::octahedron(1.).translate_z(-5))
        .hull()
        // .rotate_z(45)
        .translate( (-0.707*r+wall-0.4, 0.707*r, 0) )
        ;
    let side = D3::sphere_r(r)
        .difference(D3::sphere_r(r-wall))
        .intersection(
            D3::cuboid( (4*r, 4*r, 1.414*r) )
            .center()
            .translate_x(2*r)
            )
        .difference(notch.clone()
             .rotate_y(180)
             .rotate_x(90)
             .minkowski(D3::octahedron(0.2))
             .add_map(|x| x.mirror( (0,0,1) ))
             )
        ;
    let endcap3 = D3::sphere_r(r)
        .difference(D3::sphere_r(r-wall))
        .intersection(
            D3::cube(2*r)
            .center()
            .translate_y(1.707*r)
            )
        .add(notch)
        ;


    let endcap_d2 = D2::circle_r(r)
        .difference(D2::circle_r(r-wall))
        .intersection(D2::square(r))
        .rotate(45)
        .intersection(D2::square(r))
        ;
    let endcap = endcap_d2.clone()
        .rotate_extrude(180)
        .rotate([0,90,180])
        ;
    let endcap2_d2 = D2::circle_r(r)
        .difference(D2::circle_r(r-wall))
        .intersection(
            D2::rectangle( (2*r, 2*r) )
            .translate( (0.707*r-shift, -r) )
            )
        .add(D2::circle_r(r-1.414*shift)
            .difference(D2::circle_r(r-wall))
            .intersection(D2::square(r))
            .rotate(-45)
            // .intersection(D2::square(r))
            )
        .rotate(90)
        .intersection(D2::square(r))
        ;
    let endcap2 = endcap2_d2
        .rotate_extrude(360)
        .rotate([0,-90,180])
        ;
    
    // let mut result = (side + endcap + endcap2)
    let result = side + endcap3.clone() + endcap3.rotate_x(180)
        // .intersection(
            // D3::cuboid( (4*r, 4*r, 1.414*r) )
            // .translate( (-2*r, -2*r, -0.707*r + shift) )
            // )
        ;
    // result += result.rotate( (180, 90, 0) )
        // .translate_y(-1)
        // .difference(D3::cube(100))
        ;
    println!("$fn=256;\n{}", &result);
    Ok(())
}
