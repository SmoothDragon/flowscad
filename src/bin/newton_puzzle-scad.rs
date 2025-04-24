use flowscad::*;
use anyhow::Result;


fn main() -> Result<()> {
    let d_base = X(50.);
    let h_base = X(50.);
    let w_wall = X(10.);
    let air_gap = X(1.);
    let outer = D2::chamfered_circle_r(w_wall/2.)
        .translate_x(d_base/2.-w_wall/2.)
        .add(D2::square(w_wall).translate_y(-w_wall/2.))
        .add(D2::triangle( (0,0), (0,w_wall), (w_wall,0)).translate( (d_base/2.-w_wall, h_base-w_wall/4.) ))
        .hull()
        .map(|x| x.clone() - x.translate( (-w_wall, w_wall) ))
        .rotate_extrude(360)
        ;

    let r_inner = d_base/2.-w_wall-air_gap;
    let inner = D2::chamfered_circle_r(w_wall/2.)
        .translate_x(r_inner-w_wall/2.)
        .add(D2::square(w_wall).translate_y(-w_wall/2.))
        .add(D2::triangle( (0,0), (0,r_inner), (r_inner,0)).translate( (0, h_base-w_wall/4.) ))
        .hull()
        // .translate_y(w_wall+air_gap)
        .rotate_extrude(360)
        .translate_x(d_base)
        ;

    let result = outer + inner;

    println!("$fn=128;\n{}", &result);
    Ok(())
}

