use flowscad::*;
use anyhow::Result;


/// Curved design based on simple 3-piece burr puzzle.


fn main() -> Result<()> {
    let s = X(10.);
    let h = X(2.75);
    let bevel = X(1.);
    let w = 3*h;
    let l = 3*w;
    let gap = X(0.1);

    let piece = (D2::circle_d(3*s).translate( (-2.75*s, 3*s) )
        + ((D2::circle_r(4*s)-D2::circle_r(1.5*s)) & D2::square(4*s))
            .translate( (-2.75*s, 0.5*s) )
        + D2::rectangle( (2.5*s, s) ).center()
        )
        .add_map(|x| x.rotate(180))
        .sub(D2::rectangle( (h+gap, 3*s+gap) ).center())
        .sub(D2::rectangle( (3*s+gap, 0.5*(2.5*s-h)+gap) ).center().translate_x(1.5*s))
        .rotate(-45)
        ;

    println!("$fn=256;\n{}", &piece);
    Ok(())
}

