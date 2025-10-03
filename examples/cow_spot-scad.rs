use flowscad::*;

use rand::{thread_rng, Rng};
use rand::distributions::Open01;
use rand_distr::{Normal, Distribution};

// mean 2, standard deviation 3
// let v = normal.sample(&mut rand::rng());
// println!("{} is from a N(2, 9) distribution", v)

fn main() {
    let normal = Normal::new(0.0, 1.0).unwrap();
    let ri = 0.4;
    let r = 1.;
    let n = 10;
    let spot = (0..n) 
        .map(|_| (thread_rng().sample(Open01), thread_rng().sample(Open01), thread_rng().sample(Open01)) )
        .map(|(rr, x,y): (f32, f32, f32)| D2::circle_r(1.+0.5*rr).translate_x(x).rotate(360.*y))
        .union()
        .offset_radius(r)
        .offset_radius(-r)
        .scale(10.)
        ;

    println!("$fn=256;\n{}", spot);
}

