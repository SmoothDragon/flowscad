use flowscad::*;
use anyhow::Result;


/// Curved design based on simple 3-piece burr puzzle.


fn main() -> Result<()> {
    let outer_r = X(170.);   // Radius of arcs
    let inner_r = X(10.);   // Radius of arcs
    let gap = X(0.5);
    let cut = X(1.0);
    let fins = 60;
    let cut = X(30.);
    let shift = 1.3*cut;

    let slit = D2::triangle( (0,0), (cut, gap), (cut, -gap) );
    let piece = D2::circle_r(outer_r).sub(D2::circle_r(inner_r))
            .and(D2::sector(outer_r+gap, 90).rotate(0))
            .sub( slit.clone()
                .add_map(move |x| x.translate_x(shift))
                .add_map(move |x| x.translate_x(2*shift))
                .rotate(X(360.)/64)
                .iter_rotate_equal(32)
                .union()
            )
            .sub( slit.clone()
                .translate_x(0.6*cut)
                .add_map(move |x| x.translate_x(shift))
                .add_map(move |x| x.translate_x(2*shift))
                .iter_rotate_equal(32)
                .union()
            )
            .sub( slit.clone()
                .add_map(move |x| x.translate_x(shift))
                .add_map(move |x| x.translate_x(2*shift))
                .translate_x(1.1*cut)
                .rotate(X(360.)/128)
                .iter_rotate_equal(64)
                .union()
            )
            .sub( slit.clone()
                .add_map(move |x| x.translate_x(shift))
                .add_map(move |x| x.translate_x(2*shift))
                .translate_x(2.1*cut)
                .rotate(X(360.)/256)
                .iter_rotate_equal(128)
                .union()
            )
        ;

    println!("$fn=64;\n{}", &piece);
    Ok(())
}

