use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let octahedron = D3::convex_hull([
        [1,1,0], [-1,1,0],[-1,-1,0],[1,-1,0],  // point in xy-plane
        [1,0,1], [-1,0,1],[-1,0,-1],[1,0,-1],  // point in xz-plane
        [0,1,1], [0,-1,1],[0,-1,-1],[0,1,-1],  // point in yz-plane
        ]);
    println!("{}", &octahedron);
    Ok(())
}

