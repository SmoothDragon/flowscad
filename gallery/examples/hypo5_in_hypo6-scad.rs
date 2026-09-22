use flowscad::*;

use std::f32::consts::PI;
use num_complex::Complex32;


fn main() {
    let resolution = 300;
    let k = 5;
    let hypo5: Face = hypocycloid(k, resolution);
    let hypo6: Face = hypocycloid(k+1, resolution);

    let n = 1080;
    let h = (n as f32) * 0.2;
    let layers5: Vec<Face> = (0..n)
        .map(|ii| {
            let mut layer = hypo5.clone();
            layer.rotate(Deg((-ii as f32)/(k as f32)));
            layer.translate(Deg(ii as f32).into());
            layer.scale(10.);
            layer
        })
        .collect();

    let layers6: Vec<Face> = (0..n)
        .map(|ii| {
            let mut layer = hypo6.clone();
            layer.scale(15.);
            layer
        })
        .collect();

    let poly5 = polygon_stack(layers5, 0.2);
    // let poly6 = D3::cylinder_r(h/2.0, 100.) - D2::from(&hypo6)
    let poly6 = (D2::circle_d(150.) - D2::from(&hypo6.scaled(11.))).linear_extrude(h+1.0);
    // let poly6 = polygon_stack(layers6, 0.2);
    // let result = (poly6 + poly5).and(D3::frustum_d(h, 150., 1.));
    // let result = (poly5).and(D3::frustum_d(h, 150., 1.));
    
    let result = poly6 + poly5;

    println!("$fn=128;\n{}", result.scad());
}
