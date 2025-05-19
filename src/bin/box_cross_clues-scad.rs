use flowscad::*;


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
    let wall = X(2.);
    let w_break = X(2.5);
    // Coord cards 8.5mm
    // Carboard tiles 22mm
    // Word cards 17.5mm
    // Compartments of 25, 20, 10 with two 2.5mm break walls

    let w_outer = w_card + card_gap + 2* wall;
    let w_inner = w_card + card_gap;
    let result = D3::beveled_box( (w_outer, w_outer, 0.8*w_outer), 1.5)
        .sub(D3::beveled_box( (w_inner, w_inner, w_outer), 1.5)
            .translate( (wall, wall, wall) )
        )
        .add(D3::beveled_box( (w_outer, w_break, 0.8*w_outer), 1.5)
            .translate_y(20+wall)
        )
        .add(D3::beveled_box( (w_outer, w_break, 0.8*w_outer), 1.5)
            .translate_y(30+2*wall)
        )
        ;
    println!("{}", result);

}


