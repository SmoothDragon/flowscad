use flowscad::*;
use anyhow::Result;




fn main() -> Result<()> {
    let cr80_id = (X(54.0), X(85.6));  // CR80 ID card size
    let box_l = X(94.5);  // Corner square side length
    let box_w = X(64.5);  // Inside square side length
    let box_h = X(12.2);  // Height of square ring
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
        .translate_y(-hid_dia*0.35)
        ;

    let holes = rsa_mirror_outline.clone() 
        + rsa_mirror_outline.clone().translate_y(rsa_dia+wall)
        + hid_holder_outline.clone()
        ;

    let holes_notch = rsa_mirror_outline_notch.clone() 
        + rsa_mirror_outline_notch.clone().translate_y(rsa_dia+wall)
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
        .chamfer45_round(wall)
        .mirror( [0,0,1])
        ;

    let base = perimeter.clone()
        .sub(holes_notch)
        .linear_extrude(rsa_d)
        ;

    let result = perimeter_lip 
        + base
        - D2::text("CCRL".to_string())
            .scale(0.7)
            .linear_extrude(1)
            .rotate( (90,0,90) )
            .translate( (box_w/2-0.5, 20, 4) )
        ;

    println!("$fn=64;\n{}", &result);
    Ok(())
}

