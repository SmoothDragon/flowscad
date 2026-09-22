
use flowscad::*;

fn main() {
    let w = 62;
    let h = 36;
    let w_bar = 5;
    let r_corner = 2;
    let h_buckle = 5;

    let buckle = D2::rounded_rectangle( (w,h), r_corner)
        .center()
        .difference(D2::rounded_rectangle( (w-2*w_bar, h-2*w_bar), r_corner).center())
        .add(D2::rectangle( (w, w_bar) ).center())
        .linear_extrude(h_buckle)
        ;

    println!("$fn=256;\n{}", &buckle);
}
