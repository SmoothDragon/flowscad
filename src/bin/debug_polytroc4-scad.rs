use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let result = D3::polytroc_from_bittroc4(
        BitTroc4{c4:BitCube4(0xcc00_ce60_0673_0033), c3:BitCube3(0o330_376_066_u32)},
        15., 0.1);

    println!("$fn=128;\n{}", &result);
    Ok(())
}

