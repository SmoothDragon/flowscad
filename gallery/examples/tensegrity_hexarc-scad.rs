use std::fs;
use flowscad::*;


fn main() {
    let l_rod: f32 = 50.;
    let h_rod: f32 = 5.;
    let phi = (1. + 5_f32.sqrt()) / 2.;
    let scale: f32 = l_rod / phi;
    let rod_shift = l_rod / phi / 2.;
    let angle = phi.atan() * 180. / PI;
    let r_sphere = (phi+2.).sqrt() * rod_shift;

    let rod = (D2::hexagon(h_rod) - D2::hexagon(3.))
        .translate_x(r_sphere)
        .rotate_extrude(2.*angle)
        .rotate_z((180. - 2.*angle)/2.)
        .add_map(|x| x.rotate_x(180))
        .add_map(|x| x.clone().rotate( (90, 90, 0)) + x.clone().rotate( (90, 0, 90)))
        ;
    

    println!("$fn=128;\n{}", rod);
}


