use flowscad::*;
use anyhow::Result;
use ::bitperm::*;

fn main() -> Result<()> {
    let side = 30.0;
    let dot = D3::from(BitTroc4{c4:BitCube4(0x01), c3:BitCube3(0o0)}).color(ColorEnum::Red).scale(0.2);
    let cube = ((dot.clone() + dot.clone().translate_x(side)).hull())
        .add_map(|x| x.translate_y(side))
        .add((dot.clone() + dot.clone().translate_y(side)).hull()
            .add_map(|x| x.translate_x(side))
            )
        .add_map(|x| x.translate_z(side))
        .add(
            (dot.clone() + dot.clone().translate_z(side)).hull()
            .add_map(|x| x.translate_x(side))
            .add_map(|x| x.translate_y(side))
        )
        .add((dot.clone() + dot.clone().translate([side,side,side])).hull())
        .add((dot.clone().translate_x(side) + dot.clone().translate([0.,side,side])).hull())
        .add((dot.clone().translate_y(side) + dot.clone().translate([side,0.,side])).hull())
        .add((dot.clone().translate_z(side) + dot.clone().translate([side,side,0.])).hull())
        ;
    let outer = cube.clone()
        .iter_translate([side,0.,0.], 3).union()
        .iter_translate([0.,side,0.], 3).union()
        .iter_translate([0.,0.,side], 3).union()
        ;

    let inner = cube.clone()
        .iter_translate([side,0.,0.], 2).union()
        .iter_translate([0.,side,0.], 2).union()
        .iter_translate([0.,0.,side], 2).union()
        .translate([side/2., side/2., side/2.])
        ;

    let result = outer + inner;


    println!("$fn=128;\n{}", &result);
    Ok(())
}

