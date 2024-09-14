use anyhow::Result;
use flowscad::*;

fn main() -> Result<()> {
    let f = D2::square(10)
        .translate(v2(4.,5.))
        .iter_rotate(10, 20)
        .hull()
        ;
    // println!("{:?}", &f);
    println!("{}", &f);
    Ok(())
}

