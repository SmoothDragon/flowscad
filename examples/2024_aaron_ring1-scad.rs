use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let d_inner = X(103.1);
    let w_lower = X(11.);
    let w_upper = X(5.4);
    let h_ring = X(58.74);
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

