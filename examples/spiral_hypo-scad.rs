use ndarray::{Array1, concatenate, Axis, s};
use num_complex::Complex32;
use std::f32::consts::PI;

use flowscad::*;

fn main() {
    let k = 5.0_f32;
    let resolution = 256;

    // --- Generate theta ---
    let theta = Array1::linspace(0.0, 2.0 * PI, resolution + 1);

    // --- Circle ---
    let circle = theta.mapv(|t| Complex32::from_polar(1.0, t));

    // --- Hypocycloid inner and outer ---
    let exp1 = theta.mapv(|t| Complex32::from_polar(1.0, t));
    let exp_k1 = theta.mapv(|t| Complex32::from_polar(1.0, (k - 1.0) * t));
    let exp_k = theta.mapv(|t| Complex32::from_polar(1.0, k * t));

    let hypo_inner = exp1.mapv(|z| (k - 1.0) * z) + exp_k1.mapv(|z| z.conj());
    let hypo_outer = exp1.mapv(|z| k * z) + exp_k.mapv(|z| z.conj());

    // --- Build levels ---
    let step = 8;
    let mut level: Vec<Array1<Complex32>> = Vec::new();

    for i in (0..theta.len()).step_by(step) {
        let rot = Complex32::from_polar(1.0, -theta[i] / k);
        let hypo_rotate = hypo_inner.mapv(|z| z * rot) + circle[i];
        level.push(hypo_rotate.slice(s![..-1]).to_owned());
    }

    // --- Concatenate all levels ---
    let h_array = concatenate(
        Axis(0),
        &level.iter().map(|arr| arr.view()).collect::<Vec<_>>(),
    )
    .unwrap();

    // --- Convert to 3D points ---
    let n = resolution as u32;
    let height = 0.2_f32;
    let points: Vec<(f32, f32, f32)> = h_array
        .iter()
        .enumerate()
        .map(|(i, &z)| (z.re, z.im, (i as f32 / n as f32) as f32 * height))
        .collect();

    // --- Faces ---
    let n_levels = level.len() as u32;
    let mut faces: Vec<Vec<u32>> = Vec::new();

    for i in 0..n_levels - 1 {
        for j in 0..n {
            faces.push(Vec::from([
                i * n + j,
                (i + 1) * n + (j + 1) % n,
                (i + 1) * n + j,
            ]));
        }
    }

    for i in 0..n_levels - 1 {
        for j in 0..n {
            faces.push(Vec::from([
                i * n + j,
                i * n + (j + 1) % n,
                (i + 1) * n + (j + 1) % n,
            ]));
        }
    }

    // --- Bottom cap ---
    let bottom: Vec<u32> = (1..n).rev().collect();
    faces.push(bottom);
    // if bottom.len() >= 3 {
        // faces.push([bottom[0], bottom[1], bottom[2]]);
    // }

    // --- Top cap ---
    let top: Vec<u32> = ((n_levels - 1) * n..n_levels * n).collect();
    faces.push(top);
    // if top.len() >= 3 {
        // faces.push([top[0], top[1], top[2]]);
    // }

    let piece = D3::polyhedron(points, faces);
    // piece = scale([10,10,15])(piece)
    // println!("Generated {} points and {} faces", points.len(), faces.len());
    println!("$fn=128;\n{}", piece);
}

