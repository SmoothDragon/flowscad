use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let gap = 0.2;
    let side = 20.;
    let w_wall = 4.;
    let solution = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x0377_0757_0776), c3:BitCube3(0o033_033)}, side, gap);
    let result = solution;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

