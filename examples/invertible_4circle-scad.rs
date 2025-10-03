use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let r = X(10.);
    let whole = D2::circle_r(r)
        .translate( [2*r, 2*r] )
        .iter_rotate_equal(4)
        .union()
        .add(D2::square(4*r).center())
        ;
    let remove = D2::circle_r(r)
        .translate_x(2*r)
        .iter_rotate_equal(4)
        .union()
        ;
    let result = (whole - remove)
        .linear_extrude(10)
        ;
    println!("$fn=256;\n{}", &result);
    Ok(())
}
