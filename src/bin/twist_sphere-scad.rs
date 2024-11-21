use flowscad::*;
use anyhow::Result;

fn cutter(height: X) -> D3 {
    D2::sector(100, 45)
        .rotate(-22.5)
        .translate_x(0.2)
        .intersection(D2::circle_r(50))
        .iter_cyclic(4)
        .union()
        .linear_extrude_extra(height, 90.0, 100)
        .center()
}

fn main() -> Result<()> {
    let h = X(74.0);
    let wall = X(4.0);
    let w_twist = X(0.5);
    let gap = X(0.0);

    let result = D3::sphere_d(h)
        .difference(D3::sphere_d(h-2*wall))
        .difference(cutter(h*0.707))
        .intersection(
            D3::cube(h)
            .center()
            .translate_z((1.0-0.707)/2.0*h)
            )
        ;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

