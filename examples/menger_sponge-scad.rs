use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let side = X(64.4);
    let iterations = 3;
    let scale = X(1.0)/3;
    let shift = side*scale;
    let mut hole: D2 = D2::square(side*scale).center();
    let mut next_hole: D2 = hole.clone().scale(scale);
    for _ in 0..iterations {
        next_hole += next_hole.clone().translate_x(shift)
            + next_hole.clone().translate_x(-shift);
        next_hole += next_hole.clone().translate_y(shift)
            + next_hole.clone().translate_y(-shift);
        hole = hole.clone() + next_hole;
        next_hole = hole.clone().scale(scale);
    }
    let cutter = hole.linear_extrude(2*side).translate_z(-side);

    let result = D3::cube(side).center() 
        - cutter.clone()
        - cutter.clone().rotate_x(90)
        - cutter.clone().rotate_y(90)
        ;

    println!("$fn=100;\n{}", &result);
    Ok(())
}
