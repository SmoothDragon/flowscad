use flowscad::*;
use anyhow::Result;




fn main() -> Result<()> {
    // Measured diameters 7.4 to 5.3  over length of 90.
    // This is approximately 1mm of diameter loss per 45mm of length
    let d_per_mm = X(1.0/45.0);
    let l_hold = X(50.);
    let d_hold = X(6.0);  // smallest diameter of hold

    let w_wall = X(2.0);
    let h_center = X(100.);
    let d_center = X(40.);

    let hold = D3::Frustum(l_hold, d_hold + 2*w_wall, d_hold + d_per_mm * l_hold + 2*w_wall)
        .sub(D3::Frustum(l_hold+0.02, d_hold, d_hold + d_per_mm * l_hold).translate_z(-0.01))
        .sub(D3::Cube(3*l_hold).translate_z(-l_hold).rotate_z(45))
        .rotate_x(45)
        .translate_z(-0.707*l_hold/2)
        // .translate_y(0.707*l_hold/2)
        .translate_x( (d_hold+d_center)/2 )
        .iter_rotate( (0,0,36), 10)
        .union()
        .add(D3::Cylinder(h_center, d_center).center())
        ;

    let result = hold;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

