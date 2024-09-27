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
    
    let mut result = (side + endcap + endcap2)
        .intersection(
            D3::cuboid( (4*r, 4*r, 1.414*r) )
            .translate( (-2*r, -2*r, -0.707*r + shift) )
            )
        ;
    // result += result.rotate( (180, 90, 0) )
        // .translate_y(-1)
        // .difference(D3::cube(100))
        ;
    println!("$fn=256;\n{}", &result);
    Ok(())
}
