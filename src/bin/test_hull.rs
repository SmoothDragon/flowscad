use tgdscad::*;

fn main() {
    let f = D2::square(10)
        .translate(XY(4.,5.))
        .iter_rotate(X(10.), 20)
        .hull()
        ;
    // println!("{:?}", &f);
    println!("{}", &f);
}

