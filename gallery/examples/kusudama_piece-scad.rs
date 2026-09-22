use flowscad::*;

use std::f32::consts::PI;
use num_complex::Complex32;



fn main() {
    let s: f32 = 40.; 
    let alpha = 2.0_f32.powf(-0.5).atan() * 180. / PI;
    let volume = (s * s * s) / 6.;  // One sixth of cube volume
    let base = 3.0_f32.sqrt() * s * s;
    let height = 6.0 * volume / base;
    let gap: f32 = 0.4;
    let drop: f32 = -1.0;
    let slot: f32 = 2.0;

    let top = D3::cube(s)
        .center()
        .rotate_x(45)
        .rotate_y(alpha)
        .translate_z(-0.5 * 3.0_f32.sqrt() * s + height)
        // .rotate_z(60.)
        ;

    let shape = top.clone()
        .and(D3::cube(2.*s).center().translate_z(s))
        .sub(top.clone().translate_z(drop))
        ;

    // shape = shape.clone() - shape.clone().translate_z(-0.5);

    let fins = D2::rectangle( (0.55 * 2.0_f32.sqrt() * s, 0.25 * 2.0_f32.sqrt() * s - gap) )
        .iter_rotate_equal(3)
        .union()
        .linear_extrude(0.4)
        ;

    let fins = D2::circle_d(0.5 * s)
        .translate_x(-0.4 * s)
        .iter_rotate_equal(3)
        .union()
        .sub(D2::rectangle( (slot, s) )
            .translate( (-0.4 * s - slot, -slot) )
            .iter_rotate_equal(3)
            .union()
        )
        .linear_extrude(0.4)
        .sub(top.clone().translate_z(drop))
        ;

    let shape = shape.clone()
        .add(fins)
        // .sub(shape.clone())
        ;

    println!("$fn=256;\n{}", shape.scad());
}

/*
def makeHalfCubeFromSierpinskiCube(size, sierpinskiCube):
    # Rotate the cube to get one vertex straight down
    theta = degrees(atan(1/sqrt(2)))  # second rotation to get corner in place
    rotatedCube = rotate([45, theta, 0])(sierpinskiCube)  # vertex down
    # Make cube large enough to "be" the top half plane
    topHalfPlane = translate([0, 0, size])(cube(2*size, center=True))
    halfCube = intersection()(rotatedCube, topHalfPlane)
    return halfCube


*/
