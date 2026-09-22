use anyhow::Result;
use flowscad::*;

fn invertible_noam_bubble(r: f32) -> D3 {
    D2::circle_r(r)
        .intersection(D2::half_plane(Aim::N))
        .add(D2::circle_r(r/3.0).translate( (r/3.0,0) ))
        .add(D2::circle_r(r/3.0).translate( (-r/3.0,0) ))
        .hull()
        .linear_extrude(8)
}

fn main() -> Result<()> {
    println!("$fn=256;\n{}", invertible_noam_bubble(80.));
    Ok(())
}

