use flowscad::*;

fn main() {
    let w_trim = 18.0;
    let h_join = 50.0;
    let w_join = 80.0;
    let w_nozzle = 0.4;
    let gap =0.01;
    let w_wall = 4.0*w_nozzle+gap;

    let join = D2::rectangle( (h_join+w_wall, w_trim+2.0*w_wall) )
        .sub(D2::rectangle( (h_join, w_trim) ).translate( (w_wall, w_wall) ))
        .translate_y(-0.5*w_trim-w_wall)
        .sub(D2::rectangle( (h_join, gap) ).translate( (4.0*w_nozzle+gap, 0.5*w_trim+2.0*w_nozzle) ))
        .sub(D2::rectangle( (h_join, gap) ).translate( (4.0*w_nozzle+gap, -0.5*w_trim-2.0*w_nozzle) ))
        .sub(D2::rectangle( (gap, w_trim) ).translate( (2.0*w_nozzle, w_nozzle) ))
        .sub(D2::rectangle( (gap, w_trim) ).translate( (2.0*w_nozzle, -w_nozzle-w_trim) ))
        .linear_extrude(w_join)
        ;
    println!("$fn=256;\n{}", join);
}

