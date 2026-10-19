use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let w_wall = X(1.2);
    let h_box = X(96.0) + 4;  // height of card
    let w_box = X(62.0) + 2*w_wall + 2.0;  // width of card
    let y_box = 6*w_wall + 15 +15 +20 +20 +20;  // 5 compartments
    let d_hole = w_box - 30;

    let mybox = D3::beveled_box(v3(w_box, y_box, h_box), 0.5)
        .sub(D3::cuboid(v3(w_box, y_box, 2*h_box+w_wall) + v3(1,1,1) * -2*w_wall)
            .translate( v3(1,1,1) * w_wall)
        )
        ;

    let divider = D3::cuboid( v3(w_box, w_wall, h_box - 2*w_wall) )
        .translate_z(w_wall)
        ;


    let mybox = mybox
        + divider.clone().translate_y(15+w_wall)
        + divider.clone().translate_y(15+15+2*w_wall)
        + divider.clone().translate_y(15+15+20+3*w_wall)
        + divider.clone().translate_y(15+15+20+20+4*w_wall)
        ;

    let end_result = mybox -
        D2::circle_d(d_hole)
        .add(D2::square(d_hole).center().translate_y(h_box))
        .hull()
        .linear_extrude(2*y_box)
        .rotate_x(90)
        .translate_x(w_box/2)
        .translate_y(1.1*y_box)
        .translate_z(d_hole)
        ;
    println!("$fn=128;\n{}", &end_result);
    Ok(())
}
