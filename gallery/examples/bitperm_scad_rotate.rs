use anyhow::Result;
/// The example rotates a three cuboid of different lengths joined at the origin.
use bitperm::BitCube4;
use flowscad::*;

fn main() -> Result<()> {
    let x: BitCube4 = 0x0000000100011113.into();

    let result: D3 = x.into();
    println!("$fn=128;\n{}", &result);
    let result: D3 = x.rotate_d().into();
    let result = result.translate((15, 15, 15));
    println!("$fn=128;\n{}", &result);
    let result: D3 = x.rotate_d().rotate_d().into();
    let result = result.translate((30, 30, 30));
    println!("$fn=128;\n{}", &result);
    Ok(())
}
