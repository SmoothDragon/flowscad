// extern crate itertools;

// use std::fmt;
// use itertools::Itertools;
// use std::ops::Add;

use tgdscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let e = D2::circle(4.)?;
    let f = D2::circle(10.)?
        .add(e)
        .translate(v2(4.,5.))
        .scale_xy(v2(3.,2.))
        .add(D2::square(9)?)
        .scale(4)?
        .minkowski(D2::Rectangle(v2(0.5,1.5)))
        ;
    // println!("{:?}", &f);
    println!("{}", &f);
    let _g = vec![D2::circle(5)?, D2::square(6)?];
    // let u: Vec<_> = _g.iter().map(|x| format!("{}", x)).collect();
    let u = _g.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join("\n");
    println!("{}", u);

    let v = vec!["circle(1)", "square(2)"];
    let _result = format!("union() {{\n  {}\n}}", v.join("\n  "));
    // println!("JOIN: {}", result);
    Ok(())
}

