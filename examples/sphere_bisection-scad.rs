use flowscad::*;
use anyhow::Result;

fn cutter(height: X) -> D3 {
    D2::sector(100, 45)
        .rotate(-22.5)
        .translate_x(0.2)
        .rotate(22.5)
        .intersection(D2::circle_r(50))
        .iter_cyclic(4)
        .union()
        .linear_extrude_extra(height, 0.0, 100)
        .center()
}

fn main() -> Result<()> {
    let h = X(74.0);
    let wall = X(4.0);

    let result = D3::sphere_d(h)
        .difference(D3::sphere_d(h-2*wall))
        .difference(cutter(h*0.707))
        .intersection(
            D3::cube(h)
            .center()
            .translate_z((1.0-0.707)/2.0*h)
            )
        ;
    let result = cutter(h*0.707)
        .add_map(|x| x.rotate_z(45))
        ;

    println!("$fa=2; $fs=0.2;\n{}", &result);
    // println!("$fa=1; $fs=0.1;\n{}", &result);
    Ok(())
}

