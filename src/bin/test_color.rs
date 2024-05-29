// extern crate itertools;

// use std::fmt;
// use itertools::Itertools;
// use std::ops::Add;

use tgdscad::*;

fn main() {
    let e = D2::Circle(X(4.));
    let f = D2::Circle(X(10.))
        .add(e)
        .translate(XY(4.,5.))
        .scale_xy(XY(3.,2.))
        .add(D2::Square(X(9.)))
        .scale(X(4.))
        .minkowski(D2::Rectangle(XY(0.5,1.5)))
        .color(Color::Red)
        ;
    println!("{}", &f);
}

