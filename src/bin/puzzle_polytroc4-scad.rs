use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    // let result = D3::from(BitTroc4{c4:BitCube4(0x2007770020), c3:BitCube3(0o033033)}).color(ColorEnum::Red);
    let result = 
        D3::from(BitTroc4{c4:BitCube4(0x0007400000), c3:BitCube3(0o0)}).color(ColorEnum::Red)
        + D3::from(BitTroc4{c4:BitCube4(0x00000000), c3:BitCube3(0o30031)}).color(ColorEnum::Yellow)
        + D3::from(BitTroc4{c4:BitCube4(0x00200020), c3:BitCube3(0o1000)}).color(ColorEnum::Blue)
        + D3::from(BitTroc4{c4:BitCube4(0x0000170000), c3:BitCube3(0o0)}).color(ColorEnum::Orange)
        + D3::from(BitTroc4{c4:BitCube4(0x20_0000_0000), c3:BitCube3(0o2002)}).color(ColorEnum::Pink).translate((-30,-30,0))
        ;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

