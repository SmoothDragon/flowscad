use flowscad::*;
use anyhow::Result;


/// Design to hook around the handle of a Pillar work cubicle to make it easier
/// to access a backpack with a loop strap on top.

fn main() -> Result<()> {
    let h = X(10.);
    let bevel = X(1.);
    let w = 3*h;
    let l = 3*w;
    let gap = X(0.1);
    let gap2 = gap/2;

    let piece1 = D3::beveled_box( v3(l,h-gap2,h), bevel)
        .add(D3::beveled_box( v3(l,h-gap2,h), bevel).translate_y(2*h+gap2))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel).translate_x(2*w+gap2))
        .translate_y(3*w)
        ;

    let piece2 = D3::beveled_box( v3(l,h-gap2,h), bevel).translate_y(2*h+gap2)
        .add(D3::beveled_box( v3(w+h-gap2,h-gap2,h), bevel))
        .add(D3::beveled_box( v3(w+h-gap2,h-gap2,h), bevel).translate_x(w+2*h-gap2))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel).translate_x(2*w+gap2))
        .translate_y(1.5*w)
        ;

    let piece3 = D3::beveled_box( v3(l,h-gap2,h), bevel).translate_y(2*h+gap2)
        .add(D3::beveled_box( v3(w+h-gap2,w,h), bevel))
        .add(D3::beveled_box( v3(w+h-gap2,h-gap2,h), bevel).translate_x(w+2*h-gap2))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel).translate_x(2*w+gap2))
        ;

    let result = piece1+piece2+piece3;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

