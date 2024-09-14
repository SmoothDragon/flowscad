use flowscad::*;
use anyhow::Result;

pub fn main() -> Result<()> {
    let vertices =  [  // Cuboctahedron
        [1.0,1.0,0.0], [-1.0,1.0,0.0],[-1.0,-1.0,0.0],[1.0,-1.0,0.0],  // point in xy-plane
        [1.0,0.0,1.0], [-1.0,0.0,1.0],[-1.0,0.0,-1.0],[1.0,0.0,-1.0],  // point in xz-plane
        [0.0,1.0,1.0], [0.0,-1.0,1.0],[0.0,-1.0,-1.0],[0.0,1.0,-1.0],  // point in yz-plane
        ];
    println!("{:?}", convex_hull_3d(vertices.to_vec()));
    Ok(())
}
