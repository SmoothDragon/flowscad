use flowscad::*;
use anyhow::Result;


/// Curved design based on simple 3-piece burr puzzle.


fn main() -> Result<()> {
    let outer_r = X(100.);   // Radius of arcs
    let inner_r = X(10.);   // Radius of arcs
    let gap = X(0.1);
    let cut = X(1.0);
    let fins = 60;

    let piece = D2::circle_r(outer_r).sub(D2::circle_r(inner_r))
            .and(D2::sector(outer_r+gap, 90).rotate(0))
            // .sub(D2::triangle( (outer_r*0.8, 0), (outer_r+gap, -cut), (outer_r+gap, cut))
                // .rotate(X(180.)/fins)
                // .iter_rotate_equal(fins)
                // .union()
            // )
            // .sub(D2::triangle( (outer_r*0.6, 0), (outer_r+gap, -cut), (outer_r+gap, cut))
                // .rotate(X(180.)/fins)
                // .iter_rotate_equal(fins/2)
                // .union()
            // )
            // Inner
            .sub( D2::triangle( (0, 0), (0.4*outer_r, -cut), (0.4*outer_r, cut))
                .rotate(X(360.)/64)
                .iter_rotate_equal(32)
                .union()
            )
            .sub( (0..96).filter(|ii| ii%3 != 0)
                .map(|ii| 
                    D2::triangle( (0, 0), (0.4*outer_r, -cut), (0.4*outer_r, cut))
                    .translate_x(0.45*outer_r)
                    .rotate((ii as f32) * 360.0/(96 as f32))
                )
                .union()
                // .rotate(X(360.)/192)
            )
            .sub( D2::triangle( (0, 0), (0.4*outer_r, -cut), (0.4*outer_r, cut))
                .translate_x(0.25*outer_r)
                .iter_rotate_equal(32)
                .union()
            )
            .sub( D2::triangle( (0, 0), (0.4*outer_r, -cut), (0.4*outer_r, cut))
                .translate_x(0.7*outer_r)
                .iter_rotate_equal(32)
                .union()
            )
            // .sub( D2::triangle( (0, 0), (0.4*outer_r, -cut), (0.4*outer_r, cut))
                // .translate_x(0.45*outer_r)
                // .iter_rotate_equal(96)
                // .union()
            // )
            // .sub( D2::triangle( (outer_r*0.7, 0), (0.9*outer_r, -cut), (0.9*outer_r, cut))
                // .rotate(X(180.)/fins)
                // .iter_rotate_equal(fins)
                // .union()
            // )

        ;

    println!("$fn=64;\n{}", &piece);
    Ok(())
}

