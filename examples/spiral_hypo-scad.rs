use ndarray::{Array1, concatenate, Axis, s};
use num_complex::Complex32;
use std::f32::consts::PI;

use flowscad::*;

fn exp1j(theta: f32) -> Complex32 {
    Complex32::new(theta.cos(), theta.sin())
}

fn main() {
    let h_layer = X(0.2);
    let w_nozzle = X(0.4);
    let gap = X(0.4);

    let r = X(20.);
    let h: X = X(100.);
    let layers: i32 = (h / h_layer).trunc();
    let layers: u32 = 2;
    let k = 3.0_f32;  // points of hypocycloid
    let resolution = 16;

    let theta = Array1::linspace(0.0, 2.0 * PI, resolution + 1);
    let circle = theta.mapv(|t| exp1j(t));

    let hypo_inner = theta.mapv(|t| (k - 1.0) * exp1j(t) + exp1j((k - 1.0) * t).conj());
    let hypo_outer = theta.mapv(|t| k * exp1j(t) + exp1j(k * t).conj());

// step = 8

// for i in range(0, len(theta), step):
    // rotate = np.exp(1j*-theta[i]/k)
    // hypo_rotate = rotate*hypo_inner + circle[i]
    // level.append(hypo_rotate[:-1])

    // --- Build levels ---
    let step = 8;

    let level: Vec<Array1<Complex32>> = (0..theta.len()).step_by(step)
        .map(|ii| hypo_inner
            .mapv(|z| z * exp1j(-theta[ii] / k) + circle[ii])
            .slice(ndarray::s![..-1])
            .to_owned()
        )
        .collect()
        ;

    // --- Concatenate all levels ---
    let h_array = concatenate(
        Axis(0),
        &level.iter().map(|arr| arr.view()).collect::<Vec<_>>(),
    )
    .unwrap();

    // --- Convert to 3D points ---
    let height = 0.2_f32;
    let points: Vec<(f32, f32, f32)> = h_array
        .iter()
        .enumerate()
        .map(|(i, &z)| (z.re, z.im, (i as f32 / layers as f32) as f32 * height))
        .collect();

    // --- Faces ---
    let n_levels = level.len() as u32;
    let mut faces: Vec<Vec<u32>> = Vec::new();

    for i in 0..n_levels - 1 {
        for j in 0..layers {
            faces.push(Vec::from([
                i * layers + j,
                (i + 1) * layers + (j + 1) % layers,
                (i + 1) * layers + j,
            ]));
        }
    }

    for i in 0..n_levels - 1 {
        for j in 0..layers {
            faces.push(Vec::from([
                i * layers + j,
                i * layers + (j + 1) % layers,
                (i + 1) * layers + (j + 1) % layers,
            ]));
        }
    }

    // --- Bottom cap ---
    let bottom: Vec<u32> = (1..layers).rev().collect();
    faces.push(bottom);
    // if bottom.len() >= 3 {
        // faces.push([bottom[0], bottom[1], bottom[2]]);
    // }

    // --- Top cap ---
    let top: Vec<u32> = ((n_levels - 1) * layers..n_levels * layers).collect();
    faces.push(top);
    // if top.len() >= 3 {
        // faces.push([top[0], top[1], top[2]]);
    // }




    /*
h_array = np.concatenate(level)
n = resolution
height = .2
points = [(point.real, point.imag, (i//n)*height) for i,point in enumerate(h_array)]
faces = [(i*n + j, (i+1)*n + (j+1)%n, (i+1)*n +j) for i in range(len(level)-1) for j in range(n)]
faces.extend([(i*n + j, i*n + (j+1)%n, (i+1)*n +(j+1)%n) for i in range(len(level)-1) for j in range(n)])
faces.append(list(range(n-1,0,-1)))
N = len(level)
faces.append(list(range((N-1)*n, N*n)))
*/
    let piece = D3::polyhedron(points, faces);
    // piece = scale([10,10,15])(piece)
    // println!("Generated {} points and {} faces", points.len(), faces.len());
    println!("$fn=128;\n{}", piece);
}

