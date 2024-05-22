use tgdscad::*;

// CONST PI = 3.1414926;
/*
fn racetrack(r:f32) -> D2 {
    D2::Circle(r)
        .translate(r*PI, 0)
        .add(D2::Circle(r)
}
*/
fn main() {
    let e = D2::Circle(X(4.));
    let _f = D2::Circle(X(10.))
        .add(e)
        .translate(XY(4.,5.))
        .scale_xy(XY(3., 2.))
        .add(D2::Square(X(9.)))
        .scale(4.)
        .minkowski(D2::Rectangle(XY(0.5,1.5)))
        ;


}

/*
if __name__ == '__main__':
    fn = 256
    r = 25
    piece = sd.circle(r=r)
    piece = sd.hull()(piece, sd.translate([r*np.pi,0])(piece))
    piece = sd.linear_extrude(10)(piece)
    final = sd.scad_render(piece, file_header=f'$fn={fn};')
    print(final)
*/
