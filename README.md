# flowscad

[![Crates.io](https://img.shields.io/crates/v/flowscad.svg)](https://crates.io/crates/flowscad)
[![Docs.rs](https://docs.rs/flowscad/badge.svg)](https://docs.rs/flowscad)

A Rust library for building 2D and 3D geometry with a functional, chainable API.
Renders to [OpenSCAD](https://openscad.org/) source (for 3D printing, CAD, and STL
export) and to SVG (for laser cutting, plotting, and 2D vector graphics).

```rust
use flowscad::*;

fn main() {
    let ring = D2::circle_d(40).sub(D2::circle_d(30))
        .linear_extrude(5);
    println!("$fn=128;\n{}", ring);
}
```

Piping the output to OpenSCAD produces a solid model:

```bash
cargo run --example brio > brio.scad
openscad brio.scad          # opens the model in OpenSCAD
```

## Install

```bash
cargo add flowscad
```

Minimum Rust: 1.81.

### System dependencies (Ubuntu / Debian)

`flowscad` uses [qhull](http://www.qhull.org/) for 3D convex hulls, which needs
the C library headers at build time:

```bash
sudo apt install libclang1 clang qhull-bin
```

To render the generated `.scad` files, install OpenSCAD:

```bash
sudo apt install openscad
```

## A few more examples

2D combinator, iteration, and extrusion:

```rust
use flowscad::*;

let racetrack = D2::circle_r(12.5)
    .translate((0., 12.5 * PI / 4.0))
    .iter_rotate_equal(2)
    .hull()
    .linear_extrude(10);
```

3D primitives and boolean ops:

```rust
use flowscad::*;

let d = 6.0;
let l = 15.0;
let brio = D3::cylinder_d(l, d)
    .translate_y(d / 2.0)
    .add(D3::cuboid((d / 2.0, d, l)))
    .rotate_y(90);
```

SVG output for laser cutting:

```rust
use flowscad::*;

let icon = D1::line((1, 1), (9, 1))
    .to_svg(SvgProp::laser_cut());
```

More examples live in [`examples/`](examples/). Run any of them with, e.g.:

```bash
cargo run --example racetrack > racetrack.scad
cargo run --example icon_svg  > icon.svg
```

## The gallery

The [`gallery/`](gallery/) directory is a separate `flowscad-gallery` package
containing ~180 personal designs: puzzle pieces, badge holders, coins, laser-cut
templates, and various 3D-printed objects. It is not published to crates.io; it
exists to compile-check real-world usage of the API.

Build the gallery on its own:

```bash
cd gallery
cargo build --examples
```

## License

LGPL-3.0-or-later. See [`LICENSE`](LICENSE).
