use flowscad::*;
use anyhow::Result;


/// Curved design based on simple 3-piece burr puzzle.


fn main() -> Result<()> {
    let outer_r = X(170.);   // Radius of arcs
    let inner_r = X(10.);   // Radius of arcs
    let gap = X(0.5);
    let fins = 60;
    let cut = X(30.);
    let shift = 1.7*cut;

    let slit = D2::rectangle( (cut, cut) )
        .sub(D2::rectangle( (cut, cut) ).translate( (gap, -gap) ))
        .rotate(-45)
        .add_map(move |x| x.translate_x(shift))
        .add_map(move |x| x.translate_x(2*shift))
        .add_map(move |x| x.translate_x(4*shift))
        ;
    // let slit2 = slit.clone().mirror((0,1)).translate( (0.707*cut, cut) );
    let slit2 = slit.clone().mirror((0,1)).translate( (shift/-2, 0.707*cut) );
    let piece = D2::rounded_rectangle( (200, 200), 5)
        .sub(slit.translate_y(10)
            .iter_translate((0,10), 17)
            .union()
        )
        .sub(slit2.translate_y(10)
            .iter_translate((0,10), 17)
            .union()
        )
        ;

    println!("$fn=64;\n{}", &piece);
    Ok(())
}

