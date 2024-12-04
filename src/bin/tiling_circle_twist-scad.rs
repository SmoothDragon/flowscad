use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let h_piece = X(50.);
    let layer = X(0.2);
    let layers: u64 = 250;  // 50/0.2

    let slice = D2::rectangle( (1,2) ).translate( (0, -1) )
        .add(D2::circle_r(1))
        ;
    let disc = slice.linear_extrude(2./250.)
        // .translate_z(-1./250.)
        ;
    let half = arange(-1., 1., 250)
        .map(|x| disc.clone().translate( ((1.-x*x).powf(0.5), 0, x)))
        .union()
        .translate_x(-1.)
        ;
    // let result = half.clone().rotate_z
        // .add(
    let result = half
        .add_map(|x| x.rotate((90,0,180)))
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

