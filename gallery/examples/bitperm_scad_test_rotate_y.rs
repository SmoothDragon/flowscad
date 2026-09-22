use anyhow::Result;
/// The example rotates a three cuboid of different lengths joined at the origin.
use bitperm::BitCube3;
use flowscad::*;

fn main() -> Result<()> {
    let x: BitCube3 = 0o4017.into();

    let result: D3 = x.into();
    println!("$fn=128;\n{}", &result);
    let result: D3 = x.rotate_y().into();
    let result = result.translate((15, 15, 15));
    println!("$fn=128;\n{}", &result);
    let result: D3 = x.rotate_y().rotate_y().into();
    let result = result.translate((30, 30, 30));
    println!("$fn=128;\n{}", &result);
    Ok(())
}
