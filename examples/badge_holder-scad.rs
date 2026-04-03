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

pub fn holder_stack(xy: impl Into<XY>, r: f32) -> D3 {
    let h_layer = 0.2;
    let mut layers: Vec<D2> = Vec::new();
    let xy = xy.into();
    let mut xyi = xy.clone() + 1.0;

    for ii in 0..5 {
        xyi = xyi + 0.5 * h_layer;
        layers.push(
            D2::rounded_rectangle(xyi, r)
                .translate(-0.5 * xyi)
                .sub(D2::rounded_rectangle(xy - 10, r).translate(-0.5 * (xy - 10))),
        );
    }
    for ii in 5..(5 + 8) {
        xyi = xyi + 0.5 * h_layer;
        layers.push(
            D2::rounded_rectangle(xyi, r)
                .translate(-0.5 * xyi)
                .sub(D2::rounded_rectangle(xyi - 1.6, r).translate(-0.5 * (xyi - 1.6)))
                    // D2::rounded_rectangle(xy + 0.5 * h_layer * ii as f32 - 1.6, r)
                        // .translate(-0.5 * (xy + 0.5 * h_layer * ii as f32 - 1.6)),
                // ),
        );
    }
    for ii in (0..2) {
        xyi = xyi - 0.5 * h_layer;
        layers.push(
            D2::rounded_rectangle(xyi, r)
                .translate(-0.5 * xyi)
                .sub(D2::rounded_rectangle(xyi - 1.6, r).translate(-0.5 * (xyi - 1.6)))
            // D2::rounded_rectangle(xy + h_layer * ii as f32, r)
                // .translate(-0.5 * (xy + h_layer * ii as f32))
                // .sub(D2::rounded_rectangle(xy - 1.6, r).translate(-0.5 * (xy - 1.6))),
        );
    }
    /*
     */
    D3::stack(layers, 0.2)
}

fn main() {
    let h_layer = 0.2;
    let w: f32 = 54.0;
    let h: f32 = 85.6;
    let xy: XY = (w, h).into();
    //let base = D2::rounded_rectangle(xy, 3.0);
    let result = holder_stack(xy, 3.0);
    println!("$fn=512;\n{}", result.scad());
}
