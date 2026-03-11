use flowscad::*;
use anyhow::Result;
use bitperm::*;

fn main() -> Result<()> {
    let gap = 0.05;
    let side = 20.;
    let w_wall = 4.;
    let troc = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x1), c3:BitCube3(0o0)}, side, gap);
    let duo_troc = D3:: polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x10001), c3:BitCube3(0o0)}, side, gap);
    let quad_troc = D3:: polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x33), c3:BitCube3(0o0)}, side, gap);
    let cut_quad_troc = quad_troc.translate([-0.5*side, -0.5*side, 0.]) - 
        D3::cube(2.*side)
        .rotate_x(45)
        .translate_z(-gap)
        .add_map(|x| x.mirror([0,0,1]))
        ;

    let result = cut_quad_troc.rotate_y(-90);
    // let result = holder2;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

