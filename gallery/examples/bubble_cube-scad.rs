use flowscad::*;

use std::f32::consts::PI;

fn main() {
    let theta = (-1.0_f32/3.).acos()*180.0/PI;
    let phi: f32 = 90. - (theta/2.);
    let l_edge = 50.;
    let dot = D3::truncated_octahedron(0.4);
    let center = dot.clone()
        .translate(v3(l_edge * 0.707 * (1. + phi.cos()), 0, 0))
        .iter_rotate(v3(0,0,90), 4)
        .hull()
        ;
    let wing = dot.clone()
        .translate(v3(l_edge * 0.707 * (1. + phi.cos()), 0, 0))
        .add(dot.clone().translate(v3(l_edge * 0.707, 0., l_edge / 2.)))
        .add(dot.clone().translate(v3(l_edge * 0.707, 0., -l_edge / 2.)))
        .hull()
        .iter_rotate(v3(0,0,90), 4)
        .union()
        ;
    let plate = dot.clone()
        .translate(v3(l_edge * 0.707 * (1. + phi.cos()), 0, 0))
        .add(dot.clone().translate(v3(l_edge * 0.707, 0., l_edge / 2.)))
        .add_map(move |x| x.rotate(v3(0,0,90)))
        .hull()
        .iter_rotate(v3(0,0,90), 4)
        .union()
        .add_map(move |x| x.rotate(v3(180,0,0)))
        ;
    let shape = (wing+plate+center)
        .rotate(v3(90,45,0))
        ;
    
    println!("{}", &shape);
}

