// pub use crate::Deg;
// pub use crate::Rad;
pub use crate::D2Trait;
pub use crate::Face;
pub use crate::*;

pub fn polygon_stack(layers: Vec<Face>, h_layer: f32) -> D3 {
    // First face, add bottom layer of first stack
    // for every layer add triangles between two layers
    // Add top layer
    let mut points: Vec<(f32, f32, f32)> = Vec::new();
    for ii in 0..layers.len() {
        for point in layers[ii].0.clone() {
            points.push( (point.re, point.im, (ii as f32) * h_layer) );
        }
    }

    let mut faces: Vec<Vec<usize>> = Vec::new();
    let n = layers.len();
    let points_per_layer: usize = layers[0].0.len();  // The point list will be in $n$-long bands for each level

    for i in 0..(n - 1) {
        for j in 0..points_per_layer {
            faces.push(Vec::from([
                i * points_per_layer + j,
                (i + 1) * points_per_layer + (j + 1) % points_per_layer,
                (i + 1) * points_per_layer + j,
            ]));
        }
    }

    for i in 0..(n - 1) {
        for j in 0..points_per_layer {
            faces.push(Vec::from([
                i * points_per_layer + j,
                i * points_per_layer + (j + 1) % points_per_layer,
                (i + 1) * points_per_layer + (j + 1) % points_per_layer,
            ]));
        }
    }

    let bottom: Vec<usize> = (0..points_per_layer).rev().collect();
    faces.push(bottom);
    let top: Vec<usize> = ((n - 1) * points_per_layer..n * points_per_layer).collect();
    faces.push(top);
    D3::polyhedron(points, faces)
}
