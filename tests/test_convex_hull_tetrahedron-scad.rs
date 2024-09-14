use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let tetrahedron = D3::convex_hull([
        [1,1,1], 
        [1,-1,-1], 
        [-1,1,-1], 
        [-1,-1,1], 
        ]);
    println!("{}", &tetrahedron);
    Ok(())
}

