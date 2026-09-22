use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let moon = D3::Import{filename: "moon_visible.stl", center: false};
    let result = moon.scale(0.85)
        .translate([-1.0, 0.5, 0.01])
        .sub(D3::cylinder_d(5.0, 70.0)
            // .translate([1.3, -0.5, -0.01])
        );
    let numbers = (1..=6)
        .map(|x| D2::text(x.to_string())
            .linear_extrude(3)
            .translate_y(10)
            .rotate_x(70)
            .translate_y(-41.5)
            .rotate_z(80+x*60)
        )
        .union()
        ;
        // .map(|x| D2::text(x.to_string())
             // .translate_y(-inner_spread/2.6)
             // .rotate(30+x*60))
        // .union()
        // .linear_extrude(10)
        // .translate_z(-h_peg/2)
        // .rotate_y(180)
        // ;
    println!("$fn=64;\n{}", result + numbers);
    Ok(())
}
