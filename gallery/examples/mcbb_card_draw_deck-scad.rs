use flowscad::*;
use anyhow::Result;

// Width of box: 290mm
// Height over placards: 59mm

fn main() -> Result<()> {
    let w_wall = X(2.0);
    let card_gap = X(1.0);
    let w_card = X(62.0);
    let h_card = X(96.0);
    let x_box: X = 2*w_card + 3*w_wall + 2*card_gap;  // width of card
    let y_box: X = h_card + 2*w_wall + 2*card_gap;  // height of card
    let z_box = X(59.0);
    let d_hole = 30;

    let mybox = D3::beveled_box(v3(x_box, y_box, z_box), 1.0)
        .sub(D3::cuboid(v3(x_box, y_box, 2*z_box) + v3(1,1,1) * -2*w_wall)
            .translate( v3(1,1,1) * w_wall)
        )
        ;

    let divider = D3::cuboid( v3(w_wall, y_box, z_box- 2*w_wall) )
        .translate_z(w_wall)
        ;


    let mybox = mybox
        .add(divider.translate_x(w_wall + X(62.0)))
        .translate( (-x_box/2, -y_box/2, w_wall) )
        ;

    let cutter = D2::circle_d(d_hole)
        .add(D2::square(d_hole).center().translate_y(z_box))
        .hull()
        .linear_extrude(2*y_box)
        // .rotate_x(90)
        // .translate_x(x_box/2)
        // .translate_y(1.1*y_box)
        // .translate_z(d_hole)
        ;
    let end_result = mybox 
        - cutter.clone().translate( (x_box/4, y_box*0.45, 0))
        - cutter.clone().translate( (-x_box/4, y_box*0.45, 0))
        - cutter.clone().rotate((0,0,180)).translate( (x_box/4, -y_box*0.45, 0))
        - cutter.clone().rotate((0,0,180)).translate( (-x_box/4, -y_box*0.45, 0))
        - cutter.clone().rotate((0,0,90)).translate( (-x_box*0.45, 0, 0))
        - cutter.clone().rotate((0,0,-90)).translate( (x_box*0.45, 0, 0))
        ;
    println!("$fn=128;\n{}", &end_result);
    Ok(())
}
