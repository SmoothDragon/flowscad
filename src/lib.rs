//! # flowscad
//!
//! Build 2D and 3D geometry with a functional, chainable API and render to
//! [OpenSCAD](https://openscad.org/) source or to SVG.
//!
//! The three top-level types are:
//!
//! - [`D1`] — 1D line segments and paths, primarily used for SVG output.
//! - [`D2`] — 2D shapes (circles, squares, polygons, faces), combinable with
//!   `add`, `sub`, `hull`, `minkowski`, and friends, extrudable to 3D.
//! - [`D3`] — 3D solids (cuboids, cylinders, spheres, polyhedra), plus the
//!   same set of combinators and transforms.
//!
//! Values of these types render to strings via [`Display`](std::fmt::Display),
//! producing OpenSCAD or SVG source that you typically pipe to disk:
//!
//! ```no_run
//! use flowscad::*;
//!
//! let ring = D2::circle_d(40).sub(D2::circle_d(30)).linear_extrude(5);
//! println!("$fn=128;\n{}", ring);
//! ```
//!
//! See the [`examples/`](https://github.com/SmoothDragon/rust_scad/tree/main/examples)
//! directory in the repository for runnable demonstrations.

mod scad1d;
pub use crate::scad1d::*;

mod scad2d;
pub use crate::scad2d::*;

mod scad3d;
pub use crate::scad3d::*;

mod cartesian;
pub use crate::cartesian::*;

mod common;
pub use crate::common::*;
pub use crate::common::PI;
pub use crate::common::ScadParameter::*;

mod convex_hull;
pub use crate::convex_hull::*;

mod d2;
pub use crate::d2::*;

mod d3;
pub use crate::d3::*;

use ::bitperm::*;
