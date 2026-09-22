
use flowscad::*;
use anyhow::Result;


fn spherical_cow<T: Into<X>>(radius: T) -> D3 {
    let r = radius.into();
    let body = D3::sphere_r(r)
        .translate_z(r*(2.0_f64.powf(-0.5)))
        // .intersection(D3::cuboid(v3(4*r, 4*r, 1.6*r)).center().translate_z(0.8*r))
        .intersection(D3::cuboid(v3(4*r, 4*r, 2.0*r)).center().translate_z(1.0*r))
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

fn shadow_cow<T: Into<X>>(radius: T) -> D2 {
    let r = radius.into();
    let body = D2::circle_r(r*2.0_f32.powf(-0.5));
    let leg = D2::circle_r(r/4)
        .translate([0.85*r*0.5, 0.85*r*0.5*3.0_f64.powf(0.5)])
        .add_map(|x| x.mirror( (1,0) ))
        .add_map(|x| x.mirror( (0,1) ))
        ; 
    body + leg
}

fn main() -> Result<()> {
    let r=12;
    let r=16;
    let d = 32.0;
    let cow = spherical_cow(r);
    let result = (1..=6)
        .map(|ii| cow.clone()
            .sub(
            D2::Import(format!("cow-marker-{}.dxf", ii), false)
                .translate([-0.5, -0.5])
                .translate([-d/4.0, -d/4.0])
                .scale(2.0)
                .offset_radius(0.1)
                .linear_extrude(d)
            )
            .and(D3::cylinder_d(d,d))
            .translate_z(1.0)
            .add(shadow_cow(r).linear_extrude(1.))
            .translate(((ii%3-1)*40, (ii%2)*50, 0))
        )
        .union()
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}
