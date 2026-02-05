use flowscad::*;

use std::f32::consts::PI;
use num_complex::Complex32;


// fn exp1j(theta: f32) -> Complex32 {
    // Complex32::new(theta.cos(), theta.sin())
// }

fn expi(theta: f32) -> C32 {
    C32::new(theta.cos(), theta.sin())
}

fn expi_deg(theta: f32) -> C32 {
    let theta = theta * PI / 180.;
    C32::new(theta.cos(), theta.sin())
}

/// Create a circular face that toggles between the inner and outer radii
pub fn alternating_circle(inner_r: f32, outer_r: f32, segments_pairs: u32) -> Face {
    let pts: u32 = 360;
    let segments = 2 * segments_pairs;
    let l_band = pts / segments;
    Face( (0..pts)
        .map(|ii| {
            let r = match ii {
                ii if ii.rem_euclid(l_band) == 0 => 0.5 * (inner_r + outer_r),
                ii if ii.rem_euclid(2 * l_band) < l_band => inner_r,
                _ => outer_r - ((ii%2) as f32)*0.25,
            };
            r * expi_deg(((ii * 360) as f32) / pts as f32)
        })
        .collect()
    )
}

fn averaged(v: Vec<Face>) -> Vec<Face> {
    v.iter().enumerate()
        .map(|(i, cur)| {
            match (i > 0, i + 1 < v.len()) {
                (true, true) => (cur + &v[i - 1] + &v[i + 1]).scaled(1. / 3.),
                (true, false) => (&v[i - 1] + cur).scaled(1. / 2.),
                (false, true) => (cur + &v[i + 1]).scaled(1. / 2.),
                (false, false) => cur.clone(), // only one element
            }
        })
        .collect()
}


pub fn yabane_stack(inner_r: f32, outer_r: f32, segments_pairs: u32) -> D3 {
    let h_layer = 0.2;
    let perimeter = outer_r * 2. * PI;
    let theta = Rad((2. * PI) / (perimeter / h_layer));
    let theta = Rad(h_layer / outer_r);
    let pattern = alternating_circle(inner_r, outer_r, 3);
    let pattern_trans = circle(0.5*(inner_r+outer_r), 360);
    let pattern_trans2 = alternating_circle(0.5*(inner_r+outer_r), 0.5*(inner_r+outer_r), 3);

    let center_layers = 8;
    let slope_layers = 25 - center_layers/2;
    let mut layers: Vec<Face> = Vec::new();
    for ii in 0..slope_layers { layers.push(pattern.rotated((slope_layers - ii) as f32 * theta)); }
    layers.push(pattern_trans.clone());
    for ii in 0..center_layers { layers.push(pattern.rotated(Deg(60.)).circular_shift(60)); }
    layers.push(pattern_trans.clone());
    for ii in 0..slope_layers { layers.push(pattern.rotated(ii as f32 * theta)); }
    // layers.push(pattern_trans.clone());
    layers.push(pattern_trans.clone().circular_shift(10));
    for ii in 0..slope_layers { layers.push(pattern.rotated(Deg(60.) + (slope_layers + ii) as f32 * theta).circular_shift(80)); }
    layers.push(pattern_trans.clone());
    for ii in 0..center_layers { layers.push(pattern.rotated(slope_layers as f32 * 2. * theta).circular_shift(20)); }
    layers.push(pattern_trans.clone());
    for ii in 0..slope_layers { layers.push(pattern.rotated(Deg(60.) + (2*slope_layers-ii) as f32 * theta).circular_shift(80)); }
    layers.push(pattern_trans.clone().circular_shift(10));
    for ii in 0..slope_layers { layers.push(pattern.rotated((slope_layers - ii) as f32 * theta)); }
    layers.push(pattern_trans.clone());
    for ii in 0..center_layers { layers.push(pattern.rotated(Deg(60.)).circular_shift(60)); }
    layers.push(pattern_trans.clone());
    for ii in 0..slope_layers { layers.push(pattern.rotated(ii as f32 * theta)); }
    let height = layers.len() as f32 * 0.2;
    polygon_stack(layers, 0.2)
        .translate_z(-height/2.)
}




fn main() {
    let h_layer = 0.2;

    let napkin_inner_r: f32 = 20.;
    let napkin_thickness = 5.;
    let detail_depth = 1.;
    let outer_r = napkin_inner_r + napkin_thickness;
    let inner_r = outer_r - detail_depth;

    let result = yabane_stack(inner_r, outer_r, 3);
    let itadakmasu = "いただきます".to_string().chars().enumerate()
        .map(|(ii, ch)| D2::Hiragana(ch.to_string()).linear_extrude(3)
            .scale(1.5)
            .rotate_x(90)
            .translate_y(napkin_inner_r+0.5)
            .and(D3::cylinder_r(100, napkin_inner_r+0.5).translate_z(-50))
            .rotate_z(-55. * ii as f32)
        )
        .union()
        ;
    let result = result
        .sub(itadakmasu)
        .sub(D3::cylinder_r(100, napkin_inner_r).translate_z(-50))
        .sub(D3::cylinder_r(100, napkin_inner_r).translate_z(-50).rotate_z(1))
        .sub(D3::cylinder_r(100, napkin_inner_r).translate_z(-50).rotate_z(1.5))
        ;

    println!("$fn=512;\n{}", result.scad());
}
