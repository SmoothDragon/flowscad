use flowscad::*;

fn main() {
    let w = 5.;
    let h = 30.;
    let base = D2::rectangle((w,h))
        .center()
        .add_map(|x| x.rotate(90))
        .add_map(|x| x.translate( (h/2.+w,h/2.+w) ))
        .translate( (w/2., -h/2.) )
        .add(D2::rectangle( (2.122*w, 0.707*w) )
            .translate_y(-0.707*w)
            .rotate(45)
        )
        .rotate(-45)
        .scale_y(2.)
        .linear_extrude(10)
        ;
    // let wing = base.clone() - base.clone().translate( (1., 1.) );
    // let xx = (base + wing.clone().mirror( (1, 1) ) + wing.clone().translate( (side, side) ))
        // .rotate(-45)
        // .scale_x(0.5)
        // .offset_radius(5)
        // .offset_radius(-2)
        // .linear_extrude(10)
        // ;
    println!("$fn=256;\n{}", base.scad());
}
