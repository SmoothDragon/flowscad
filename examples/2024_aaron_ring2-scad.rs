use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let d_inner = X(103.1);
    let w_lower = X(5.4);
    // let w_upper = X(0.5);
    let w_upper = X(0.8);
    // let h_ring = X(46.04);
    let h_ring = X(44.6);
    let layer = X(0.2);

    let result = D2::rectangle( (w_upper, layer) )
        .translate_y(h_ring -layer)
        .add(D2::rectangle( (w_lower, layer) ))
        .hull()
        .translate_x(d_inner/2)
        .rotate_extrude(360)
        ;

    println!("$fn=512;\n{}", &result);
    Ok(())
}

