use std::fs;
use flowscad::*;


fn main() {
    let ll = 11.;
    let bevel = 0.75;
    let tl = 11.;
    let gap = 0.1;
    let mut piece = Vec::new();
    piece.push(D3::beveled_cube_block( (4, 4, 4), ll, bevel, gap));
    piece.push(D3::beveled_cube_block( (4, 4, 2), ll, bevel, gap)
        .translate(v3(5.0*tl,0.,0.))
        );
    piece.push(D3::beveled_cube_block( (2, 5, 5), ll, bevel, gap)
        .add(D3::beveled_cube_block( (3, 1, 5), ll, bevel, gap))
        .translate(v3(0., 5.0*tl, 0.))
        );
    piece.push(D3::beveled_cube_block( (2, 5, 1), ll, bevel, gap)
        .add(D3::beveled_cube_block( (3, 1, 1), ll, bevel, gap))
        .translate(v3(4.*tl, 5.0*tl, 0.))
        );
    piece.push(D3::beveled_cube_block( (3, 5, 1), ll, bevel, gap)
        .translate(v3(8.*tl, 5.0*tl, 0.))
        );
    piece.push(D3::beveled_cube_block( (3, 4, 1), ll, bevel, gap)
        .translate(v3(10.*tl, 0., 0.))
        );
    piece.push(D3::beveled_cube_block( (2, 2, 3), ll, bevel, gap)
        .add(D3::beveled_cube_block( (3, 1, 3), ll, bevel, gap))
        .translate(v3(0.*tl, 11.0*tl, 0.))
        );
    piece.push(D3::beveled_cube_block( (1, 2, 3), ll, bevel, gap)
        .add(D3::beveled_cube_block( (3, 1, 3), ll, bevel, gap))
        .translate(v3(4.*tl, 11.0*tl, 0.))
        );

    // let result = D3::Union(Box::new(piece.clone()));
    let result = piece.clone().into_iter().union();
    println!("{}", result);

    for ii in 0..piece.len() {
        fs::write(format!("puzzle_6x6x6_{ii}.scad"), format!("{}", piece[ii])).expect("Unable to write file");
    }
}


