use anyhow::Result;
use flowscad::*;


fn main() -> Result<()> {
    let result = circle2()
        .r(5.)
        ;
    println!("{:?}", &result);

    Ok(())
}

