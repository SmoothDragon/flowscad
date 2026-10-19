use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let w_wall = X(1.2);
    let ir_hex = X(40.0);
    let r_corner = X(5.0);
    let h_bottom = X(2.0);
    let h_magnet = X(20.0);
    // let h_magnet = X(10.0);
    let w_magnet = X(10.0);
    let d_magnet = X(3.0);
    let gap = X(0.1);
    let h_base = h_magnet + h_bottom;
    let or_hex = ir_hex * 2.0 / 3.0_f32.sqrt();
    let r_bowl = ir_hex - d_magnet - w_wall;
    // let r_corner = (or_hex - ir_hex) / (1.0 + 2.0/3.0_f32.sqrt());
    // let r_corner = (or_hex - r_bowl) / (1.0 + 2.0/3.0_f32.sqrt());
    let h_peg = X(5.0);
    let r_peg = r_corner-2*w_wall;
    // let r_bowl_corner = h_base - h_bottom;
    let r_bowl_corner = h_base / 2;
    let peg_shift = (ir_hex - r_corner) / 0.866;
    let mag_shift = ir_hex;

    let hex = D2::circle_r(r_corner)
        .translate_x(peg_shift)
        .iter_rotate_equal(6)
        .hull()
        .linear_extrude(h_base)
        ;
    let bowl_hole = D2::rectangle( (r_bowl_corner, h_base) )
        .add(D2::circle_r(r_bowl_corner)
            .translate([r_bowl-r_bowl_corner, r_bowl_corner])
        )
        .add(D2::square(r_bowl_corner)
            .translate([r_bowl-r_bowl_corner, r_bowl-r_bowl_corner])
        )
        .hull()
        .and(D2::square(2*ir_hex))
        .translate_y(h_bottom)
        // .translate_y(2*h_base)
        .rotate_extrude(360)
        ;

    let magnet_hole = D3::cuboid((w_magnet, d_magnet, 2*h_magnet))
        .center()
        .translate((0, mag_shift-d_magnet/2-w_wall, 0))
        .iter_rotate([0,0,60], 6)
        .union()
        ;

    let hex = hex 
        - bowl_hole
        - magnet_hole
        ;

    let end_result = hex;
    println!("$fn=64;\n{}", &end_result);
    Ok(())
}
