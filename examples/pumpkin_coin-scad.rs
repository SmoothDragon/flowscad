use flowscad::*;
use anyhow::Result;


/// Special coin designed for pumpkin jam competition

// TODO: add this as a function
fn chamfer_extrude(shape: D2) -> D3 {
    let layers = 10;
    let h_layer = X(0.1);
    let h_shape = X(10.);
    let result = (0..layers).
        map(|x| shape.clone()
            .offset_chamfer(-h_layer*x*0.7)
            .linear_extrude(h_layer)
            .translate_z(h_layer*x)
            )
        .union()
        .translate_z(h_shape/2-h_layer*layers)
        .add(shape.linear_extrude(h_shape/2-h_layer*layers))
        .add_map(|x| x.mirror((0,0,1)))
        ;
    result
}
    
fn chamfer_extrude_out(shape: D2) -> D3 {
    let h_layer = X(0.04);  // Hal of minimum layer height of 0.08mm
    let layers = 50;  // 50 layers will add up to a 2mm height
    let result = shape.clone() .linear_extrude(h_layer*12)
        + (12..layers).map(|x| shape.clone()
            .offset_chamfer(h_layer*(x-12)*0.5)
            .linear_extrude(h_layer)
            .translate_z(-h_layer*x)
            )
            .union()
        ;
    result
}
    

fn main() -> Result<()> {
    let r = X(25.);
    let l_edge = r * 2.0_f32.powf(-1.5);
    let s = X(10.);
    let h = X(10.);
    let h2 = X(2.);
    let bevel = X(1.);
    let w = 3*h;
    let l = 3*w;
    let gap = X(0.1);
    let gap2 = gap/2;

    let koch = D2::koch_snowflake(r, 2)
            .scale(1./3.)
            .add_map(|x| x
            .translate_y(2.*r/3.)
            .iter_rotate_equal(6)
            .union()
            )
            // .add(D2::circle_d(2.*r+5.)-D2::circle_d(2.*r+1.))
            .minkowski(D2::square(0.01))
            // .linear_extrude(h2)
        ;
    let koch = chamfer_extrude_out(koch);
    let side2 = (D2::circle_d(r+1)
            .translate_x(r/2.)
            .map(|x| x.clone() & x.clone().rotate(60))
            -D2::circle_d(r-1).translate_x(r/2)
            )
        .add_map(|x| x.mirror([1,0]))
        // .translate_x(r/2.)
        .iter_rotate_equal(6)
        .union()
        // .add(D2::circle_d(2.*r+5.)-D2::circle_d(2.*r+1.))
        .linear_extrude(h2)
        ;
    let piece = (D2::circle_d(3*s).translate( (-2.75*s, 3*s) )
        + ((D2::circle_r(4*s)-D2::circle_r(1.5*s)) & D2::square(4*s))
            .translate( (-2.75*s, 0.5*s) )
        + D2::rectangle( (2.5*s, s) ).center()
        )
        .add_map(|x| x.rotate(180))
        - D2::rectangle( (s+gap, 3*s+gap) ).center()
        ;
    let piece1 = chamfer_extrude(piece.clone());
    let piece2 = chamfer_extrude(piece.clone()
        - D2::rectangle( (3*s+gap, s+gap) ).center().translate_x(s)
    );


    let result = piece1
        + piece2.clone().translate_x(5*s)
        + piece2.translate_x(-5*s)
        ;
    // let result = chamfer_extrude_out(koch)
    let result = koch.clone().translate_z(h2) 
        + koch.hull()
        + side2.translate_z(-h2)
        // + D2::circle_d(2.*r+5.)-D2::circle_d(2.*r+1.))
        ;

    println!("$fn=64;\n{}", &result);
    Ok(())
}

