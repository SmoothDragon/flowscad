use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let gap = 0.2;
    let side = 20.;
    let w_wall = 4.;
    let bucolic = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x0001_0001_0057), c3:BitCube3(0o3)}, side, gap);
    let result = bucolic;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

