use flowscad::*;
use anyhow::Result;




fn main() -> Result<()> {
    let cr80_id = (X(54.0), X(85.6));  // CR80 ID card size
    let box_l = X(94.5);  // Corner square side length
    let box_w = X(66.0);  // Inside square side length
    let box_h = X(11.0);  // Height of square ring
    let rsa_d = X(9.4);
    let rsa_w = X(20.3);
    let rsa_l = X(58.9);
    let rsa_dia = X(27.7);
    let rsa_knob_d = X(4.0);
    let rsa_knob_offset = X(3.3);
    let badge_flap = X(1.2);
    let wall = X(2.0);
    let w = rsa_l+rsa_knob_offset + 2*wall;
    let l = rsa_dia + 2*wall;

    let hid_dia = X(45.0);
    let hid_h = X(12.0);
    let gap = 0.3;
    let rsa_gap = X(0.8);

    let id_shift = X(16.0);  // mm shifted up from center
    let id_shape = D2::rounded_rectangle( cr80_id, 1).center()
        .translate_y(id_shift)
        ;

    let rsa_holder_outline = D2::circle_d(rsa_dia).translate( (rsa_dia/2+wall+rsa_knob_offset, l/2) )
        .add(D2::rounded_rectangle((0.75*w, rsa_w), 1).translate( (0.25*w-wall, l/2-rsa_w/2) ))
        .translate_x(-w/2)
        ;

    let rsa_holder_outline_notch = rsa_holder_outline.clone()
        .add(D2::rounded_rectangle( (rsa_dia, rsa_knob_d), 0.5).translate( (wall-w/2, l/2-rsa_knob_d/2)))
        ;

    let rsa_mirror_outline = rsa_holder_outline.clone()
        .mirror([1,0])
        ;

    let rsa_mirror_outline_notch = rsa_holder_outline_notch.clone()
        .mirror([1,0])
        ;

    let hid_holder_outline = D2::circle_d(hid_dia)
        .and(D2::circle_d(3*hid_dia).translate_y(-1.15*hid_dia))
        .translate_y(-hid_dia*0.35+2)
        ;

    let holes = rsa_mirror_outline.clone() 
        + rsa_mirror_outline.clone().translate_y(rsa_dia+rsa_gap)
        + hid_holder_outline.clone()
        ;

    let holes_notch = rsa_mirror_outline_notch.clone() 
        + rsa_mirror_outline_notch.clone().translate_y(rsa_dia+rsa_gap)
        + hid_holder_outline.clone()
        ;

    let perimeter = D2::rounded_rectangle( (box_w, box_l), 3*wall)
        .center()
        .translate_y(l/2)
        // .add(D2::rounded_rectangle( (0.8*hid_dia, 1.8*hid_dia), 3*wall).center())
        .add(D2::circle_d(hid_dia+2*wall).translate_y(-0.35*hid_dia))
        // .hull()
        ;

    let perimeter_lip = perimeter.clone()
        .sub(holes.clone())
        .offset_radius(wall)
        .and(perimeter.clone())
        .add(D2::rounded_rectangle((25,11), 2).center()
            .sub(D2::rounded_rectangle((14,4), 1).center())
            .translate_y(box_l/2 + l/2 + 2.0)
        )
        .chamfer45_round(wall)
        .mirror([0,0,1])
        .add_map(|x| x.mirror([0,0,1])
            .and(D3::cuboid((2*box_w, box_l, 2*box_h))
                .translate((-box_w, 1.9*l,0))
            )
        )
        ;

    let perimeter_outline = perimeter.clone()
        .sub(id_shape.clone())
        .sub(id_shape.clone().translate_y(10))
        .sub(D2::rectangle((2*box_w, box_l)).translate((-box_w, 1.9*l)))
        ;

    let perimeter_id = perimeter_outline.clone()
        .sub(hid_holder_outline.clone())
        .linear_extrude(1.5)
        .translate_z(box_h)
        .sub(rsa_mirror_outline_notch.clone()
            .linear_extrude(20)
            .and(D3::cube(40).translate_y(-20))
            .add_map(|x| x.translate_y(rsa_dia+rsa_gap))
            )
        + perimeter_outline.clone()
        // .chamfer45(-1.5)
        .chamfer45_round(-1.5)
        .and(perimeter.clone()
            .linear_extrude(1.0)
            + perimeter.clone().chamfer45(0.5).translate_z(1.0)
        )
        .sub(D3::cuboid((2*box_w, box_l, 2*box_h))
            .rotate_x(45)
            .translate((-box_w, 1.9*l,0))
        )
        .translate_z(box_h+1.5)
        .add(D2::rounded_rectangle((8,2),1)
            .center()
            .linear_extrude(box_h+0.75)
            .translate_y(box_l/2 + l/2 - 2.0)
        )
        .sub(rsa_mirror_outline_notch.clone()
            .linear_extrude(20)
            .and(D3::cube(100).translate_y(-50))
            .add_map(|x| x.translate_y(rsa_dia+rsa_gap))
            )
        ;

    let base = perimeter.clone()
        .sub(holes_notch)
        .linear_extrude(box_h)
        ;

    let result = perimeter_lip.clone()
        + base.clone()
        + perimeter_id.clone()
        + D2::text("CCRL".to_string())
            .scale(0.9)
            .linear_extrude(0.6)
            .rotate( (90,0,90) )
            .translate( (box_w/2-0.3, 16.5, box_h/2) )
        + D2::text("L".to_string())
            .scale(0.9)
            .linear_extrude(0.6)
            .rotate( (90,0,-90) )
            .translate( (-box_w/2+0.3, 16.5, box_h/2) )
        + D2::text("H".to_string())
            .scale(0.9)
            .linear_extrude(0.6)
            .rotate( (90,0,-90) )
            .translate( (-box_w/2+0.3, 45, box_h/2) )
        ;

    // let result = perimeter_id.clone();

    println!("$fn=128;\n{}", &result);
    Ok(())
}

