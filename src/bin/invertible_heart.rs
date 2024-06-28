use anyhow::Result;
use tgdscad::*;

fn invertible_heart(r:f64) -> Result<D2> {
    let theta = (2.0/PI).atan()*180./PI;
    Ok(D2::circle(0.5*r)
        .translate(v2(0., r*PI/4.))
        .rotate(theta)
        .iter_rotate_equal(2)
        .hull()
        .intersection(D2::HalfPlane(Aim::W))
        .add_map(|x| x.mirror(v2(1., 0.)))
        )
}

fn main() -> Result<()> {
    println!("{}", invertible_heart(25.)?);
    Ok(())
}

