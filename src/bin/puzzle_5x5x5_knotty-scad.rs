use flowscad::*;

fn main() {
    let ll = 11.;
    let bevel = 0.75;
    let tl = X(10.);
    let gap = 0.1;
    let piece1a = D3::beveled_cube_block( (5, 1, 2), tl, bevel, gap)
        .add(D3::beveled_cube_block( (1, 4, 3), tl, bevel, gap)
            .translate(v3(4,0,0)*tl))
        .add(D3::beveled_cube_block( (4, 1, 1), tl, bevel, gap)
            .translate(v3(1,3,0)*tl))
        .add(D3::beveled_cube_block( (1, 2, 1), tl, bevel, gap)
            .translate(v3(1,2,0)*tl))
        .translate_z(2*tl)
        .color(ColorEnum::Red)
        ;
    let piece1b = D3::beveled_cube_block( (5, 1, 1), tl, bevel, gap)
            .translate(v3(0,0,1)*tl)
        .add(D3::beveled_cube_block( (1, 4, 2), tl, bevel, gap)
            .translate(v3(4,0,0)*tl))
        .add(D3::beveled_cube_block( (4, 1, 1), tl, bevel, gap)
            .translate(v3(1,3,1)*tl))
        .add(D3::beveled_cube_block( (1, 2, 1), tl, bevel, gap)
            .translate(v3(1,2,1)*tl))
        .color(ColorEnum::Red)
        ;
    let piece1 = piece1a.clone() + piece1b.clone();
    let piece2 = piece1.clone()
        .rotate( (90,0,90) )
        .color(ColorEnum::Blue)
        ;
    let piece3 = piece2.clone()
        .rotate( (90,0,90) )
        .color(ColorEnum::Green)
        ;

    // println!("{}", piece1+piece2+piece3); // Useful for visualizing
    // println!("{}", piece1+piece2);
    println!("{}", piece1a);
    // println!("{}", piece1b.rotate_x(180));
}

