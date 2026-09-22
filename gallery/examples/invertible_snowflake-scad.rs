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

fn radial_lines(n: u32, r_circle: f32, w_line: f32) -> D2 {
    D2::rectangle(XY(r_circle, w_line))
        .translate_y(-w_line/2.)
        .iter_rotate_equal(n)
        .union()
}
            

fn main() -> Result<()> {
    let r_circle = 50.;
    let h_layer: f32 = 0.2;
    let w_nozzle = 0.4;
    let h_ridge = 4.*w_nozzle;
    let n_ridge = 4*(r_circle as u32);
    let result = D2::circle_r(r_circle)
        .add(radial_lines(n_ridge, r_circle+h_ridge, 3.*w_nozzle))
        .sub(radial_lines(n_ridge, r_circle+h_ridge-w_nozzle, w_nozzle))
        .sub(D2::circle_r(r_circle-w_nozzle))
        ;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

