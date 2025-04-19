use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let mut result = D3::rhombdo();
    result += result.translate((1,1,1));

    println!("$fn=128;\n{}", &result);
    Ok(())
}

