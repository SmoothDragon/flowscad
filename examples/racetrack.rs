use anyhow::Result;
use flowscad::*;

fn racetrack(r: X) -> Result<D3> {
    Ok(D2::circle_r(0.5*r)
        .translate( (0., r*PI/4.) )
        .iter_rotate_equal(2)
        .hull()
        .linear_extrude(10)
        )
}

fn main() -> Result<()> {
    println!("{}", racetrack(X(25.))?);
    Ok(())
}
