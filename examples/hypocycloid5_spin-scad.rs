use flowscad::*;

use std::f32::consts::PI;
use num_complex::Complex32;


// fn exp1j(theta: f32) -> Complex32 {
    // Complex32::new(theta.cos(), theta.sin())
// }

fn main() {
    let resolution = 250;
    let k = 5;
    let hypo5: Face = hypocycloid(k, resolution)
        // .scaled(5.)
        ;

    // let theta = Array1::linspace(0.0, 2.0 * PI, resolution + 1);
    // let circle = theta.mapv(|t| exp1j(t));
    // let level: Vec<Array1<Complex32>> = (0..theta.len()).step_by(step)
        // .map(|ii| hypo_inner
            // .mapv(|z| z * exp1j(-theta[ii] / k) + circle[ii])
            // .slice(ndarray::s![..-1])
            // .to_owned()
        // )
        // .collect()
        // ;

    let n = 360;
    let layers: Vec<Face> = (0..n)
        .map(|ii| {
            let mut layer = hypo5.clone();
            layer.rotate(Deg((-ii as f32)/(k as f32)));
            layer.translate(Deg(ii as f32).into());
            layer.scale(10.);
            // layer.translate( expi(-2.0*PI / (resolution as f32) * (ii as f32)));
            layer
        })
        .collect();


    let poly = polygon_stack(layers, 0.2);


    println!("$fn=256;\n{}", poly.scad());
}
