use tgdscad::*;

fn invertible_heart(r:f64) -> D2 {
    let theta = (2.0/PI).atan()*180./PI;
    D2::Circle(X(0.5*r))
        .translate(XY(0., r*PI/4.))
        .rotate(X(theta))
        .iter_rotate_equal(2)
        .hull()
        .intersection(D2::HalfPlane(Aim::W))
        .add_map(|x| x.mirror(XY(1., 0.)))
}

fn main() {
    println!("{}", invertible_heart(25.));
}

