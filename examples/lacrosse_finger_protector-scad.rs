use flowscad::*;
use anyhow::Result;




fn main() -> Result<()> {
    let d = X(17.5);
    let l = X(35.0);
    let h = X(4.0);
    let bevel = X(1.0);

    let outer = D2::rounded_rectangle( (l,d), d/2).center().linear_extrude(h-2*bevel).translate_z(bevel);
    let inner = D2::rounded_rectangle( (l-2*bevel,d-2*bevel), d/2-bevel).center().linear_extrude(h);

    let result = (outer+inner).hull();

    println!("$fn=128;\n{}", &result);
    Ok(())
}

