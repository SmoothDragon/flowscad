use std::f32::consts::PI;

use anyhow::Result;

use flowscad::*;

/* Start with vertex A on the x-axis and theta=0
 * Rotate to theta=30, where vertex A will begin curved arc
 * Rotate to theta=60, where vertex A will end curved arc
 * 
 */

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


fn shift_to_square_edges2(theta: f32) -> XY {
    let mut t = theta;
    while t > 120. {
        t -= 120.;
    }
    while t < 0. {
        t += 120.;
    }
    match t {
        t if t < 30. => XY(1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t).0, -1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t-120.).1),
        t if t < 60. => XY(-1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t+120.).0, -1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t-120.).1),
        t if t < 90. => XY(-1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t+120.).0, 1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t).1),
        t if t < 120. => XY(1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t-120.).0, 1.-XY(2.*3.0_f32.powf(-0.5),0.).rotate_deg(t).1),
        _ => XY(0.,0.),
    }
}

fn main() -> Result<()> {
    let r = X(20.);
    let h_layer = X(0.2);
    let w_nozzle = X(0.4);
    let gap = X(0.8);
    let overhang_factor = X(0.2);  // What percentage of next layer overhangs 
    let angle_increment = overhang_factor / r * w_nozzle * 180 / PI;
    let second_angle = angle_increment/2;
    let h_tree: X = 6 * r;
    let layers: i32 = (h_tree / h_layer).trunc();

    let star = D2::rectangle( (2*r, 2*w_nozzle) ).center()
        .iter_rotate_equal(3)
        .union()
        ;

    let tree_core = D2::rectangle( (2*r+2*gap+2*w_nozzle, 2*w_nozzle+2*gap+2*w_nozzle) ).center()
        .iter_rotate_equal(3)
        .union()
        ;
    let tree_hole = D2::rectangle( (2*r+2*gap, 2*w_nozzle+2*gap) ).center()
            .iter_rotate_equal(3)
            .union()
        ;


    let inner = (0..layers)
        .map(|ii| star.clone()
            .rotate(ii * angle_increment)
            .intersection(D2::circle_r(1.73*r*(layers-ii)/layers))
            .linear_extrude(h_layer)
            .translate_z(ii * h_layer)
        )
        .union()
        ;

    let tower = (0..layers/3+34)
        .map(|ii| tree_core.clone()
            .add(D2::rectangle( (2*r, 2*w_nozzle) )
                // .translate_y(-w_nozzle)
                .rotate(-60)
                .translate_x(r+w_nozzle)
                .iter_rotate_equal(6)
                .union()
            )
            .sub(tree_hole.clone())
            .rotate(ii * angle_increment)
            .intersection(D2::circle_r(1.73*r*(layers-ii)/layers))
            .linear_extrude(h_layer)
            .translate_z(ii * h_layer)
        )
        .union()
        ;
    

    let result = tower+inner;
    /*
    let h_vase = 100.;
    let factor = 0.85;
    let reuleaux = D2::circle_r(r)
        .and(D2::circle_r(r).translate( XY(3.0_f32.sqrt()/2., 0.5)*r ))
        .and(D2::circle_r(r).translate( XY(3.0_f32.sqrt()/2., -0.5)*r ))
        // .translate_x( (-0.5)*r )
        .translate_x( -3.0_f32.powf(-0.5)*r )
        .rotate(180.0)
        ;
    let limit = 400;
    let result = (0..limit)
        .map(|n| {let n = n as f32;
            reuleaux.clone()
            .rotate(n*1.2)
            .translate(shift_to_square_edges(n)*r/2.)
            .scale(1.-n/limit as f32)
            .linear_extrude(h_layer)
            .translate_z(h_layer*n)
        })
        .union()
        // .add_map(|x| x.rotate_z(180.))
        // .add_map(|x| x.rotate_z(90.))
        ;
    let result = (0..30)
        .map(|n| { let n = n as f32;
             reuleaux.clone()
            .rotate(n)
            .translate(shift_to_square_edges(n as f32)*r/2.)
            .scale((1.-n/30.)+n*factor/30.)
            .linear_extrude(h_layer)
            .translate_z(h_layer * n as f32)
        })
        .union()
        .add_map(|x| x
                 .scale3(XYZ(factor, factor, 1.0))
                 .rotate_z(-90.)
                 .translate_z(30.*h_layer)
                 )
        .add_map(|x| x
                 .scale3(XYZ(factor.powi(2), factor.powi(2), 1.0))
                 .rotate_z(180.)
                 .translate_z(60.*h_layer)
                 )
        .add_map(|x| x
                 .scale3(XYZ(factor.powi(4), factor.powi(4), 1.0))
                 .translate_z(120.*h_layer)
                 )
        .add_map(|x| x
                 .scale3(XYZ(factor.powi(8), factor.powi(8), 1.0))
                 .translate_z(240.*h_layer)
                 )
                 */
    println!("$fn=128;\n{}", &result);
    Ok(())
}

