use std::fs;
use flowscad::*;


fn main() {
    let w_edge = 1.6;
    let w_scoop = 50.0;

    let base = D2::circle_d(1.5 * w_scoop)
        .and(D2::circle_d(w_scoop*1.5).translate_y(0.5 * w_scoop))
        .and(D2::rectangle( (w_scoop, 2. * w_scoop) ).center())
        ;

    let bottom = base.clone().linear_extrude(w_edge);

    let shell = base.clone()
        .sub(base.clone().offset_delta(-w_edge))
        .and(D2::rectangle( (w_scoop, 2. * w_scoop) ).translate_x(-w_scoop*0.5))
        .linear_extrude(1.5 * w_scoop)
        .and(D3::sphere_r(1.5 * w_scoop).translate( (0, w_scoop*1.2, 0.*w_scoop*0.3) ))
        ;

    let lip = D3::cuboid( (w_scoop, w_edge, 0.5 * w_scoop) ).translate_x(-0.5 * w_scoop);
       
    println!("$fn=256; {}", lip+shell+bottom);
}


