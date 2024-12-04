use anyhow::Result;
use flowscad::*;


fn main() -> Result<()> {
    let result = circle2()
        .r(5.)
        .d(2.)
        ;
    println!("{:?}", &result);

    let result = result
        .r(50.)
        .d(10.)
        ;

    println!("{:?}", &result);
    Ok(())
}

