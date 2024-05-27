use tgdscad::*;

fn main() {
    let f = D2::Square(X(10.))
        .translate(XY(4.,5.))
        .rotate_iter(X(10.), 20)
        .hull()
        ;
    // println!("{:?}", &f);
    println!("{}", &f);
}

