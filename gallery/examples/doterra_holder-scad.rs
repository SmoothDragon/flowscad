use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let show_bottles = false;
    let d_bottle = X(29.);
    let h_bottle = X(67.);
    let gap = X(0.6);
    let d_hole = 2*gap + d_bottle;
    let w_wall = X(4.);
    let h_wall = h_bottle/3;
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

    let base = D2::chamfered_circle_r(w_wall/2.)
        .add_map(|x| x.translate_y(h_wall - w_wall))
        .hull()
        .translate( (d_hole/2. + w_wall/2., w_wall/2.) )
        .add(D2::rectangle( (d_hole/2.-w_wall/2., w_wall/2.) )
             .translate_x(w_wall))
        .rotate_extrude(360)
        .translate_x(d_hole+w_wall)
        .iter_rotate([0,0,60],6)
        .union()
        .add(handle)
        ;

    let bottles = D3::cylinder_d(h_bottle, d_bottle)
        .translate( (d_hole + w_wall, 0, w_wall + gap) )
        ;

    let result = if show_bottles {
        base + bottles
    } else {
        base
    };

    println!("$fn=128;\n{}", &result);
    Ok(())
}

