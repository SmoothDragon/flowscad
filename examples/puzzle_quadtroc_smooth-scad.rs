use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let gap = 0.1;
    let side = 20.;
    let w_wall = 4.;
    let troc = D3:: polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x1), c3:BitCube3(0o0)}, side, gap);
    let duo_troc = D3:: polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x10001), c3:BitCube3(0o0)}, side, gap);
    let quad_troc = D3:: polytroc_from_bittroc4(BitTroc4{c4:BitCube4(0x33), c3:BitCube3(0o0)}, side, gap);

    let holder = D3::beveled_box(v3(3.*side+w_wall, 3.*side+2.*w_wall, 3.*side+w_wall), 2.)
        .translate(-v3(1,1,1)*(w_wall+gap))
        ;
    let result = holder.clone()
        .sub(D3::cube(3.*side))
        // .add(troc.clone().translate(v3(1,0,1)*side))
        .add(duo_troc.clone().translate(v3(1,0,0)*side))
        .add(quad_troc.clone()
            .translate(v3(0,1,0)*side)
            .rotate_y(-90)
        )
        .add(duo_troc.clone().translate(v3(1,3,0)*side))
        .add(troc.clone()
            .add_map(|x| x.translate_y(3.*side))
        )
        .add(D3::hull(troc.clone()+troc.clone().translate(v3(3,0,0)*side))
            .translate(v3(-1,-1,-1)*side*0.25)
            .add_map(|x| x.translate_y(3.5*side))
            )
        .add(D3::hull(troc.clone()+troc.clone().translate(v3(0,3,0)*side))
            .translate(v3(-1,-1,-1)*side*0.25)
            )
        .add(D3::hull(troc.clone()+troc.clone().translate(v3(0,0,3)*side))
            .translate(v3(-1,-1,-1)*side*0.25)
            .add_map(|x| x.translate_y(3.5*side))
            )
        .and(holder)
        .translate_z(w_wall)
        ;
    let pieces = quad_troc.clone().translate(v3(-2,0,0)*side)
        + quad_troc.clone().translate(v3(-2,2.5,0)*side)
        + quad_troc.clone().translate(v3(-2,-2.5,0)*side)
        + quad_troc.clone().translate(v3(0.5,-2.5,0)*side)
        + quad_troc.clone().translate(v3(3.,-2.5,0)*side)
        + quad_troc.clone().translate(v3(-2,5,0)*side)
        + quad_troc.clone().translate(v3(0.5,5,0)*side)
        + quad_troc.clone().translate(v3(3.,5,0)*side)
        ;
    let result = result + pieces.translate_z(side*0.5-gap);
    println!("$fn=128;\n{}", &result);
    Ok(())
}

