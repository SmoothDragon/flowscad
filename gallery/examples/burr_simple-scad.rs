use flowscad::*;
use anyhow::Result;


/// Design to hook around the handle of a Pillar work cubicle to make it easier
/// to access a backpack with a loop strap on top.

fn main() -> Result<()> {
    let h = X(10.);
    let bevel = X(1.);
    let w: X= 3*h;
    let l: X = 3*w;
    let gap = X(0.1);
    // let gap = X(3.1);
    let gap2 = gap/2;

    let piece1 = D3::beveled_box( v3(l,h-gap2,h), bevel)
        .add(D3::beveled_box( v3(l,h-gap2,h), bevel).translate_y(2*h+gap2))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel).translate_x(2*w+gap2))
        .translate( v3(-l/2,-w/2,-h/2) )
        .color(ColorEnum::Red)
        ;

    let piece2 = D3::beveled_box( v3(l,h-gap2,h), bevel).translate_y(2*h+gap2)
        .add(D3::beveled_box( v3(w+h-gap2,h-gap2,h), bevel))
        .add(D3::beveled_box( v3(w+h-gap2,h-gap2,h), bevel).translate_x(w+2*h-gap2))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel).translate_x(2*w+gap2))
        .translate( v3(-l/2,-w/2,-h/2) )
        .color(ColorEnum::Blue)
        ;

    let piece3 = D3::beveled_box( v3(l,h-gap2,h), bevel).translate_y(2*h+gap2)
        .add(D3::beveled_box( v3(w+h-gap2,w,h), bevel))
        .add(D3::beveled_box( v3(w+h-gap2,h-gap2,h), bevel).translate_x(w+2*h-gap2))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel))
        .add(D3::beveled_box( v3(w-gap2,w,h), bevel).translate_x(2*w+gap2))
        .translate( v3(-l/2,-w/2,-h/2) )
        .color(ColorEnum::Green)
        ;

    let result = piece1
        .rotate_y(90)
        +piece2
        .rotate_z(-90)
        +piece3
        .rotate_x(-90)
        ;
    println!("$fn=128;\n{}", &result);
    Ok(())
}

