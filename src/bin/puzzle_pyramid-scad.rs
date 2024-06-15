
use tgdscad::*;

fn main() {
    let s = 80.;  // base edge length of pyramid
    // Full Pyramid 2x2 ratio base
    let pyramid = D3::Box(XYZ(s, s, 0.01))
        .translate(XYZ(-s/2., -s/2., 0.))
        .add(D3::Box(XYZ(0.01, 0.01, 0.01))
             .translate(XYZ(0., 0., s/2.))
             )
        .hull()
        ;
    // 2x1 ratio pyramid
    let piece1 = pyramid
        .translate(XYZ(s/2., 0., 0.))
        .intersection(pyramid.clone())
        ;

    // Small pyramid 1x1 ratio
    let piece2 = piece1.translate(XYZ(-s/4., s/2., 0.)).intersection(pyramid.clone());
    // 2x1 - 1x1
    let piece3 = piece1.translate(XYZ(-s/4., s/2.-1., 1.)).difference(pyramid.clone()).translate(XYZ(0.,0.,-1.));
    // Odd shaped center piece which we will need two copies of
    let piece4 = pyramid
        .difference(piece1.clone())
        .difference(piece1.clone().translate(XYZ(-s/2.,0.,0.)))
        .intersection(D3::cube(s).translate(XYZ(-s/2.,0.,0.)))
        .rotate(XYZ(90.,0.,0.))
        ;
    // Square base to hold puzzle in
    let base = D3::Box(XYZ(s+4., s+4., 4.)).difference(D3::cube(s).translate(XYZ(2.,2.,2.)));
    // Lay all the pieces out for printing
    let puzzle = piece1
        .add(base.translate(XYZ(-s*1.2+5., -s/2., 0.))) 
        .add(piece2.translate(XYZ(0., s*0.6+5., 0.)))
        .add(piece3.translate(XYZ(-s * 0.6+5., s*0.2, 0.)))
        .add(piece4.translate(XYZ(0., -s*0.6, 0.)))
        .add(piece4.translate(XYZ(-s * 0.75+5., -s*0.6, 0.)))
        ;

    println!("{}", &puzzle);
}
