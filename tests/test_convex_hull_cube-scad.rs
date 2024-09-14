use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let cube = D3::convex_hull([
        [1,1,1], 
        [1,1,-1], 
        [1,-1,-1], 
        [1,-1,1], 
        [-1,-1,1], 
        [-1,-1,-1], 
        [-1,1,-1], 
        [-1,1,1], 
        ]);
    println!("{}", &cube);
    Ok(())
}

