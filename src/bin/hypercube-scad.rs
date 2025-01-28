use flowscad::*;
use anyhow::Result;


/// Start with a Reuleaux triangle with vertices at the cubic roots of unity.
/// Given a rotation angle between 0 and 30, returns the XY shift necessary to keep 1 and w^2 on
/// the square edges.
/// With a rotation of theta, 1 will move inwards and need to be shifted to the right.
/// Likewise w^2 will move upwards and need to be shifted down.
/// These two
fn shift_to_square_edges(theta: f32) -> XY {
    let mut t = theta;
    while t > 120. {
        t -= 120.;
    }
    while t < 0. {
        t += 120.;
    }
    match t {
        t if t <= 30. => XY(1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t).0, -1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t-120.).1),
        t if t <= 60. => XY(-1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t+120.).0, -1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t-120.).1),
        t if t <= 90. => XY(-1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t+120.).0, 1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t).1),
        t if t <= 120. => XY(1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t-120.).0, 1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t).1),
        _ => XY(0.,0.),
    }
}

fn main() -> Result<()> {
    let r = X(100.);
    let h_layer = 0.2;
    let h_vase = 100.;
    let layer = 2.828 * h_layer;
    let middle = D3::cube(layer);
    let planar = D3::cuboid((layer, r, layer)).center()
        .translate( (0.5*r, 0., 0.5*r) )
        .add(middle)
        .hull()
        .iter_rotate([0,0,90], 4)
        .union()
        .iter_rotate([90,0,0], 4)
        .union()
        .add(D3::cube(0.333*r).center())
        .rotate(v3(45, -90.+2.0_f64.powf(0.5).atan()*180.0/PI, 0))
        .intersection(D3::cube(2.*r).center().translate_z(0.5*r))
        // .add_map(|x| x.rotate(180.))
        // .rotate(-45.)
        // .linear_extrude(2.*h_layer)
        // .translate_z(-h_layer)
        // .rotate_y(45)
        // .add_map(|x| x.rotate_z(90.))
        ;
    let result = planar.clone()
        // .add(planar.clone().rotate_z(90))
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

