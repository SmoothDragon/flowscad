use anyhow::Result;
use bitperm::BitCube4;
use flowscad::*;

fn main() -> Result<()> {
    let x: BitCube4 = 0x1003f.into();

    let result: D3 = x.rotate_y().into();

    println!("$fn=128;\n{}", &result);
    Ok(())
}
