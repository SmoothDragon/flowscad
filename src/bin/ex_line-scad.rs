use anyhow::Result;
use flowscad::*;

fn main() -> Result<()> {
    let p0 = XY(10.,0.);
    let p1 = p0.clone().rotate_deg(120);
    let p2 = p1.clone().rotate_deg(120);

    let triangle = D2::line(p0,p1,1) + D2::line(p1,p2,1) + D2::line(p2,p0,1);

    println!("{}", &triangle);
    Ok(())
}

