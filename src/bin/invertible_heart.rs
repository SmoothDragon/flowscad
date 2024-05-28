use tgdscad::*;

fn invertible_heart(r:f32) -> D2 {
    let theta = (2.0/PI).atan()*180./PI;
    D2::Circle(X(0.5*r))
        .translate(XY(0., r*PI/4.))
        .hull(D2::Square(X(r)).translate(XY(-0.5*r, -r)))
        .rotate(X(theta))
        .intersection(D2::HalfPlane(Aim::W))
        .add_map(|x| D2::Mirror(XY(1., 0.), Box::new(x)))
}

fn main() {
    println!("{}", invertible_heart(25.));
}

