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
    let r = X(40.);
    let h_layer = 0.2;
    let h_vase = 100.;
    let reuleaux = D2::circle_r(r)
        .and(D2::circle_r(r).translate( XY(3.0_f32.sqrt()/2., 0.5)*r ))
        .and(D2::circle_r(r).translate( XY(3.0_f32.sqrt()/2., -0.5)*r ))
        // .translate_x( (-0.5)*r )
        .translate_x( -3.0_f32.powf(-0.5)*r )
        .rotate(180.0)
        ;
    let result = (0..360)
        .map(|n| reuleaux.clone()
            .rotate(n)
            .translate(shift_to_square_edges(n as f32)*r/2.)
            .linear_extrude(h_layer)
            .translate_z(h_layer * n as f32)
            )
        .union()
        // .add_map(|x| x.rotate_z(180.))
        // .add_map(|x| x.rotate_z(90.))
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

