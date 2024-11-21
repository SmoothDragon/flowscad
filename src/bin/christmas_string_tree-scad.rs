use flowscad::*;
use anyhow::Result;

/// Calculates continuous piecewise linear function for 0<t<11
fn tree_edge(t: f64) -> f64 {
    match t {
        t if t<0. || t>11. => 0.,
        t if t<3. => 4.-2./3.*t,
        t if t<4. => t-1.,
        t if t<7. => 3.-2./3.*(t-4.),
        t if t<8. => t-6.,
        t if t<11. => 2.-2./3.*(t-8.),
        _ => 0.,
    }
}

fn triangle(r_triangle: f64, w: f64, h: f64) -> D3 {
    D2::regular_polygon(6, w)
        .translate_y(r_triangle)
        .add_map(|x| x.rotate(120))
        .hull()
        .iter_rotate_equal(3)
        .union()
        .linear_extrude(h)
}

fn main() -> Result<()> {
    let unit = 5.;
    let layer = 0.1;
    let h_tree = 100.;
    let h_func = 11.;
    let h_base = 5.;
    let r_edge = 2.5;
    let scale = h_tree/h_func;
    let limit = (h_tree/layer) as u64;
    // hex radius needed to achieve width of 0.6mm
    let r_hex_hole = 0.6*3.0_f64.powf(-0.5);
    // hex radius needed to achieve width of 0.4mm
    let r_hex_bridge = 0.4*3.0_f64.powf(-0.5);

    let upper = (0..=limit)
        .map(|ii| ii as f64)
        .map(|ii| (tree_edge( ii/scale*layer )*scale, ii*layer))
        .map(|(y, z)| D2::regular_polygon(6, r_edge)
            .translate_y(y)
            .iter_rotate_equal(6)
            .union()
            .linear_extrude(layer+0.01)
            .translate_z(z)
            )
        .union()
        ;
    let base = triangle(tree_edge(0.)*scale, r_edge, h_base)
        .add_map(|x| x.rotate_z(60))
        .translate_z(-h_base)
        ;
    let level = triangle(tree_edge(0.5)*scale, 0.6, 0.4)
        .translate_z(0.5*scale)
        ;
    let bridges = (0..=limit)
        .filter(|ii| ii%8 < 4)
        .map(|ii| (ii as f64, if ii%16 < 8 {0} else {60}))
        .map(|(ii, theta)| triangle(tree_edge(ii/scale*layer)*scale, r_hex_bridge, layer+0.01)
             .rotate_z(theta)
             .translate_z(ii*layer)
             )
        .union()
        ;

    let holes = (0..=limit)
        .filter(|ii| ii%8 < 4)
        .map(|ii| (ii as f64, if ii%16 < 8 {0} else {60}))
        .map(|(ii, theta)| triangle(tree_edge(ii/scale*layer)*scale, r_hex_hole, layer+0.01)
             .rotate_z(theta)
             .translate_z(ii*layer)
             )
        .union()
        ;

    let result = upper + base - holes + bridges;

    println!("$fn=128;\n{}", &result);
    Ok(())
}
