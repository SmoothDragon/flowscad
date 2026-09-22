use flowscad::*;
use anyhow::Result;
use ::bitperm::*;


fn prism_troc(gap: f32) -> D3 {
    let side = 20.;
    D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x0777), c3:BitCube3(0o0)}, side, gap)
        .add_map(|x| x.translate([0.5*side,0.5*side,0.5*side]))
        .add_map(|x| x.translate([0.5*side,0.5*side,0.5*side]))
}

fn main() -> Result<()> {
    let gap = 0.2;
    let side = 20.;
    let w_wall = 4.;
    let base = D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0xffff), c3:BitCube3(0o0)}, side, gap)
        .add(D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x7), c3:BitCube3(0o7)}, side, gap)
            .add_map(|x| x.translate_x(side))
            .add_map(|x| x.translate([0.5*side,0.5*side,0.5*side]))
            .translate_y(3.0*side)
        )
        .add(D3::polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x111), c3:BitCube3(0o111)}, side, gap)
            .add_map(|x| x.translate_y(side))
            .add_map(|x| x.translate([0.5*side,0.5*side,0.5*side]))
            .translate_x(3.0*side)
        )
        ;
    /*
    let pprism_troc = D3:: polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x0777), c3:BitCube3(0o0)}, side, gap)
        .add_map(|x| x.translate([0.5*side,0.5*side,0.5*side]))
        .add_map(|x| x.translate([0.5*side,0.5*side,0.5*side]))
        .translate([0.5*side+w_wall, 0.5*side+w_wall, 0.5*side])
        ;
    let base = D3::cuboid([3.*side+2.0*w_wall, 3.*side+2.0*w_wall, 0.5*side]);
    let angle = base.clone().translate([0.5*side,0.5*side,0.]) - base.clone().translate_z(0.1) - base.clone().translate_z(0.1);
    let angle = angle.translate([-0.5*side, -0.5*side, 0.]);
    let w_square = 3.*side+2.*w_wall;
    let angle = D2::square(w_square)
        .sub(D2::square(1.414 * w_square)
            .center()
            .rotate(45)
        )
        .linear_extrude(0.5*side)
        ;
    let steps = 100;
    let slide = (0..steps)
        .map(|ii| { let xx = 0.2 * (ii as f32); angle.clone().translate([xx, xx, xx]) } )
        .union()
        ;
    */
    // let base = base + angle.clone().translate([side,side,0.5*side]);
    // let base = base + angle.clone().translate([-0.5*side, -0.5*side, 0.5*side]);
    // let base = base + slide;

    let result = base
        .and(
            D3::cuboid( [ 3.0*side, 3.0*side, 0.5*side])
            .add(D3::cuboid( [ 3.0*side, 3.0*side, 0.5*side])
                .translate([ 1.0*side, 1.0*side, 1.0*side])
            )
            .hull()
        )
        ;
    // let result = result.clone()
        // .add_map(|x| x
            // .rotate([0,180,90])
            // .translate([ 4.0*side, 4.0*side, 2.5*side])
        // )
        // ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

