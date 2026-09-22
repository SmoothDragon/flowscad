use std::fs;
use flowscad::*;


fn main() {
    let l_rod: f32 = 50.;
    let h_rod: f32 = 5.;
    let phi = (1. + 5_f32.sqrt()) / 2.;
    let scale: f32 = l_rod / phi;
    let rod_shift = l_rod / phi / 2.;

    let rod = D3::beveled_box( (l_rod, l_rod, h_rod), 1.)
        .translate( (-l_rod/2., -h_rod/2., -h_rod/2.) )
        .translate_y(rod_shift)
        .add_map(|x| x.rotate_x(180))
        .add_map(|x| x.clone().rotate( (90, 90, 0)) + x.clone().rotate( (90, 0, 90)))
        .and(D3::sphere_r((phi+2.).sqrt() * rod_shift))
        ;
    

    println!("{}", rod);
}


