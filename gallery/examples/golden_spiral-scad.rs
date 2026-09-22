use flowscad::*;
use ndarray::{Array1, concatenate, Axis, s};
use num_complex::Complex32 as C32;
use std::f32::consts::PI;

// pub use crate::D2Trait;

fn expi(theta: f32) -> C32 {
    C32::new(theta.cos(), theta.sin())
}
pub fn hypocycloid(k: usize, edges: usize) -> Face {
    let theta = Array1::linspace(0.0, 2.0 * PI, edges+1).slice(ndarray::s![..-1]).to_owned();
    let k: f32 = (k-1) as f32;
    Face(theta.map(|rads| k * expi(*rads) + expi(k * *rads).conj())
        // .slice(ndarray::s![..-1])
        // .to_owned()
    )
}

/// Generate points for a golden logarithmic spiral:
/// r(θ) = a * exp(b * θ)
/// b chosen so that r(θ + π/2) = φ * r(θ)
fn generate_golden_spiral_points(
    turns: f32,
    samples_per_turn: usize,
    a: f32,
) -> Vec<C32> {
    let phi = (1.0 + 5f32.sqrt()) / 2.0;
    let b = 4.0 * phi.ln() / PI;
    let theta_end = turns * 2.0 * PI;
    let total_samples = (turns * samples_per_turn as f32).round() as usize;

    let mut pts = Vec::with_capacity(total_samples + 1);
    for i in 0..=total_samples {
        let t = i as f32 / total_samples as f32;
        let theta = t * theta_end;
        let r = 2.0 * a * (b * theta).exp();
        let x = r * theta.cos();
        let y = r * theta.sin();
        pts.push(C32::new(x, y));
    }
    pts
}

fn first() { 
    let pts = generate_golden_spiral_points(2.0, 128, 0.05);
    let path = Path::new(pts, 0.8).d2();
    let result = path
        // .scale(1.5)
        .iter_rotate_equal(5)
        .union()
        .linear_extrude_extra(150, -300, 500)
        .intersection(D3::frustum_d(150., 100., 1.0))
        ;
    println!("$fn=128;\n{}", result.scad());
}

fn second() {
    let turns = 2.0;
    let samples_per_turn = 125;  // 250 and 500
    let pts = generate_golden_spiral_points(turns, samples_per_turn, 1.0);
    let h = 100.0;
    let h_level = 0.2;
    let levels = (h / h_level) as usize;
    let result = (0..levels)
        .map(|ii| Path::new((&pts[0..(levels-ii)/2]).to_vec(), 0.8)
            .d2()
            .iter_rotate_equal(6)
            .union()
            .linear_extrude(h_level)
            .translate_z(h_level * (ii as f32))
        )
        .union()
        ;
    println!("$fn=128;\n{}", result.scad());
}

fn main() {
    first();
}
/*

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
*/
