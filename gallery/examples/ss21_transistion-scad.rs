use flowscad::*;

use std::f32::consts::PI;

fn main() {
    let epsilon = 0.01;
    let d_top = 11.0;
    let d_base = 23.95;
    let d_hole = 10.42;
    let h_part = 23.3;

    let taper = D3::frustum_d(h_part, d_base, d_top);
    let hole = D3::cylinder_d(h_part + 2. * epsilon, d_hole).translate_z(-epsilon);
    let result = taper - hole;
    println!("$fn=512;\n{}", result.scad());
}
