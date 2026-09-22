use flowscad::*;
use anyhow::Result;
use ::bitperm::*;

fn main() -> Result<()> {
    let gap = 0.2;
    let side = 20.;
    let w_wall = 4.;
    let bucolic = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x0001_0001_0057), c3:BitCube3(0o3)}, side, gap);
    let rhombus = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x0003), c3:BitCube3(0o3)}, side, gap);
    let square = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x0033), c3:BitCube3(0o0)}, side, gap);
    let result = rhombus
        .iter_translate([2.5*side, 0., 0.], 4)
        .union()
        .iter_translate([0., 2.5*side, 0.], 4)
        .union()
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

