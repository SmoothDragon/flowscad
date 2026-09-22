use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {

    // let pattern = D2::Import{filename: "cow-marker-1.dxf", center: true};
    let d = 32.0;
    let pattern = (1..=6)
        .map(|ii| 
            D2::circle_d(d)
            .sub(
            D2::Import(format!("cow-marker-{}.dxf", ii), false)
                .translate([-0.5, -0.5])
                .translate([-d/4.0, -d/4.0])
                .scale(2.0)
                .offset_radius(0.2)
            )
            .linear_extrude(1.0)
            .translate_z(1.0)
            .add(D2::circle_d(d).linear_extrude(1.0))
            .translate_x(35.0 * (ii-1) as f32)
        )
        .union()
        ;

    let result = pattern;
    println!("$fn=128;\n{}", &result);
    Ok(())
}
