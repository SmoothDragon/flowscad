use flowscad::*;

fn main() {
    let side = 30.;
    let base = D2::square(side);
    let wing = base.clone() - base.clone().translate( (1., 1.) );
    let xx = (base + wing.clone().mirror( (1, 1) ) + wing.clone().translate( (side, side) ))
        .rotate(-45)
        .scale_x(0.5)
        .offset_radius(5)
        .offset_radius(-2)
        .linear_extrude(10)
        ;
    println!("$fn=256;\n{}", xx.scad());
}
