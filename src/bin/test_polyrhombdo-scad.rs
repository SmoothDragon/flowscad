use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let result = D3::polyrhombdo_from_bittroc4(
        BitTroc4{c4:BitCube4(0x8001_0401_0021_111f), c3:BitCube3(0o400020001)}, 15., 0.1);
        // BitTroc4{c4:BitCube4(0xcc00_ce60_0673_0033), c3:BitCube3(0o330_376_066_u32)},
        // BitTroc4{c4:BitCube4(0x1), c3:BitCube3(0o777777777_u32)},
        // 4., 0.1);

    println!("$fn=128;\n{}", &result);
    Ok(())
}

