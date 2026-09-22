use flowscad::*;
use anyhow::Result;


/// Design to hook around the handle of a Pillar work cubicle to make it easier
/// to access a backpack with a loop strap on top.

fn main() -> Result<()> {
    let w_handle = X(30.);
    let d_handle = X(15.);
    let thickness = X(10.);

    let holder = D3::beveled_box( v3(w_handle+2*thickness, thickness, 3*w_handle), 2)
        .add(D3::beveled_box( v3(thickness, d_handle+2*thickness, w_handle), 2))
        .add(D3::beveled_box( v3(thickness, d_handle+2*thickness, w_handle), 2)
            .translate( (w_handle+thickness, 0, 2*w_handle)))
        .add(D3::beveled_box( v3(w_handle+2*thickness, thickness, w_handle), 2)
            .translate_y(d_handle+thickness))
        .add(D3::beveled_box( v3(w_handle+2*thickness, thickness, w_handle), 2)
            .translate((0, d_handle+thickness, 2*w_handle)))
        .add(D3::beveled_box(v3(w_handle+2*thickness, 2*thickness+w_handle, thickness), 2)
            .translate_y(d_handle+thickness))
        .add(D3::beveled_box( v3(w_handle+2*thickness, thickness, w_handle), 2)
            .translate_y(2*w_handle+thickness))
        .rotate_y(90)
        ;

    println!("$fn=128;\n{}", &holder);
    Ok(())
}

