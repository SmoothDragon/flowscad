use flowscad::*;
use anyhow::Result;


/// Curved design based on simple 3-piece burr puzzle.

// TODO: add this as a function
fn chamfer_extrude(shape: D2) -> D3 {
    let layers = 10;
    let h_layer = X(0.1);
    let h_shape = X(10.);
    let result = (0..layers).
        map(|x| shape.clone()
            .offset_chamfer(-h_layer*x*0.7)
            .linear_extrude(h_layer)
            .translate_z(h_layer*x)
            )
        .union()
        .translate_z(h_shape/2-h_layer*layers)
        .add(shape.linear_extrude(h_shape/2-h_layer*layers))
        .add_map(|x| x.mirror((0,0,1)))
        ;
    result
}
    

fn main() -> Result<()> {
    let s = X(10.);
    let h = X(10.);
    let bevel = X(1.);
    let w = 3*h;
    let l = 3*w;
    let gap = X(0.1);
    let gap2 = gap/2;

    let piece = (D2::circle_d(3*s).translate( (-2.75*s, 3*s) )
        + ((D2::circle_r(4*s)-D2::circle_r(1.5*s)) & D2::square(4*s))
            .translate( (-2.75*s, 0.5*s) )
        + D2::rectangle( (2.5*s, s) ).center()
        )
        .add_map(|x| x.rotate(180))
        - D2::rectangle( (s+gap, 3*s+gap) ).center()
        ;
    let piece1 = chamfer_extrude(piece.clone());
    let piece2 = chamfer_extrude(piece.clone()
        - D2::rectangle( (3*s+gap, s+gap) ).center().translate_x(s)
    );


    let result = piece1
        + piece2.clone().translate_x(5*s)
        + piece2.translate_x(-5*s)
        ;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

