use anyhow::Result;
use flowscad::*;

fn invertible_noam_bubble(r: f32) -> D3 {
    D2::circle(r)
        .intersection(D2::half_plane(Aim::N))
        .add(D2::circle(r*0.5).translate( (r*0.5,0) ))
        .add(D2::circle(r*0.5).translate( (-r*0.5,0) ))
        .hull()
        .linear_extrude(8.)
}

fn main() -> Result<()> {
    println!("$fn=256;\n{}", invertible_noam_bubble(40.));
    Ok(())
}

