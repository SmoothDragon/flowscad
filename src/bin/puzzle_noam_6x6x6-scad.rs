use std::fs;
use flowscad::*;


fn main() {
    let l_cube = 81.0/6.0;
    let bevel = l_cube*0.1;
    let gap = 0.2;
    let mut piece = Vec::new();
    piece.push(D3::beveled_cube_block( (4, 4, 4), l_cube, bevel, gap));
    piece.push(D3::beveled_cube_block( (4, 4, 2), l_cube, bevel, gap)
        .translate(v3(5.0*l_cube,0.,0.))
        );
    piece.push(D3::beveled_cube_block( (2, 5, 5), l_cube, bevel, gap)
        .add(D3::beveled_cube_block( (3, 1, 5), l_cube, bevel, gap))
        .translate(v3(0., 5.0*l_cube, 0.))
        );
    piece.push(D3::beveled_cube_block( (2, 5, 1), l_cube, bevel, gap)
        .add(D3::beveled_cube_block( (3, 1, 1), l_cube, bevel, gap))
        .translate(v3(4.*l_cube, 5.0*l_cube, 0.))
        );
    piece.push(D3::beveled_cube_block( (3, 5, 1), l_cube, bevel, gap)
        .translate(v3(8.*l_cube, 5.0*l_cube, 0.))
        );
    piece.push(D3::beveled_cube_block( (3, 4, 1), l_cube, bevel, gap)
        .translate(v3(10.*l_cube, 0., 0.))
        );
    piece.push(D3::beveled_cube_block( (2, 2, 3), l_cube, bevel, gap)
        .add(D3::beveled_cube_block( (3, 1, 3), l_cube, bevel, gap))
        .translate(v3(0.*l_cube, 11.0*l_cube, 0.))
        );
    piece.push(D3::beveled_cube_block( (1, 2, 3), l_cube, bevel, gap)
        .add(D3::beveled_cube_block( (3, 1, 3), l_cube, bevel, gap))
        .translate(v3(4.*l_cube, 11.0*l_cube, 0.))
        );

    // let result = D3::Union(Box::new(piece.clone()));
    let result = piece.clone().into_iter().union();
    println!("{}", result);

    for ii in 0..piece.len() {
        fs::write(format!("puzzle_6x6x6_{ii}.scad"), format!("{}", piece[ii])).expect("Unable to write file");
    }
}


