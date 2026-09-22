use anyhow::Result;
use flowscad::*;

fn main() -> Result<()> {
    let triangle = (0..3)
        .map(|x| XY(10.,0.).rotate_deg(120*x))
        .pairs()
        .map(|(a,b)| D2::line(a,b,1))
        .union()
        ;

    println!("{}", &triangle);
    Ok(())
}

