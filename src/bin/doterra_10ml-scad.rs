use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    let d_top = X(10.);
    let d_hold = X(35.);
    let gap_top_hold = d_hold/2 + d_top;
    let gap = X(0.2);
    let d_col = d_top +5.;
    let h_col = X(90.);
    let d_bottle = X(20.5);
    let h_bottle = X(83.8);
    let h_bot = h_bottle/2;
    let d_bot = d_hold + 0.7 * h_bottle;
    let h_total = h_col + gap_top_hold + d_top/2;
    let h_insert = X(60.);

    let base = D2::rectangle( (d_col/2, h_col) )
        .add(D2::circle_d(d_hold)
             .translate_y(h_col)
             )
        .add(D2::circle_d(d_top)
             .translate_y(h_col+gap_top_hold)
             )
        .add(
            D2::circle_d(d_hold + 5)
            .translate_y(h_col - 1.5*d_hold)
            .add(D2::rounded_rectangle( (0.6*d_bot, h_bot), 2))

            .hull()
            )
        .offset_radius(d_top/2)
        .offset_radius(-d_top/2)
        .intersection(D2::square(1000))
        ;
    let bottle = D2::rounded_rectangle( (d_bottle, h_bottle) , 1)
        .rotate(-40)
        .translate_x(1.2*d_col)
        .translate_y(0.707*d_bottle)
        ;
    let hole = D2::circle_d(d_top+gap)
        .add_map(|x| x.translate_y(h_insert-d_top/2))
        .hull()
        .add(D2::circle_d(d_hold+gap)
             .add_map(|x| x.translate_y(h_insert-gap_top_hold-d_top/2))
             .hull()
             )
        ;
    // let profile = base + bottle - hole;
    // let profile = profile.clone() + profile.translate_y(-h_total+h_insert);
    // let result = profile; // This show pre-rotation extrude

    let holder = (base - hole)
        .rotate_extrude(360)
        ;
    let bottle_hole = D3::cylinder_d(h_bottle, d_bottle)
        .rotate_y(45)
        .translate_x(1.8*d_col)
        .translate_z(0.707*d_bottle)
        .iter_rotate((0,0,45), 8)
        .union()
        ;
    let result = holder + bottle_hole;
    // let result = bottle_hole;
    let result = (result.clone() + result.translate_z(-h_total+h_insert))
        ;
        // .difference(D3::cube(1000))
        // ;

    println!("$fn=128;\n{}", &result);
    Ok(())
}
