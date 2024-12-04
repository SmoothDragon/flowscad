use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let d = X(20.);

    let chain = D2::circle_chain(5)
        .scale(d)
        ;


    let result = chain;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

