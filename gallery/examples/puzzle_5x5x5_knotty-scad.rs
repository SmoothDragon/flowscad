use flowscad::*;

fn main() {
    let ll = 11.;
    let bevel = 0.75;
    let tl = X(10.);
    let gap = 0.1;
    let piece1 = D3::beveled_cube_block( (5, 1, 2), tl, bevel, gap)
        .add(D3::beveled_cube_block( (1, 4, 3), tl, bevel, gap)
            .translate(v3(4,0,0)*tl))
        .add(D3::beveled_cube_block( (4, 1, 1), tl, bevel, gap)
            .translate(v3(1,3,0)*tl))
        .add(D3::beveled_cube_block( (1, 2, 1), tl, bevel, gap)
            .translate(v3(1,2,0)*tl))
        // .translate_z(2*tl)
        // .color(ColorEnum::Red)
        ;
    let piece2 = D3::beveled_cube_block( (5, 1, 1), tl, bevel, gap)
            .translate(v3(0,0,1)*tl)
        .add(D3::beveled_cube_block( (1, 4, 2), tl, bevel, gap)
            .translate(v3(4,0,0)*tl))
        .add(D3::beveled_cube_block( (4, 1, 1), tl, bevel, gap)
            .translate(v3(1,3,1)*tl))
        .add(D3::beveled_cube_block( (1, 2, 1), tl, bevel, gap)
            .translate(v3(1,2,1)*tl))
        ;
    let piece1a = piece1.clone()
        .add(D3::beveled_cube_block( (2,1,2), tl, bevel, gap)
             .translate( v3(3,3,0)*tl))
        ;
    let piece2a = piece2.clone()
        .add(D3::beveled_cube_block( (1,1,2), tl, bevel, gap))
        .add(D3::beveled_cube_block( (1, 4, 1), tl, bevel, gap)
            .translate(v3(1,0,1)*tl))
        .translate( v3(0,1,-2)*tl)
        .rotate_x(180)
        ;
    let layout1 = piece1.clone()
        .add(piece2.clone()
            .translate( v3(0,1,-2)*tl)
            .rotate_x(180))
        .add_map(move |x| x.translate_x(6*tl))
        .add((piece1a+piece2a).translate_x(-6*tl))
        ;
    let piece_a = piece1.clone().translate_z(2*tl)
            .add(piece2.clone())
            .color(ColorEnum::Red)
            ;
    let piece_b = piece_a.clone()
        .rotate( (90,0,90) )
        .color(ColorEnum::Blue)
        ;
    let piece_c = piece_b.clone()
        .rotate( (90,0,90) )
        .color(ColorEnum::Green)
        ;

    // println!("{}", piece_a+piece_b+piece_c); // Useful for visualizing
    // println!("{}", piece1+piece2);
    println!("{}", layout1);
    // println!("{}", piece1b.rotate_x(180));
}

