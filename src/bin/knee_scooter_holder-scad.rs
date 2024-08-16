
use flowscad::*;

fn main() {
    let d_top_bar = 23.0;
    let w_top_bar = 35.0;
    let h_band = 5.0;
    let d_top_outer = d_top_bar + 2.0*h_band;
    let d_vert_bar = 29.0;
    let h_post_drop_hook = 65.0 - d_top_outer/2.0;
    let h_post_drop = 115.0 - d_top_outer/2.0;
    let h_post_hole_center = 104.0 - d_top_outer/2.0;
    let h_post_hole = 10.0;
    let bevel = 10.0;
    let h_catch = 25.0;

    let catch = D3::cuboid(v3(h_band, w_top_bar, h_catch-bevel))
        .translate(v3(-0.5*h_band, -0.5*w_top_bar, 0))
        .add(
            D3::cuboid(v3(h_band, w_top_bar-2.0*bevel, h_catch))
            .translate(v3(-0.5*h_band, -0.5*(w_top_bar-2.0*bevel), 0))
        )
        .hull()
        ;

    let holder = D3::cylinder(w_top_bar, d_top_bar+2.0*h_band).center() // Outer top bar
        .rotate(v3(90., 0., 0.))
        .add( // Outer vertical
            D3::cylinder(h_post_drop, d_top_bar+2.0*h_band).center()
            .translate(v3(0,0,-h_post_drop/2.0))
            )
        .add( // horizontal extension of vertical bar
            D3::cuboid(v3(d_top_outer-5.0, w_top_bar, h_post_drop))
            .translate(v3(-(d_top_outer-5.0)/2.0, -(w_top_bar)/2.0, -h_post_drop))
            )
        .add(catch.clone() // Front bag catch sloping upwards
             .rotate(v3(0,45,0))
             .translate(v3(d_top_bar/2.0,0,5))
             )
        .add(catch.clone() // Back bag catch sloping upwards
             .rotate(v3(0,-45,0))
             .translate(v3(-d_top_bar/2.0,0,5))
             )
        .add(catch // Front bag catch sloping downwards to hold backpack loop
             .rotate(v3(0,120,0))
             .translate(v3(d_top_bar/2.0,0,-h_post_drop_hook))
             )
        .difference(D3::cylinder(h_post_drop+1.0, d_vert_bar).center()
            .translate(v3(0,0,-h_post_drop/2.0))
            )
        .difference(
            D3::cylinder(w_top_bar+1.0, d_top_bar).center()
            .rotate(v3(90., 0., 0.))
            )
        .difference(
            D3::cylinder(d_top_bar+3.0*h_band, h_post_hole).center()
            .rotate(v3(0, 90, 0))
            .translate(v3(0,0,-h_post_hole_center))
            )
        .difference(
            D3::cuboid(v3(d_top_bar-5.0, w_top_bar+1.0, 2.0*h_post_drop)).center()
            .translate_z(-h_post_drop)
            // .translate(v3(-(d_top_bar-5.0)/2.0, -(w_top_bar+1.0)/2.0, -2.0*h_post_drop))
            )
        .difference(
            D3::cuboid(v3(d_top_bar-5.0, w_top_bar+1.0, 2.0*h_post_drop))
            .translate(v3(-(d_top_bar-5.0)/2.0, -(w_top_bar+1.0)/2.0, -2.0*h_post_drop))
            )
        .rotate(v3(-90,0,0))
        ;

    println!("$fn=256;\n{}", &holder);
}
