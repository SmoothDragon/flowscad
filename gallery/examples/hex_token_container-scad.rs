use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let w_wall = X(1.2);
    let ir_hex = X(30.);
    let h_bottom = X(2.0);
    // let h_magnet = X(20.0);
    let h_magnet = X(10.0);
    let w_magnet = X(10.0);
    let d_magnet = X(3.0);
    let gap = X(0.1);
    let h_base = h_magnet + h_bottom;
    let or_hex = ir_hex * 2.0 / 3.0_f32.sqrt();
    let r_bowl = ir_hex - d_magnet - 2*w_wall;
    // let r_corner = (or_hex - ir_hex) / (1.0 + 2.0/3.0_f32.sqrt());
    let r_corner = (or_hex - r_bowl) / (1.0 + 2.0/3.0_f32.sqrt());
    let h_peg = X(5.0);
    let r_peg = r_corner-2*w_wall;
    let r_bowl_hole = h_base - h_bottom;
    let peg_shift = ir_hex + r_peg - w_wall;

    let hex = D2::circle_r(r_corner)
        .translate_x(peg_shift)
        .iter_rotate_equal(6)
        .hull()
        .linear_extrude(h_base)
        ;
    let peg = D3::chamfer_cylinder_d(h_base+h_peg, 2 * r_peg, 1.5)
        .translate_x(peg_shift)
        .iter_rotate([0,0,60], 6)
        .union()
        ;
    let peg_hole = D3::chamfer_cylinder_d(h_base+h_peg, 2 * (r_peg + gap), 1.5)
        .translate_x(peg_shift)
        .translate_z(-h_base+gap)
        .iter_rotate([0,0,60], 6)
        .union()
        ;
    let bowl_hole = D2::square(r_bowl_hole)
        .add(D2::circle_r(r_bowl_hole)
            .translate([r_bowl-r_bowl_hole, r_bowl_hole])
        )
        .hull()
        .and(D2::square(2*ir_hex))
        .translate_y(h_bottom)
        .rotate_extrude(360)
        ;

    let magnet_hole = D3::cuboid((w_magnet, d_magnet, 2*h_magnet))
        .center()
        .translate_y(ir_hex-w_wall)
        .iter_rotate([0,0,60], 6)
        .union()
        ;

    let hex = hex + peg - peg_hole 
        - bowl_hole
        - magnet_hole;

    let end_result = hex;
    println!("$fn=64;\n{}", &end_result);
    Ok(())
}
