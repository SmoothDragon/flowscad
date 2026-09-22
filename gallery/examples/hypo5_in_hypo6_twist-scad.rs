use flowscad::*;

use std::f32::consts::PI;
use num_complex::Complex32;

/// Produces a sequence of k-hypocycloids rotating around inside a (k+1)- hypocycloid
pub fn rotating_hypocycloid<T: Into<Deg>>(k: usize, edges: usize, steps: usize, angle_per_step: T) -> Vec<Face> {
    let shape: Face = hypocycloid(k, edges);
    let angle: Deg = angle_per_step.into();
    let mut layers: Vec<Face> = Vec::new();
    for ii in 0..steps {
        let mut layer = shape.clone();
        layer.rotate(Deg(-(ii as f32)/(k as f32)));
        layer.translate(Deg(ii as f32).into());
        layers.push(layer);
    }
    layers
}

pub fn bound_radius(radius: f32, face: Face) -> Face {
    Face(face.0.map(|xy| { 
        let norm = xy.norm();
        if norm <= radius {
            *xy
        } else {
            (radius / norm) * xy
        }
    }))
}


fn main() {
    let edges = 300;
    let k = 5;
    let hypo5: Face = hypocycloid(k, edges);
    let hypo6: Face = hypocycloid(k+1, edges);

    let n = 900;
    let h = (n as f32) * 0.2;
    let layers5: Vec<Face> = (0..n)
        .map(|ii| {
            let mut layer = hypo5.clone();
            layer.rotate(Deg(-(ii as f32)/(k as f32)));
            layer.translate(Deg(ii as f32).into());
            layer.scale(5.);
            layer.translate( C32::new(10.,0.) );
            layer.rotate(Deg(0.5 * (ii as f32)));
            bound_radius(50. * (1. - (ii as f32)/(n as f32)), layer)
        })
        .collect();

    /*
    let layers5: Vec<Face> = rotating_hypocycloid(k, edges, n, Deg(1.)).iter()
        .enumerate()
        .map(|(ii, x)| x
            .scaled(5.)
            .translated( C32::new(10.,0.) )
            .rotated(Deg((ii as f32)/10.))
        )
        .collect()
        ;
    */

    let layers6: Vec<Face> = (0..n)
        .map(|ii| {
            let mut layer = hypo6.clone();
            layer.scale(10.);
            layer.translate( C32::new(10.,0.) );
            layer.rotate(Deg((ii as f32)/10.));
            layer
        })
        .collect();

    let poly6 = (0..n).map(|ii| {
        let mut layer = hypo6.clone();
        layer.scale(5.1);
        layer.translate( C32::new(10.,0.) );
        layer.rotate(Deg((ii as f32)/10.));
        (D2::circle_r(50. * (1. - (ii as f32)/(n as f32))) - (&layer).into())
        // (D2::circle_d(150.) - (&layer).into())
            .linear_extrude(0.2)
            .translate_z((ii as f32) * 0.2)
        })
        .union()
        .and(D3::frustum_d(h, 100., 5.));

    let poly5 = polygon_stack(layers5, 0.2)
        // .and(D3::frustum_d(h, 100., 1.))
        ;
    // let poly6 = D3::cylinder_r(h/2.0, 100.) - D2::from(&hypo6)
    // let poly6 = (D2::circle_d(150.) - D2::from(&hypo6.scaled(11.))).linear_extrude(h+1.0);
    // let poly6 = D3::cylinder_r(h/2.0, 100.).translate_z(50.) - polygon_stack(layers6, 0.2);
    // let poly6 = polygon_stack(layers6, 0.2);
    // let result = (poly6 + poly5).and(D3::frustum_d(h, 150., 1.));
    // let result = (poly5).and(D3::frustum_d(h, 150., 1.));
    
    let result = poly6 + poly5;
    // let result = poly5;
    // let result = poly6;

    println!("$fn=128;\n{}", result.scad());
}
