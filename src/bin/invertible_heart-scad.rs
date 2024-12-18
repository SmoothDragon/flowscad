use anyhow::Result;
use flowscad::*;

fn invertible_heart(r: X) -> Result<D2> {
    let theta = (2.0_f32/PI).atan()*180./PI;
    Ok(D2::circle_r(0.5*r)
        .translate(v2(0., r*PI/4.))
        .rotate(theta)
        .iter_rotate_equal(2)
        .hull()
        .intersection(D2::half_plane(Aim::W))
        .add_map(|x| x.mirror(v2(1., 0.)))
        )
}

fn main() -> Result<()> {
    println!("{}", invertible_heart(X(25.))?);
    Ok(())
}

