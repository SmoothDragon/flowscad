use tgdscad::*;

fn racetrack(r:f64) -> D3 {
    D2::Circle(X(0.5*r))
        .translate(XY(0., r*PI/4.))
        .iter_rotate_equal(2)
        .hull()
        .linear_extrude(X(10.))
        // .hull(D2::Circle(X(0.5*r)).translate(XY(0., -r*PI/4.)))
}

fn main() {
    println!("{}", racetrack(25.));
}
