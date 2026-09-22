use flowscad::*;

fn main() -> () {
    let w_extrusion = 30.;
    let r_corner = 2.;
    let w_center_square = 2.*5.8;
    let d_center_hole = 6.8;
    let w_cross_x = 2.2;
    let outer_inset = 4.07;
    let inner_inset = 8.25;

    let r_square = D2::square(r_corner);
    let cutter = D2::square(w_center_square);
    let result = D2::circle_r(r_corner).intersection(r_square.clone())
        .add(r_square.clone().translate_x(-0.5*w_extrusion+r_corner+outer_inset))
        .add(r_square.clone().translate_y(-0.5*w_extrusion+r_corner+outer_inset))
        .hull()
        .rotate(180)
        .sub(cutter.clone().translate_x(0.5*w_extrusion-r_corner-inner_inset))
        .sub(cutter.clone().translate_y(0.5*w_extrusion-r_corner-inner_inset))
        .translate((-0.5*w_extrusion+r_corner,-0.5*w_extrusion+r_corner))
        .add(D2::rectangle((w_extrusion, w_cross_x)).center().rotate(45))
        .iter_rotate_equal(4)
        .union()
        .add(D2::square(w_center_square).center())
        .sub(D2::circle_d(d_center_hole))
        .linear_extrude(50)
    ;
    println!("$fn=128;\n{}", &result);
}

