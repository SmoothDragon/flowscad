use flowscad::*;
use anyhow::Result;

fn main() -> Result<()> {
    Ok(())
}

/*

if __name__ == '__main__':
    fn = 64

    r = 1.5
    ratio = .1
    n = 3  # iterations
    final = iterSmoothX(r, ratio, n)
    size = 2**n * 4 * (1+ratio) * r  # side length or iteration
    size = 10
    # final += sd.square(size, center=True)
    final = sd.linear_extrude(size)(final)
    final = sd.scad_render(final, file_header=f'$fn={fn};')
    print(final)


def smoothX(r, ratio=.25):
    '''Make a square shaped X out of circluar arcs with radius r.
    Square side length with be 6*r.
    '''
    r_inner = ratio*r
    alpha = r+r_inner
    center = sd.square(2*alpha, center=True)
    NSEW = [(alpha,0), (-alpha,0), (0,alpha), (0,-alpha)]
    center -= sd.union()(*[sd.translate(ij)(sd.circle(r_inner)) for ij in NSEW])
    corners = [(alpha,alpha), (-alpha,alpha), (alpha,-alpha), (-alpha,-alpha)]
    center += sd.union()(*[sd.translate(ij)(sd.circle(r)) for ij in corners])
    return center

def iterSmoothX(r, ratio=.25, n=1):
    base = smoothX(r, ratio)
    level = base
    for i in range(n):
        R = 2**i*2*(1+ratio)*r
        level = base + sd.union()(*[sd.translate(ij)(level) for ij in [(R,R), (-R,R), (R,-R), (-R,-R)]])
    return level

if __name__ == '__main__':
    fn = 64

    r = 1
    ratio = .1
    n = 3  # iterations
    final = iterSmoothX(r, ratio, n)
    size = 2**n * 4 * (1+ratio) * r  # side length or iteration
    size = 10 
    # final += sd.square(size, center=True)
    final = sd.linear_extrude(size)(final)
    final = sd.scad_render(final, file_header=f'$fn={fn};')
    print(final)


*/
