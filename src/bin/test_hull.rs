use tgdscad::*;

fn main() {
    let f = D2::Square(X(10.))
        .translate(4.,5.)
        .rotate_vec(10., 20)
        // .union()
        // .hull()
        ;
    // println!("{:?}", &f);
    // println!("{}", &f);
}

