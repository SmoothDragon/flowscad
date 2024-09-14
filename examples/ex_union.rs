use anyhow::Result;
use flowscad::*;

fn main() -> Result<()> {
    let e = D2::circle_d(4);
    let g = e.iter_translate(v2(1.,2.),10).union().add(D2::square(9));

    println!("{}", &g);
    Ok(())
}

