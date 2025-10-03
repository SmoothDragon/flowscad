use std::fs;
use flowscad::*;


fn main() {
    let l_cube = 81.0/6.0;
    let bevel = l_cube*0.1;
    let gap = 0.2;
    let piece = D3::beveled_cube_block( (3,1,1), l_cube, bevel, gap)
        .add(D3::beveled_cube_block( (1, 1, 3), l_cube, bevel, gap))
        .add(D3::beveled_cube_block( (1, 2, 1), l_cube, bevel, gap))
        .add(
            D3::beveled_cube_block( (1, 2, 1), l_cube, bevel, gap)
            .translate(v3(2.*l_cube, 0.*l_cube, 0.))
        );
    let pieces = (0..3).map(|ii| piece.clone().translate(v3(0., 2.1*l_cube*(ii as f32), 0.))).union();
    println!("{}", pieces);
}


