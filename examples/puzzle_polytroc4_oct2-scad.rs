use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let result = D3::from(BitTroc4{c4:BitCube4(0x02002720020), c3:BitCube3(0o033033)}).color(ColorEnum::Red);
    // let result = 
        // D3::from(BitTroc4{c4:BitCube4(0x0272_0777_0272), c3:BitCube3(0o033032)}).color(ColorEnum::Red)
        // + 
        // D3::from(BitTroc4{c4:BitCube4(0x0000_0021_0012), c3:BitCube3(0o000_001)}).color(ColorEnum::Blue).translate((-10,-10,-10))
        // ;
    // let result = 
        // D3::from(BitTroc4{c4:BitCube4(0x0007400000), c3:BitCube3(0o0)}).color(ColorEnum::Red)
        // + D3::from(BitTroc4{c4:BitCube4(0x00000000), c3:BitCube3(0o30031)}).color(ColorEnum::Yellow)
        // + D3::from(BitTroc4{c4:BitCube4(0x0000170000), c3:BitCube3(0o0)}).color(ColorEnum::Orange)
        // + D3::from(BitTroc4{c4:BitCube4(0x20_0000_0000), c3:BitCube3(0o2002)}).color(ColorEnum::Pink).translate((-30,-30,0))
        // + D3::from(BitTroc4{c4:BitCube4(0x20_0000_0000), c3:BitCube3(0o2002)}).color(ColorEnum::Pink)
        // ;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

