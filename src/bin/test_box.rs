// extern crate itertools;

// use std::fmt;
// use itertools::Itertools;
// use std::ops::Add;

use tgdscad::*;

fn main() {
    let f = D3::Cube(X(10.));
    let f = D3::Box(XYZ(10., 20., 30.));
    let f = D3::BeveledBox(XYZ(10., 20., 30.), X(1.));
    println!("{}", &f);
}
