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

fn main() -> Result<()> {
    let r = X(50.);
    let h_layer = X(0.2);
    let w_nozzle = X(0.4);
    let gap = X(0.8);
    let overhang_factor = X(0.5);  // What percentage of next layer overhangs 
    let angle_increment = overhang_factor / r * w_nozzle * 180 / PI;
    let second_angle = angle_increment/2;
    let h_tree: X = 3 * r;
    let layers: i32 = (h_tree / h_layer).trunc();

    let koch = koch_snowflake(r, 1);

    let cone = (0..layers)
        .map(|ii| koch.clone()
            .rotate(ii * angle_increment)
            .scale(X(1.0) * (layers-ii)/layers)
            .linear_extrude(h_layer)
            .translate_z(ii * h_layer)
        )
        .union()
        ;

    let inner = (0..layers)
        .map(|ii| koch.clone()
            .scale(0.5)
            .offset_radius(-1)
            .offset_radius(2)
            .rotate(ii * angle_increment)
            // .scale(X(1.0) * (layers-ii)/layers)
            .linear_extrude(h_layer)
            .translate_z(ii * h_layer)
        )
        .union()
        ;


    let result = koch.clone()
        .sub(koch
            .scale(1.0/2.0)
            // .rotate(30)
        )
        .linear_extrude(10);
    let result = cone - inner;
    println!("$fn=16;\n{}", &result);
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
