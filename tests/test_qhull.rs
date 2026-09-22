//! Smoke tests for the `qhull` dependency used by flowscad's 3D convex hull code.

use std::collections::BTreeMap;
use qhull::Qh;

use flowscad::*;

#[test]
fn qhull_2d_convex_hull_of_square_with_interior_point() {
    // Three of the four points form the convex hull; the fourth (0.25, 0.25)
    // is strictly interior and should not appear in any simplex.
    let qh = Qh::builder().compute(true).build_from_iter([
        [0.0, 0.0],
        [1.0, 0.0],
        [0.0, 1.0],
        [0.25, 0.25],
    ]).unwrap();

    assert_eq!(qh.num_faces(), 3);
}

#[test]
fn qhull_cuboctahedron_produces_all_vertices_and_expected_faces() {
    let vertices = [
        [ 1.0,  1.0,  0.0], [-1.0,  1.0,  0.0], [-1.0, -1.0,  0.0], [ 1.0, -1.0,  0.0],
        [ 1.0,  0.0,  1.0], [-1.0,  0.0,  1.0], [-1.0,  0.0, -1.0], [ 1.0,  0.0, -1.0],
        [ 0.0,  1.0,  1.0], [ 0.0, -1.0,  1.0], [ 0.0, -1.0, -1.0], [ 0.0,  1.0, -1.0],
    ];
    let qh = Qh::builder().compute(true).build_from_iter(vertices).unwrap();

    let mut faces: Vec<Vec<u32>> = Vec::new();
    let mut vertex: BTreeMap<u32, XYZ> = BTreeMap::new();

    for face in qh.faces() {
        let face_num = face
            .vertices()
            .unwrap()
            .iter()
            .map(|v| {
                let v_id = v.id() - 1; // qhull is 1-indexed; we want 0-indexed
                vertex.entry(v_id).or_insert_with(|| {
                    let xyz = v.point();
                    v3(xyz[0], xyz[1], xyz[2])
                });
                v_id
            })
            .collect::<Vec<u32>>();
        faces.push(face_num);
    }

    // Every input vertex participates in the hull; the BTreeMap should hold
    // all 12 keys 0..=11 with no gaps.
    assert_eq!(vertex.len(), 12);
    assert_eq!(*vertex.first_entry().unwrap().key(), 0);
    assert_eq!(*vertex.last_entry().unwrap().key() as usize, vertex.len() - 1);

    // A cuboctahedron has 14 faces: 8 triangles + 6 squares.
    assert_eq!(faces.len(), 14);
}
