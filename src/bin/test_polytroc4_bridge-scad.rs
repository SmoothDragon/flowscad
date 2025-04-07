use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let result = D3::polytroc_from_bittroc4(
        BitTroc4{c4:BitCube4(0x8001_0401_0021_111f), c3:BitCube3(0o400020001)},
        15., 0.1);

    println!("$fn=128;\n{}", &result);
    Ok(())
}

