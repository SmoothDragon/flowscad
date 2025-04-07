use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let result = D3::polytroc_from_bittroc4(
        BitTroc4{c4:BitCube4(0x9009_0660_0660_9009), c3:BitCube3(0o505020505)},
        15., 0.1);

    println!("$fn=128;\n{}", &result);
    Ok(())
}

