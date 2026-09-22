use flowscad::*;
use anyhow::Result;
use ::bitperm::*;

fn main() -> Result<()> {
    let gap = 0.2;
    let side = 20.;
    let w_wall = 4.;
    let result = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x0020_0001), c3:BitCube3(0o1)}, side, gap);
    println!("$fn=128;\n{}", &result);
    Ok(())
}

