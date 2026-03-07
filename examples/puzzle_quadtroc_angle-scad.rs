use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let gap = 0.2;
    let side = 20.;
    let w_wall = 4.;
    let troc = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x1), c3:BitCube3(0o0)}, side, gap);
    let duo_troc = D3:: polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x10001), c3:BitCube3(0o0)}, side, gap);
    let quad_troc = D3:: polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x33), c3:BitCube3(0o0)}, side, gap);
    let shape = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x26406640060), c3:BitCube3(0o076076)}, side, gap);
    // let shape = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x06608421), c3:BitCube3(0o673)}, side, gap);
    let shape = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x066006600660), c3:BitCube3(0o272272272)}, side, gap);
    let shape = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x27202720672), c3:BitCube3(0o003033233)}, side, gap);
    let shape = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x0660ffff), c3:BitCube3(0o020777)}, side, gap);
    let shape = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x0200777), c3:BitCube3(0o033)}, side, gap)
        .translate([-1.*side, -1.*side, 1.*side])
        .add_map(|x| 
            x.clone().rotate_x(90)
            + x.clone().rotate_x(180)
            + x.clone().rotate_x(-90)
            + x.clone().rotate_y(90)
            + x.clone().rotate_y(-90)
            )
        ;

    // let shape = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x00033), c3:BitCube3(0o01)}, side, gap)
        // .translate([-0.5*side, -0.5*side, 0.5*side])
        // .add_map(|x| 
            // x.clone().rotate_x(90)
            // + x.clone().rotate_x(180)
            // + x.clone().rotate_x(-90)
            // + x.clone().rotate_y(90)
            // + x.clone().rotate_y(-90)
            // )
        // ;


    println!("$fn=128;\n{}", &shape);
    Ok(())
}

