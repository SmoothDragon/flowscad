use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let cuboid = D3::convex_hull([
            v3(  0,  0,  0 ),  //0
            v3( 10,  0,  0 ),  //1
            v3( 10,  7,  0 ),  //2
            v3(  0,  7,  0 ),  //3
            v3(  0,  0,  5 ),  //4
            v3( 10,  0,  5 ),  //5
            v3( 10,  7,  5 ),  //6
            v3(  0,  7,  5 )]  //7
                ).scad();
    println!("{}", &cuboid);
    Ok(())
}

