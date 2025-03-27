
use flowscad::*;
use anyhow::Result;


fn spherical_cow<T: Into<X>>(radius: T) -> D3 {
    let r = radius.into();
    let body = D3::sphere_r(r)
        .translate_z(r*(2.0_f64.powf(-0.5)))
        .intersection(D3::cube(4*r).center().translate_z(2*r))
        ;
    let leg = D3::sphere_r(r/4)
        .translate(0.85*r*v3(0.5, 0.5*3.0_f64.powf(0.5), 2.0_f64.powf(-0.5)))
        .add_map(|x| x.translate_z(-r))
        .hull()
        .add_map(|x| x.mirror( (1,0,0) ))
        .add_map(|x| x.mirror( (0,1,0) ))
        .intersection(D3::cube(4*r).center().translate_z(2*r))
        ; 
    let head = D3::sphere_r(r/2)
        .add(D3::sphere_r(r/8)
            .intersection(D3::cube(4*r).center().translate_z(2*r))
            .rotate_y(100)
            // .scale_x(0.5)
            .translate(r/4*v3(1, 1, 1.8))
            .add_map(|x| x.mirror( (1,0,0) ))
            )
        .translate(r*v3(0, 0.8, 0.75))
        ;
    body + leg + head
}

fn main() -> Result<()> {
    let r=12;
    let cow = spherical_cow(r);
    let result = (1..=6)
        .map(|x| cow.clone()
             .difference((D2::text(x.to_string()))
                 .linear_extrude(5)
                 .translate( (0, 0, 1.5*r as f32) )
                 )
             .translate(((x%3-1)*40, (x%2)*30, 0))
             )
        .union()
        ;
    println!("$fn=64;\n{}", &result);
    Ok(())
}
