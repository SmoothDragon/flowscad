use std::collections::BTreeMap;
use qhull::Qh;

use flowscad::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vertices =  [  // Cuboctahedron
        [1.0,1.0,0.0], [-1.0,1.0,0.0],[-1.0,-1.0,0.0],[1.0,-1.0,0.0],  // point in xy-plane
        [1.0,0.0,1.0], [-1.0,0.0,1.0],[-1.0,0.0,-1.0],[1.0,0.0,-1.0],  // point in xz-plane
        [0.0,1.0,1.0], [0.0,-1.0,1.0],[0.0,-1.0,-1.0],[0.0,1.0,-1.0],  // point in yz-plane
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
                let v_id = v.id()-1;  // 1-indexed to 0-indexed
                vertex.entry(v_id)
                    // .or_insert_with(|| v.point().to_owned());
                    .or_insert_with(|| {
                        let xyz =v.point();
                        v3(xyz[0], xyz[1], xyz[2])
                    });
                v_id
            })
            .collect::<Vec<u32>>()
            ;
        faces.push(face_num);
	}
    // Given the BTreeMap contains `n` distinct u32 entries,
    // assert the first entry is `0` and the last entry is `n-1`.
    // This proves all values `0..n` are present. 
    assert!(*vertex.first_entry().unwrap().key() == 0);
    assert!(*vertex.last_entry().unwrap().key() as usize == vertex.len()-1);

    println!("{:?}", &vertex);
    println!("{:?}", &vertex.len());
    println!("{:?}", faces);
    println!("{:?}", faces.len());
    let vertices: Vec<XYZ> = vertex.into_values().collect();
    println!("{:?}", vertices);


    Ok(())
}
