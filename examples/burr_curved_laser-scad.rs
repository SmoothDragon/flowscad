use flowscad::*;
use anyhow::Result;


/// Curved design based on simple 3-piece burr puzzle.


fn main() -> Result<()> {
    let r = X(10.);   // Radius of arcs
    let knob = X(1.);  // Additional knob radius
    let extension = X(5.); // How much to extend figure by
    let h = X(2.75);  // Height of 3mm wood
    let gap = X(0.1);
    let annoying = (6.0_f32 - 4.0*1.414).sqrt();

    let piece = D2::circle_r(r).sub(D2::circle_r(r-gap))
            .and(D2::sector(r+gap, 135).rotate(5))
            .add(D2::circle_r(knob).translate( (r-knob, knob) ))
            .translate( (r*0.707-1.414*knob, -r*0.707) )
            .add(D2::circle_r(knob))
            .translate( (r*0.207, r*0.207) )
            .add(D2::rectangle( (r*annoying, 2*knob) ).center().rotate(45)) 
            .add_map(|x| x.rotate(180))
            .offset_radius(extension)
            .offset_radius(-0.5*extension)
            // .minkowski(D2::circle_r(extension))
        // .sub(D2::rectangle( (h+gap, 3*s+gap) ).center())
        // .sub(D2::rectangle( (3*s+gap, 0.5*(2.5*s-h)+gap) ).center().translate_x(1.5*s))
        // .rotate(45)
        ;

    println!("$fn=64;\n{}", &piece);
    Ok(())
}

