use flowscad::*;
use anyhow::Result;


fn circle_arc_path(radius: f32, count: u32) -> D2 {
    let n = count as f32;
    let circles = D2::circle_r(radius)
        .iter_translate([2.*radius, 0.], count)
        .union()
        ;
    circles.clone()
        .add(D2::rectangle([(n-1.)*2.*radius, 3.0_f32.sqrt()*radius])
            .translate_y(-3.0_f32.sqrt()/2.*radius)
            )
        .sub(circles.clone()
            .translate([radius,3.0_f32.sqrt()*radius])
            )
        .sub(circles.clone()
            .translate([radius, -3.0_f32.sqrt()*radius])
            )
}

fn circle_arc_face(side: f32, ridges: u32) -> D3 { 
    let n = ridges as f32;
    let ridge_radius = side/n/2.;
    circle_arc_path(ridge_radius, ridges)
        .translate_x(ridge_radius)
        .add(D2::rectangle([side, 2.*side]).translate_y(-2.*side))
        .translate_x(ridge_radius)
        .linear_extrude(side)
}

fn circle_arc_cube(side: f32, ridges: u32) -> D3 { 
    let face = circle_arc_face(side, ridges)
        .translate([-side/2., side/2., -side/2.])
        .rotate(v3(45, -90.+2.0_f64.powf(0.5).atan()*180.0/PI, 0))
        .iter_rotate([0.,0.,120.], 3)
        .intersection()
        ;
    face
}

fn main() -> Result<()> {
    let result = circle_arc_path(10., 5);
    let result = circle_arc_cube(10., 5);

    println!("$fn=128;\n{}", &result);
    Ok(())
}

