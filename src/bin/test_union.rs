// extern crate itertools;

// use std::fmt;
// use itertools::Itertools;
// use std::ops::Add;
use std::cell::RefCell;

use tgdscad::*;
// use crate::*;

fn main() {
    let e = D2::Circle(X(4.));
    // let f = e.translate_iter(1.,2.,10) .collect::<Vec<_>>() ;
    // let g = D2::Union(RefCell::new(f));
    let g = e.translate_iter(1.,2.,10).sum::<D2>();

    // println!("{:?}", &f);
    println!("{}", &g);
}

