use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {

    let w_wall_inner = X(0.5);
    let ir_hex = X(50.0);
    let r_corner = X(5.0);
    let h_bottom = X(2.0);
    let h_top = X(0.0);

    let h_magnet = X(19.0);
    let w_magnet = X(9.2);
    let d_magnet = X(2.6);
    let hole_mag = D3::cuboid( (w_magnet, d_magnet, h_magnet) );

    let x_wall = h_magnet+10;
    let y_wall = h_magnet+10;

    let w_wall = d_magnet+1;

    let gap = X(0.1);

    let h_base = w_magnet-0.2;
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

    let corner = D2::circle_r(r_corner);
    let base = corner.clone().translate( [x_wall/2-r_corner, y_wall/2-r_corner] )
        .add(corner.clone().translate( [x_wall/2-r_corner, -y_wall/2+r_corner] ))
        .add(corner.clone().translate( [-x_wall/2+r_corner, y_wall/2-r_corner] ))
        .add(corner.clone().translate( [-x_wall/2+r_corner, -y_wall/2+r_corner] ))
        .hull()
        .linear_extrude(h_base)
        ;

    let corner_hole = D3::sphere_r(r_bowl_corner)
        .and(D3::cuboid(v3(1,1,1) * 2*r_bowl_corner).center().translate_z(1));
    let bowl_hole = 
        corner_hole.clone().translate( v3((x_wall/2-r_bowl_corner-w_wall), (y_wall/2-r_bowl_corner-w_wall), r_bowl_corner+ h_bottom ) )
        .add(corner_hole.clone().translate( v3((x_wall/2-r_bowl_corner-w_wall), -(y_wall/2-r_bowl_corner-w_wall), r_bowl_corner+ h_bottom) ))
        .add(corner_hole.clone().translate( v3(r_bowl_corner+w_wall/2, (y_wall/2-r_bowl_corner-w_wall), r_bowl_corner+ h_bottom) ))
        .add(corner_hole.clone().translate( v3(r_bowl_corner+w_wall/2, -(y_wall/2-r_bowl_corner-w_wall), r_bowl_corner+ h_bottom) ))
        .add_map(|x| x.translate_z(h_base))
        .hull()
        .add_map(|x| x.translate_x(-x_wall/2+w_wall/2))
        ;

    let d_magnet = d_magnet +0.4;
    let magnet_hole = D3::cuboid((h_magnet, d_magnet, 4*w_magnet)).center();
    let magnet_holes = magnet_hole.clone().translate_y(y_wall/2-d_magnet/2-w_wall_inner)
        .add(magnet_hole.clone().translate_y(-(y_wall/2-d_magnet/2-w_wall_inner)))
        .add(magnet_hole.clone().rotate([0,0,90])
            .translate_x((x_wall/2-d_magnet/2-w_wall_inner)))
        .add(magnet_hole.clone().rotate([0,0,90])
            .translate_x(-(x_wall/2-d_magnet/2-w_wall_inner)))
        ;



    let end_result = base
        // - bowl_hole
        - magnet_holes
        ;
    // let end_result = hex + lid.translate_y(2.1*ir_hex);
    // let end_result = lid;
    println!("$fn=128;\n{}", &end_result);
    Ok(())
}
