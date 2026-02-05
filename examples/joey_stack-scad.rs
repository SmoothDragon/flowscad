use ndarray::array;
use ndarray::Array2;
use ndarray::{Array1, concatenate, Axis, s};
// use num_complex::Complex32;
use std::f32::consts::PI;

use flowscad::*;

fn triangle_main(theta: f32) -> D3 {
    let sint = (1.-theta.cos()) / theta.sin();
    let cost = (1.-sint*sint).sqrt();
    (D3::convex_hull(vec![
        // [0., 0., 0.],
        [0., 0., 1.-theta.sin()],  // center fold point
        [1., cost, sint+1.-theta.sin()],
        [-1., cost, sint+1.-theta.sin()],
        [0., cost, 0.],
        [1.-theta.sin(), cost, 0.],
    ]) 
    +
    D3::convex_hull(vec![
        // [0., 0., 0.],
        [0., 0., 1.-theta.sin()-0.01],  // center fold point
        [0., 0., 1.-theta.sin()],  // center fold point
        [1., cost, sint+1.-theta.sin()],
        [theta.cos(), 0., 1.],
    ])
    )
    .add_map(|x| x.mirror([1,0,0]))
    .add_map(|x| x.mirror([0,1,0]))
    .add_map(|x| x.mirror([0,0,1]))
    // .minkowski(D3::sphere_r(0.1))
    .add_map(|x| x.rotate_y(90).translate_y(cost))
    // .add_map(|x| x.translate_y(2.*cost))
    // .add_map(|x| x.translate_y(4.*cost))
    // .add_map(|x| x.translate_y(8.*cost))
    // .rotate_x(90)
    // .intersection(D3::cube(10.))
    // .scale(15)
}

fn main() {
    let theta = 45. * PI / 180.0;
    let piece = triangle_main(theta);
    println!("$fn=128;\n{}", piece);
}
/*
fn main() {
    let theta = 45. * PI / 180.0;
    let sint = (1.-theta.cos()) / theta.sin();
    let cost = (1.-sint*sint).sqrt();
    /*
    let mut points: Array2<f32> = Array2::zeros( (20, 3) );
    points.row_mut(0).assign(&array![0., 0., 0.]);
    points.row_mut(1).assign(&array![theta.cos(), 0., theta.sin()]);
    points.row_mut(2).assign(&array![-theta.cos(), 0., theta.sin()]);
    points.row_mut(3).assign(&array![1., cost, sint]);
    points.row_mut(4).assign(&array![-1., cost, sint]);
    points.row_mut(5).assign(&array![1., -cost, sint]);
    points.row_mut(6).assign(&array![-1., -cost, sint]);
    // let mut points: Array1<(f32, f32, f32)> = array![
        // (theta.cos(), 0., theta.sin()),
    // ];
    */
    let mut points: Array2<f32> = array![
        [0., 0., 0.],
        [theta.cos(), 0., theta.sin()],
        [-theta.cos(), 0., theta.sin()],
        [1., cost, sint],
        [-1., cost, sint],
        [1., -cost, sint],
        [-1., -cost, sint],
    ];

    for ii in 0..points.len() {
        let mut target = points.row_mut(ii);
        target += &array![0., 0., 1.-theta.sin()];
    }
    println!("{:?}", points);

    let faces: Array2<usize> = array![
        [0, 3, 1],
        [0, 2, 4],
        [0, 4, 3],
    ];

    println!("{:?}", &faces);
    let p: Vec<Vec<_>> = points.rows().into_iter().map(|r| r.to_vec()).collect();
    let f: Vec<Vec<_>> = faces.rows().into_iter().map(|r| r.to_vec()).collect();
    // println!("{:?}", faces.into_raw_vec());
    // println!("{:?}", faces.to_vec());
    let piece = D3::polyhedron(p, f);
    // piece = scale([10,10,15])(piece)
    // println!("Generated {} points and {} faces", points.len(), faces.len());
    // println!("$fn=128;\n{}", piece);
}
*/
