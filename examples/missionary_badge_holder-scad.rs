use flowscad::*;

use std::f32::consts::PI;

// Rounded rectangle with a rounded rectangle hole
fn rounded_layer_out(xy: impl Into<XY>, r: f32) -> D2 {
    let shift = 0.8; // Two 0.4mm nozzle passes
    let xy = xy.into();
    D2::rounded_rectangle(xy + 2. * shift, r + shift)
        .center()
        .sub(D2::rounded_rectangle(xy, r).center())
}

pub fn holder_stack(xy: impl Into<XY>, h: f32, r: f32) -> D3 {
    let h_layer: f32 = 0.2;
    let h_base: f32 = 1.0;
    let h_lip: f32 = 1.0;
    let n_base: usize = (h_base / h_layer).ceil() as usize;
    let n_badge: usize = (h / h_layer).ceil() as usize;
    let n_lip: usize = (h_lip / h_layer).ceil() as usize;
    let mut layers: Vec<D2> = Vec::new();
    let xy = xy.into();
    let mut xyi = xy.clone() + 1.0;

    for ii in 0..n_base {
        xyi = xyi + 0.5 * h_layer;
        layers.push(
            D2::rounded_rectangle(xyi, r)
                .translate(-0.5 * xyi)
                .sub(D2::rounded_rectangle(xy - 10, r).translate(-0.5 * (xy - 10))),
        );
    }
    for ii in 0..n_badge {
        xyi = xyi + 0.5 * h_layer;
        layers.push(
            D2::rounded_rectangle(xyi, r)
                .translate(-0.5 * xyi)
                .sub(D2::rounded_rectangle(xyi - 1.6, r).translate(-0.5 * (xyi - 1.6))), // D2::rounded_rectangle(xy + 0.5 * h_layer * ii as f32 - 1.6, r)
                                                                                         // .translate(-0.5 * (xy + 0.5 * h_layer * ii as f32 - 1.6)),
                                                                                         // ),
        );
    }
    for ii in (0..n_lip) {
        xyi = xyi - 0.45 * h_layer;
        layers.push(
            D2::rounded_rectangle(xyi, r)
                .translate(-0.5 * xyi)
                .sub(D2::rounded_rectangle(xyi - 1.6, r).translate(-0.5 * (xyi - 1.6))), // D2::rounded_rectangle(xy + h_layer * ii as f32, r)
                                                                                         // .translate(-0.5 * (xy + h_layer * ii as f32))
                                                                                         // .sub(D2::rounded_rectangle(xy - 1.6, r).translate(-0.5 * (xy - 1.6))),
        );
    }
    /*
     */
    D3::stack(layers, 0.2).translate_z(-h_base)
}

pub fn badge_fob(xy: impl Into<XY>, h: f32, r: f32) -> D3 {
    let xy = xy.into();
    D2::rounded_rectangle(xy, r)
        .translate(xy * -0.5)
        .linear_extrude(h)
}

fn main() {
    let h_badge = 45.0;
    let w_badge = 77.0;
    let thickness = 1.6;
    let thickness_with_magnet = 3.5;
    let h_magnet_from_top = 2.5;
    let r_corner = 4.0;

    let xy: XY = (w_badge, h_badge).into();
    let fob = badge_fob(xy, thickness, r_corner);
    println!("$fn=512;\n{}", fob.scad());
    // let result = holder_stack(xy, thickness, r_corner);
    // println!("$fn=512;\n{}", result.scad());
}
