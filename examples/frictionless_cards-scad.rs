use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let d_cow = X(20.0);
    let d_tri = X(15.0);
    let d_hex = X(20.0);
    let cow_ii = 1;
    let tri_ii = 2;
    let hex_ii = 3;
    let cow = D2::circle_d(d_cow)
        .sub(D2::text(cow_ii.to_string()))
        ;
    let tri = D2::regular_polygon(3, d_tri)
        .rotate(90)
        .sub(D2::text(tri_ii.to_string()))
        ;
    let hex = D2::regular_polygon(6, d_hex/2)
        .sub(D2::text(hex_ii.to_string()))
        ;
    let result = cow;
    println!("$fn=64;\n{}", &result);
    Ok(())
}
