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
        ;
    // println!("{:?}", &f);
    println!("{}", &f);
    let _g = vec![D2::Circle(X(5.)), D2::Square(X(6.))];
    // let u: Vec<_> = _g.iter().map(|x| format!("{}", x)).collect();
    let u = _g.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join("\n");
    println!("{}", u);

    let v = vec!["circle(1)", "square(2)"];
    let _result = format!("union() {{\n  {}\n}}", v.join("\n  "));
    // println!("JOIN: {}", result);
}

