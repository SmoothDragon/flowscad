// extern crate itertools;

// use std::fmt;
// use itertools::Itertools;
// use std::ops::Add;

use anyhow::Result;
use flowscad::*;
// use crate::*;

fn main() -> Result<()> {
    let e = D2::circle(4);
    // let f = e.iter_translate(1.,2.,10) .collect::<Vec<_>>() ;
    // let g = D2::Union(RefCell::new(f));
    // let g = e.iter_translate(XY(1.,2.),10).sum::<D2>();
    let g = e.iter_translate(v2(1.,2.),10).union().add(D2::square(9));

    // println!("{:?}", &f);
    println!("{}", &g);
    Ok(())
}

