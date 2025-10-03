use flowscad::*;
use anyhow::Result;

fn level_shape(level:u32, total:u32) -> D2 {
    let n = 360;
    let r = X(30.);
    let h = 10;
    let shift = X(5.)*level/total;
    let points = (0..n)
        .map(|ii| {let theta = PI/180*ii;
             // v2(r, 0).rotate(theta.0)
             v2(r+shift*(8.0_f32*theta).sin(), 0).rotate(theta.0)
        })
         .collect::<Vec<XY>>()
         ;
    D2::polygon(points)
}


fn main() -> Result<()> {
    let levels = 100; // total number of levels 10mm at 0.1mm slice
    let result = (0..levels)
        .map(|ii| level_shape(ii, levels)
            .linear_extrude(0.1)
            .translate_z(ii as f32 * 0.1)
            )
        .union()
        ;

    println!("$fn=100;\n{}", &result);
    Ok(())
}
