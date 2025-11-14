use std::f32::consts::PI;

use anyhow::Result;

use flowscad::*;

pub fn koch_snowflake(radius: X, iterations: usize) -> D2 {
    let x_radius = radius * 3.0_f32.sqrt() / 2;
    let y_radius = radius / 2;
    let mut result = D2::regular_polygon(3, radius).add_map(|x| x.rotate(180));  // Start with hexagram
    for ii in (0..iterations) {
        result = result.clone()
            .scale(1.0/3.0)
            .translate_x(radius * 2 / 3)
            .iter_rotate_equal(6)
            .union()
            .add(result)
            ;
    }
    result
}

fn shift_to_square_edges(theta: f32) -> XY {
    let mut t = theta;
    while t > 120. {
        t -= 120.;
    }
    while t < 0. {
        t += 120.;
    }
    match t {
        t if t <= 30. => XY(1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t).0, -1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t-120.).1),
        t if t <= 60. => XY(-1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t+120.).0, -1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t-120.).1),
        t if t <= 90. => XY(-1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t+120.).0, 1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t).1),
        t if t <= 120. => XY(1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t-120.).0, 1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t).1),
        _ => XY(0.,0.),
    }
}

pub fn releaux<T: Into<X>>(radius: T) -> D2 {
    let r: X = radius.into();
    D2::circle_r(r)
        .and(D2::circle_r(r).translate( XY(3.0_f32.sqrt()/2., 0.5)*r ))
        .and(D2::circle_r(r).translate( XY(3.0_f32.sqrt()/2., -0.5)*r ))
        .translate_x( -3.0_f32.powf(-0.5)*r )
        .rotate(180.0)
}

fn main() -> Result<()> {
    let r = X(20.);
    let h_layer = X(0.2);
    let w_nozzle = X(0.4);
    let gap = X(0.4);
    let h: X = X(100.);
    let layers: i32 = (h / h_layer).trunc();

    let shape = releaux(r);

    let result = (0..layers)
        .map(|ii| { 
            let theta = ii as f32;
            shape.clone()
            .rotate(theta)
            .translate(shift_to_square_edges(theta)*r/2.)
            .translate_x(r/4)
            .rotate(theta)
            .linear_extrude(h_layer)
            .translate_z(h_layer * theta)
        })
        .union()
        ;
    // let result = (0..layers)
        // .map(|ii| eye.clone()
            // .rotate(ii * angle_inner)
            // .translate_y(0.5*r_eye)
            // .rotate(ii * angle_outer)
            // .linear_extrude(h_layer)
            // .translate_z(ii * h_layer)
        // )
        // .union()
        // ;
    let outer = D3::cylinder_r(h-gap, r) -
        (0..layers)
       .map(|ii| { 
            let theta = ii as f32;
            D2::square(r+gap).center()
            // .rotate(theta)
            .translate_x(r/4)
            .rotate(theta)
            .linear_extrude(h_layer)
            .translate_z(h_layer * theta)
        })
        .union()
        ;

    println!("$fn=128;\n{}", result+outer.translate_x(3*r));
    Ok(())
}
/*
def kochSnowflake(diameter=100, iterations=3):
    rad = diameter/2
    xrad = rad*math.sqrt(3)/2
    yrad = rad/2
    # Make an equilateral triangle with circumradius=rad
    koch = [polygon([(0,rad), (-xrad, -yrad), (xrad, -yrad)])]
    # Rotate triangle and union to make hexagram
    hexagram = union()(koch[0], rotate(60)(koch[0]))
    koch.append(hexagram)
    if iterations < 2:
        return koch[iterations]
    for level in range(1, iterations):  # indicates level we are building from
        shape = koch[level]
        for i in range(6):
            shape = union()(
                rotate(60)(shape),
                translate([0,rad*2/3,0])(scale(1/3)(koch[level])),
                )
        koch.append(shape)
    return koch[-1]

def kochmasTree(
        diameter=100,       # Diameter of tree at widest point
        height=150,         # Total height of tree
        top_twist=180,      # Rotation in top part of tree
        base_diameter=50,   # Diameter of base Koch snowflake
        base_height=25,     # Height of tree base
        base_twist=0,       # Twist of tree base
        top_slices=100,     # Slices of top of tree extrusion
        base_slices=2,      # Slices of base extrusion
        koch_iterations=3,  # iterations of Koch Snowflake
        ):
    snowflake = kochSnowflake(diameter=diameter, iterations=koch_iterations)
    base_snowflake = kochSnowflake(diameter=base_diameter, iterations=koch_iterations)
    trunk = linear_extrude(
                height=base_height,
                scale=diameter/base_diameter,
                twist=base_twist,
                slices=base_slices,
                )(base_snowflake)
    top = linear_extrude(
                height=height-base_height, 
                scale=0,  # Ends in a point
                slices=top_slices,
                twist=top_twist,
                )(snowflake)
    top = translate([0,0,base_height])(top)  # Put top over base

    tree = union()(top, trunk)
    return tree

if __name__ == '__main__':
    fractalChrismasTree = kochmasTree()
    print(scad_render(fractalChrismasTree))
*/
