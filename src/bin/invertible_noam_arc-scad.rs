use anyhow::Result;
use flowscad::*;

fn invertible_noam_arc<T: Into<X>>(diameter: T) -> D3 {
    let d: X = diameter.into();
    D2::circle(d)
        // .intersection(D2::half_plane(Aim::N))
        .and(D2::half_plane(Aim::N))
        .add(D2::circle(d/3).translate( (d/3,0) ))
        .add(D2::circle(d/3).translate( (-d/3,0) ))
        .sub(D2::circle(d/3))
        .linear_extrude(5)
}

fn main() -> Result<()> {
    println!("$fn=256;\n{}", invertible_noam_arc(40.));
    Ok(())
}

