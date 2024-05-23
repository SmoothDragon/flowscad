use tgdscad::*;

fn main() {
    let f = D2::Square(X(10.))
        .translate(XY(4.,5.))
        .rotate_vec2(X(10.), 20)
        // .union()
        .hull()
        ;
    // println!("{:?}", &f);
    println!("{}", &f);
}

