use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let w_wall = X(0.5);
    let w_wall_inner = X(0.5);
    let ir_hex = X(50.0);
    let r_corner = X(5.0);
    let h_bottom = X(2.0);
    let h_top = X(0.0);

    let h_magnet = X(19.0);
    let w_magnet = X(9.2);
    let d_magnet = X(2.6);
    let hole_mag = D3::cuboid( (w_magnet, d_magnet, h_magnet) );

    let gap = X(0.1);
    let h_base = h_magnet + h_top;
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
    let chamfer = X(0.6);
    let h_lid = X(2.8);

    let hex = D2::circle_r(r_corner)
        .translate_x(peg_shift)
        .iter_rotate_equal(6)
        .hull()
        .linear_extrude(h_base)
        ;

    let bowl_hole = D3::sphere_r(r_bowl_corner)
        .translate_x( (ir_hex -r_bowl_corner-w_wall-d_magnet-w_wall_inner)/0.866 )
        .translate_z( r_bowl_corner + h_bottom )
        .iter_rotate([0,0,60], 6)
        .union()
        .add_map(|x| x.translate_z(h_base))
        .hull()
        ;

    let bowl_divider = D3::cuboid( (2*(or_hex-r_corner), 2, h_base) )
        .center()
        .translate_z(h_base/2)
        ;

    let bowl_divider2 = D3::cuboid( (2, 2*(ir_hex), h_base) )
        .center()
        .translate_z(h_base/2)
        ;

    let bowl_divider3 = D3::cuboid( (2, ir_hex, h_base) )
        .translate_x(-1)
        .iter_rotate([0,0,120], 3)
        .union()
        ;

    let magnet_hole = D3::cuboid((w_magnet, d_magnet, 2*h_magnet))
        .center()
        .translate((0, mag_shift-d_magnet/2-w_wall, 0))
        .iter_rotate([0,0,60], 6)
        .union()
        ;

    let lid_hole = D3::cuboid((6.2, 1.8, 2*h_lid))
        .center()
        .translate((0, ir_hex-0.9-w_wall, h_lid))
        .iter_rotate([0,0,60], 6)
        .union()
        ;

    let lid = D2::circle_r(r_corner)
        .translate_x(peg_shift)
        .iter_rotate_equal(6)
        .hull()
        .linear_extrude(h_lid)
        // .add(D3::chamfer_cylinder_d(2*h_lid, 2*(ir_hex-w_wall-d_magnet-w_wall_inner), chamfer)
            // .sub(D3::cylinder_d(10*h_lid, 2*(ir_hex-w_wall-d_magnet-w_wall_inner-5)))
            // )
        - (lid_hole)
        ;


    let hex = hex 
        - bowl_hole
        // + bowl_divider
        + bowl_divider3
        - magnet_hole
        ;

    let end_result = hex;
    // let end_result = hex + lid.translate_y(2.1*ir_hex);
    // let end_result = lid;
    println!("$fn=128;\n{}", &end_result);
    Ok(())
}
