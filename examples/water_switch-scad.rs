// Samsung One Touch Pure Water switch replacement
// This mechanism replaces the button with a level that is pushed forward or backwards to let water

use flowscad::*;

use std::f32::consts::PI;

fn main() {
    let h_pivot_hole = 6.2;
    let w_pivot_hole = 5.1;
    let depth_pivot_hole = 11.6;
    let w_pivot_arm = 3.3;
    let depth_pivot_arm = 7.3;
    let depth_pivot_arm_nub = 2.0;
    let depth_recess_center = 24.3; // 24.5 10mm away
    let d_inner: f32 = 60.;
    let h_bangle: f32 = 30.;
    let h_layer: f32 = 0.2;
    let r_inner_curve: f32 = 0.7 * h_bangle;
    let inner_profile = D2::circle_r(r_inner_curve)
        .and(D2::rectangle((d_inner, h_bangle)).center())
        .translate_x(r_inner_curve + d_inner)
        .rotate_extrude(360);
    let outer_profile = D2::regular_polygon(8, d_inner + h_bangle)
        .linear_extrude(h_bangle)
        .translate_z(-0.5 * h_bangle);
    let result = (inner_profile & outer_profile).minkowski(D3::sphere_r(2.));
    println!("$fn=64;\n{}", result.scad());
}
