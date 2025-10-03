use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let result = D3::from(BitTroc4{c4:BitCube4(0x20), c3:BitCube3(0o012)}).color(ColorEnum::Red);
    println!("$fn=128;\n{}", &result);
    Ok(())
}

