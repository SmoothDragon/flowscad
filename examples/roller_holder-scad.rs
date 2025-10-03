use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let d_bottle = X(20.);
    let h_bottle = X(86.);
    let gap = X(0.6);
    let d_hole = 2*gap + d_bottle;
    let d_bottom_hole = 0.5*d_hole;
    let w_wall = X(2.);
    let h_wall = 0.4 * h_bottle;
    let r_handle_shift = 0.75*d_bottle+w_wall;
    let h_handle = h_bottle*0.7;

    let grip = D2::circle_d(w_wall)
        .translate_x(d_bottle/2. + w_wall/2.)
        .rotate_extrude(360)
        .rotate_x(90)
        .translate_z(h_handle+d_bottle/2.+w_wall/2.)
        .rotate_z(30)
        ;

    let handle = D3::sphere_d(w_wall)
        .translate_y(r_handle_shift)
        .add(D3::sphere_d(w_wall)
             .translate_z(h_handle))
        .hull()
        .iter_rotate([0,0,60], 6)
        .union()
        .add(grip)
        .translate_z(h_wall-w_wall)
        ;

    // let handle = D2::circle_d(d_bottl

    let r_handle = 0.35*d_bottle;
    let h_handle = 0.7*h_bottle;
    let center = D3::cylinder_d(h_wall, 2.*d_bottle)
        .add(D3::frustum_r(0.7*h_bottle, d_bottle, 0.9*r_handle)
             .translate_z(h_wall)
             )
        .sub(D3::cylinder_d(3.*h_bottle, d_hole)
             .translate_x(d_hole+w_wall)
             .iter_rotate([0,0,60], 6)
             .union()
             .translate_z(-1)
             )
        .add(D2::circle_r(r_handle)
             .translate_x(3.*r_handle)
             .rotate_extrude(360)
             .rotate_x(90)
             .translate_z(h_handle+h_wall+2.8*r_handle)
             .rotate_z(30)
             )
        ;

    let base = D2::chamfered_circle_r(w_wall/2.)
        .add_map(|x| x.translate_y(h_wall - w_wall + 1.))
        .hull()
        .translate( (d_hole/2. + w_wall/2., w_wall/2.) )
        .add(D2::rectangle( (d_hole/2.-d_bottom_hole/2., w_wall/2.) )
             .translate_x(d_bottom_hole/2. + w_wall/2.))
        .rotate_extrude(360)
        .translate_x(d_hole+w_wall)
        .iter_rotate([0,0,60],6)
        .union()
        // .add(handle)
        .add(center)
        .sub(D3::cylinder_d(h_bottle, d_hole)
             .translate_x(-0.5*d_hole)
             .rotate_y(12)
             .translate_x(1.25*d_hole)
             .translate_z(w_wall/2.)
             .iter_rotate([0,0,60], 6)
             .union()
             )
        ;

    let bottles = D3::cylinder_d(h_bottle, d_bottle)
        .translate( (d_hole + w_wall, 0, w_wall + gap) )
        ;

    // let show_bottles = true;
    let show_bottles = false;
    let result = if show_bottles {
        base + bottles
    } else {
        base
    };

    println!("$fn=128;\n{}", &result);
    Ok(())
}

