use flowscad::*;


/// Curved design based on simple 3-piece burr puzzle.


fn main() {
    let icon = D1::line((1,1), (9,1)).to_svg(SvgProp::laser_cut());

    println!("{}", &icon);
}

