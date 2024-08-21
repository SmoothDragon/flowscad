use std::fs;
use flowscad::*;


fn main() {
    let ll = 11.;
    let bevel = 0.75;
    let tl = 11.;
    let gap = 0.1;
    let mut piece = Vec::new();
    piece.push(D3::beveled_cube_block( (3, 2, 2), ll, bevel, gap));
    piece.push(D3::beveled_cube_block( (3, 2, 2), ll, bevel, gap)
        .translate(v3(0,3,0)*tl)
        );
    piece.push(D3::beveled_cube_block( (3, 2, 2), ll, bevel, gap)
        .translate(v3(0,6,0)*tl)
        );
    piece.push(D3::beveled_cube_block( (4, 2, 1), ll, bevel, gap)
        .translate(v3(4,0,0)*tl)
        );
    piece.push(D3::beveled_cube_block( (4, 2, 1), ll, bevel, gap)
        .translate(v3(4,3,0)*tl)
        );
    piece.push(D3::beveled_cube_block( (4, 2, 1), ll, bevel, gap)
        .translate(v3(4,6,0)*tl)
        );
    piece.push(D3::beveled_box( v3(5.5,5.5,5.25)*ll, 2)
        .difference(D3::cube(5.0*ll)
            .translate( v3(0.25,0.25,0.25001)*ll )
            )
        .translate( v3(-6,0,0)*ll )
        );

    let result = piece.clone().into_iter().union();
    println!("{}", result);

    for ii in 0..piece.len() {
        fs::write(format!("puzzle_6x6x6_{ii}.scad"), format!("{}", piece[ii])).expect("Unable to write file");
    }
}


