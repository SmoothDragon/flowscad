use flowscad::*;
use anyhow::Result;

fn make_shackle() -> D3 {
    let h = X(10.);
    let h_shackle = X(30.);
    let w_shackle = X(20.);
    D2::rectangle( (w_shackle, h_shackle) )
        .sub(D2::rectangle( (w_shackle, h_shackle/5) )
             .translate( (-w_shackle/3, h_shackle*0.4) )
             )
        .sub(D2::rectangle( (w_shackle, h_shackle/5) )
             .translate( (w_shackle/3, -h_shackle*0.01) )
             )
        .sub(D2::rectangle( (w_shackle/7, h_shackle) )
             .translate( (w_shackle/7, h_shackle*0.8) )
             )
        .sub(D2::rectangle( (w_shackle/7, h_shackle) )
             .translate( (w_shackle*3/7, h_shackle*0.8) )
             )
        .sub(D2::rectangle( (w_shackle/7, h_shackle) )
             .translate( (w_shackle*5/7, h_shackle*0.8) )
             )
        .translate_x(w_shackle/8)
        .add_map(|x| x.mirror( v2(1,0) ))
        .linear_extrude(h)
}

fn make_bolt() -> D3 {
    let r_toroid = X(15.);
    let toroid = D2::rectangle( (r_toroid*2/3, r_toroid/3) )
        .center()
        .sub(D2::circle_d(r_toroid/2).translate_y(-r_toroid*0.3))
        .sub(D2::circle_d(r_toroid/2).translate_y(r_toroid*0.3))
        .translate_x(r_toroid*0.75)
        .rotate_extrude(360)
        ;
    let bolt = toroid.clone().scale3( (0.7,0.7,1) )
        .add(toroid.translate_x(1.76*r_toroid))
        .iter_rotate((0,0,90), 4)
        .union();
    bolt
}

fn main() -> Result<()> {
    // let result = make_shackle();
    let result = make_bolt();

    println!("$fn=100;\n{}", &result);
    Ok(())
}
