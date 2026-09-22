use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {

    // let pattern = D2::Import{filename: "cow-marker-1.dxf", center: true};
    let d = 32.0;
    let pattern = (1..=6)
        .map(|ii| 
            D2::Import(format!("cow-marker-{}.dxf", ii), false)
            .translate([-0.5, -0.5])
            .translate([-d/4.0, -d/4.0])
            .scale(2.0)
            .offset_radius(0.1)
            .and(D2::circle_r(16))
            .translate_x(35.0 * (ii-1) as f32)
            )
        .union()
        ;

    let result = pattern.linear_extrude(1.0);
    println!("$fn=64;\n{}", &result);
    Ok(())
}
