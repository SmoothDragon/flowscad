
use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let result = D2::from_svg("rust.svg")
        .scale(2);
    println!("{}", &result);
    Ok(())
}

