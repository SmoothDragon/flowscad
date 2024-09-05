
use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let gear = D2::from_svg("rust.svg").scale(0.5);
    let layers = 10;
    let h_layer = X(0.1);
    let h_gear = X(10.);
    let result = (0..layers).
        map(|x| gear.clone()
            .offset_radius(-h_layer*x)
            .linear_extrude(h_layer)
            .translate_z(h_layer*x)
            )
        .union()
        .translate_z(h_gear/2-h_layer*layers)
        .add(gear.linear_extrude(h_gear/2-h_layer*layers))
        .add_map(|x| x.mirror((0,0,1)))
        ;
    
    println!("{}", &result);
    Ok(())
}

