use flowscad::*;
use bitperm::*;


fn holding_cube_frame<IL: Into<X>, IW: Into<X>, IB: Into<X>, IG: Into<X>>
    (il_cube: IL, iw_wall: IW, ibevel: IB, igap: IG) -> D3 {
    let bevel = ibevel.into();
    let l = il_cube.into();
    let w = iw_wall.into();
    let gap = igap.into();
    let base = D3::beveled_box( v3(1,1,0)*(l+2*w+2*gap) + v3(0,0,w), bevel).center()
        .translate(-(l/2+w+gap)*v3(1,1,0))
        ;
    let corner = D3::beveled_box( v3(w,2*w,l+2*w+2*gap), bevel)
        .add(D3::beveled_box( v3(w,2*w,l+2*w+2*gap), bevel))
        .add(D3::beveled_box( v3(2*w,w,l+2*w+2*gap), bevel))
        .add(D3::beveled_box( v3(l+2*w+2*gap,w,2*w), bevel))
        .add(D3::beveled_box( v3(l+2*w+2*gap,w,2*w), bevel)
             .translate( v3(0,0,l+2*gap) )
             )
        .add(D3::beveled_box( v3(0.707*(l+2*w+2*gap),w,2*w), bevel)
             .translate( v3(-w,0,-w))
             .rotate((0,-45,0))
             .add_map(move|x| x.mirror( (1,0,0) ))
             .add_map(move|x| x.mirror( (0,0,1) ).translate((0,0,l-w)))
             .translate( v3((l+2*gap)/2+w,0,1.414*w ))
             )
        .translate(-(l/2+w+gap)*v3(1,1,0))
        .iter_rotate((0,0,90),4)
        .union()
        ;
    base+corner
}

fn main() {
    let w_card = X(60.);
    let w_deck = X(50.);
    let card_gap = X(1.);
    let wall = X(4.);
    let w_break = X(2.5);
    let bevel = X(1.5);
    let gap = X(0.2);
    // Coord cards 8.5mm
    // Carboard tiles 22mm
    // Word cards 17.5mm
    // Compartments of 25, 20, 10 with two 2.5mm break walls

    let w_outer = w_card + card_gap + 2*wall;
    let w_inner = w_card + card_gap;
    let h_base = w_outer;
    let h_lid = X(4.);
    let h_lip = X(5.);
    let h_notch = X(5.);
    let h_front_lip = h_lip + h_notch;
    let h_lock = X(0.2);

    let lid = D3::beveled_box( (w_outer, w_outer, 2*h_front_lip), bevel)
        .sub( D3::cuboid( (3*w_outer, w_outer, h_front_lip) ) 
            .translate( (-w_outer, wall+bevel, h_lip) )
            )
        .and( D3::cuboid( (w_outer, w_outer, h_front_lip+h_lock) ) )
        .sub(D3::beveled_box( (w_inner, w_inner, 2*h_base), bevel)
            .translate( (wall, wall, h_lip) )
            )
        .translate_x(-w_outer-wall)
        .add( D3::cuboid( (w_outer-wall, w_outer-wall-gap, h_notch/3) )
            .translate( (wall/2, 0, h_base-h_lip) )
            .add(D3::cuboid( (w_outer-2*wall-2*gap, w_outer-wall-gap, 2*h_notch/3) )
                .translate( (wall+gap, 0, h_base-h_lip) )
            )
            .hull()
            .add(D3::cuboid( (w_outer-2*wall-2*gap, w_outer-wall-gap, h_notch) )
                .translate( (wall+gap, 0, h_base-h_lip) )
            )
            .rotate_y(180)
            .translate_z(h_base+h_lip)
            .translate_x(-wall)
            )
        ;

    let base = D3::beveled_box( (w_outer, w_outer, 2*h_base), bevel)
        .sub(D3::beveled_box( (w_inner, w_inner, 2*h_base), bevel)
            .translate( (wall, wall, wall) )
            )
        .sub( D3::cuboid( (3*w_outer, w_outer, h_base) ) 
            .translate( (-w_outer, wall-w_outer+bevel, h_base-h_lip) )
            )
        .and( D3::cuboid( (w_outer, w_outer, h_base) ) )
        .sub( D3::cuboid( (w_outer-wall+2*gap, w_outer-wall, h_notch/3) )
            .translate( (wall/2-gap, 0, h_base-h_lip) )
            .add(D3::cuboid( (w_outer-2*wall, w_outer-wall, 2*h_notch/3) )
                .translate( (wall, 0, h_base-h_lip) )
            )
            .hull()
            )
        ;

    let result = base
        + lid
        ;

    println!("{}", result);

}


