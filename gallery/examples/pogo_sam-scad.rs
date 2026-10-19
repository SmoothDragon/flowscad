use flowscad::*;
use anyhow::Result;


fn track(d: X, width: X) -> D3 {
    D2::circle_d(d)
        .add_map(|x| x.translate_x(width -d))
        .translate_x(-width /2)
        .linear_extrude(0.01)
}

fn main() -> Result<()> {
    let shift = 0.1;
    let x_pogo = X(17.2);
    let x_inner = X(15.0);
    let y_pogo = X(4.0);
    let w_wall = X(0.8);
    let z_notch = X(2.0);
    let h_notch = X(1.2);
    let z_pogo = X(10.0);
    let x_offset = x_pogo - y_pogo;

    let pogo1 = track(y_pogo+2*w_wall, x_pogo+2*w_wall);
    let pogo2 = track(y_pogo+2*w_wall, x_pogo+2*w_wall);
    let pogo3 = track(y_pogo+2*w_wall, x_pogo+2*w_wall-2*z_notch);
    let pogo4 = track(y_pogo+2*w_wall, x_pogo+2*w_wall);
    let pogo5 = track(y_pogo+2*w_wall, x_pogo+2*w_wall);
    let pogo6 = track(y_pogo+2*w_wall, x_pogo+2*w_wall-2*z_notch);

    let result =(pogo1 + pogo2.clone().translate_z(4)).hull()
        .add((pogo2 + pogo3.clone().translate_z(1)).hull())
        .add((pogo3 + pogo4.clone().translate_z(1)).hull())
        .add((pogo4 + pogo5.clone().translate_z(3)).hull())
        .add((pogo5 + pogo6.clone().translate_z(3)).hull())
        ;
    /*
    let pogo2 = D2::circle_d(y_pogo+2*w_wall)
        .add_map(|x| x.translate_x(x_offset))
        .hull()
        .linear_extrude(2)
        ;

    let pogo = D2::circle_d(y_pogo+2*w_wall)
        .add_map(|x| x.translate_x(x_offset))
        .hull()
        .linear_extrude(z_pogo)
        ;

    let pogo_in = D2::circle_d(y_pogo+2*w_wall)
        .add_map(|x| x.translate_x(x_offset-2*z_notch))
        .hull()
        .linear_extrude(z_pogo)
        .translate_z(y_pogo)
        .translate_x(z_notch)
        ;

    let end_result = (pogo + pogo_in).hull()
        ;
    */

    println!("$fn=128;\n{}", &result);
    Ok(())
}
