use anyhow::Result;
use tgdscad::*;

fn racetrack(r:f64) -> Result<D3> {
    Ok(D2::circle(0.5*r)
        .translate(v2(0., r*PI/4.))
        .iter_rotate_equal(2)
        .hull()
        .linear_extrude(10)
        // .hull(D2::Circle(X(0.5*r)).translate(XY(0., -r*PI/4.)))
        )
}

fn main() -> Result<()> {
    println!("{}", racetrack(25.)?);
    Ok(())
}
