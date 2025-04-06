use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let result = D3::polycube_from_bitcube3(BitCube3(0o156177), 17., 1., 0.1);

    println!("$fn=128;\n{}", &result);
    Ok(())
}

