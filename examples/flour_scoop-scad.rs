use flowscad::*;

fn main() {
    let factor = 1.0; // small
    let factor = 1.5; // small
    let d_cup = 100.*factor;
    let w_cup = 70.*factor;
    let h_cup = 100.*factor;
    let w_wall = 1.6;

    let side_fin = D3::cuboid( (0.5*d_cup, w_wall, h_cup) )
        .translate_y(-0.5*w_cup)
        .add_map(|x| x.translate_y(w_cup-w_wall))
        .and(D3::cylinder_d(h_cup, d_cup))
        ;

    let end_cap = D3::cylinder_d(w_wall, d_cup)
        .and(D3::cuboid( (0.5*d_cup, w_cup, h_cup) ).translate_y(-0.5*w_cup))
        .add_map(|x|  x.rotate_z(180).translate_x(0.3*d_cup))
        .add(D3::cuboid( (w_wall, w_cup, 0.3*h_cup) ).translate_y(-0.5*w_cup))
        ;

    let result = D3::cylinder_d(h_cup, d_cup)
        .sub(D3::cylinder_d(h_cup+2., d_cup - 2.*w_wall).translate_z(-1.))
        .and(D3::cuboid( (0.5*d_cup, w_cup, h_cup) ).translate_y(-0.5*w_cup))
        .add(side_fin)
        .add(end_cap)
        .and(D3::sphere_r(h_cup).translate_x(0.7*d_cup))
        // .add(D3::cuboid( (0.5*d_cup, w_wall, h_cup) ).translate_y(-0.5*w_cup))
        // .translate_y(d/2.)
        // .add(D3::cuboid( (d/2., d, l) ))
        // .rotate_y(90)
        ;

    println!("$fn=256;\n{}", result);
}

