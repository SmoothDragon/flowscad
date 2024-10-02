
use flowscad::*;

fn main() {
    let d_top_bar = X(23.0);
    let w_top_bar = 35.0;
    let h_band = 5.0;
    let d_top_outer = d_top_bar + 2.0*h_band;
    let d_vert_bar = 29.0;
    let h_post_drop_hook: X = 65.0 - d_top_outer/2.0;
    let h_post_drop: X = 115.0 - d_top_outer/2.0;
    let h_post_hole_center: X = 104.0 - d_top_outer/2.0;
    let h_post_hole = 10.0;
    let bevel = 10.0;
    let h_catch = 25.0;

    let catch = D3::cuboid(v3(h_band, w_top_bar, h_catch-bevel))
        .translate(v3(-0.5*h_band, -0.5*w_top_bar, 0))
        .add(
            D3::cuboid( (h_band, w_top_bar-2.0*bevel, h_catch) )
            .translate(v3(-0.5*h_band, -0.5*(w_top_bar-2.0*bevel), 0))
        )
        .hull()
        ;

    let holder = D3::cylinder_d(w_top_bar, d_top_bar+2.0*h_band).center() // Outer top bar
        .rotate_x(90)
        .add( // Outer vertical
            D3::cylinder_d(h_post_drop, d_top_bar+2.0*h_band).center()
            .translate_z(-h_post_drop/2.0)
            )
        .add( // horizontal extension of vertical bar
            D3::cuboid(v3(d_top_outer-5.0, w_top_bar, h_post_drop)).center()
            .translate_z(-h_post_drop/2.0)
            )
        .add(catch.clone() // Front bag catch sloping upwards
             .rotate_y(45)
             .translate(v3(d_top_bar/2.0,0,5))
             )
        .add(catch.clone() // Back bag catch sloping upwards
             .rotate_y(-45)
             .translate(v3(-d_top_bar/2.0,0,5))
             )
        .add(catch // Front bag catch sloping downwards to hold backpack loop
             .rotate_y(120)
             .translate(v3(d_top_bar/2.0,0,-h_post_drop_hook))
             )
        .difference(D3::cylinder_d(h_post_drop+1.0, d_vert_bar).center()
            .translate_z(-h_post_drop/2.0)
            )
        .difference(
            D3::cylinder_d(w_top_bar+1.0, d_top_bar).center()
            .rotate_x(90)
            )
        .difference(
            D3::cylinder_d(d_top_bar+3.0*h_band, h_post_hole).center()
            .rotate_y(90)
            .translate_z(-h_post_hole_center)
            )
        .difference(
            D3::cuboid(v3(d_top_bar-5.0, w_top_bar+1.0, 2.0*h_post_drop)).center()
            .translate_z(-h_post_drop)
            )
        .rotate_x(-90)
        ;

    println!("$fn=256;\n{}", &holder);
}
