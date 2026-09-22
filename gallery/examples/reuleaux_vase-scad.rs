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
    let e_cube = 100.;
    let h_layer: f32 = 0.2;
    let w_nozzle = 0.4;
    let n_layer = (e_cube / h_layer).round() as u32;
    let rotation = 180.;
    let reuleaux = D2::circle_r(e_cube)
        .and(D2::circle_r(e_cube).translate( XY(3.0_f32.sqrt()/2., 0.5)*e_cube ))
        .and(D2::circle_r(e_cube).translate( XY(3.0_f32.sqrt()/2., -0.5)*e_cube ))
        .translate_x( -3.0_f32.powf(-0.5)*e_cube )
        .rotate(180.0)
        ;
    let result = (0..n_layer)
        .map(|ii| {let ii = ii as f32; let theta = ii * rotation / n_layer as f32;
            reuleaux.clone()
            .rotate(theta)
            .translate(shift_to_square_edges(theta)*e_cube/2.)
            .linear_extrude(h_layer)
            .translate_z(h_layer * ii as f32)
        })
        .union()
        ;
    let spines = D2::rectangle(XY(2.*w_nozzle, e_cube))
        .translate_y(-w_nozzle)
        .iter_rotate_equal(24)
        .union()
        .intersection(D2::square(e_cube).center())
        .linear_extrude(e_cube)
        ;
    let result = result + spines;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

