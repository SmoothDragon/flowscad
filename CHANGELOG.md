# Changelog

All notable changes to `flowscad` are documented in this file. This project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2026-09-21

First release since the initial 0.1.2. Substantial expansion, some breaking
API changes.

### Added

- `D3::polyhedron` primitive for arbitrary 3D meshes.
- `D3::polytroc_from_bittroc4` and `D3::polycube_from_bitcube3` / `_bitcube4`
  builders for bit-pattern-driven puzzle geometry.
- Extended `D2Trait` with additional transformation and combinator methods
  (`translate_x`, `translate_y`, `iter_rotate`, `iter_translate`, `hull`,
  `minkowski`, ...).
- `D3` gained a full trait surface with `.rotate_x/y/z()`, `.scale`, `.iter_*`
  combinators, and `linear_extrude` support for iterators.
- 2D `Face` and `Path` types for building parametric curves (hypocycloids,
  circles, arbitrary point sequences) and lofting them into 3D via
  `polygon_stack`.
- A `gallery/` sub-package containing ~180 personal designs built with
  flowscad, used as a compile-checked showcase.

### Changed

- **Breaking:** `D2::Import(filename, center)` now takes an owned `String`
  instead of `&'static str`, so filenames can be constructed at runtime
  (e.g. via `format!()`).
- `impl From<BitCube3>`, `From<BitCube4>`, and `From<BitTroc4>` for `D3`
  produce sensible defaults; explicit builders are available for control
  over edge length, bevel, and gap.

### Fixed

- Ambiguous `PI` and `MAX2` imports resolved.
- 15 library warnings from `rustc` cleaned up (unused imports, dead code).
- Duplicate `MAX2` constant between `scad1d` and `scad2d` removed.

### Housekeeping (not user-visible)

- Repository URL in `Cargo.toml` corrected.
- Stale `exclude = ["/src/bin"]` removed.
- Examples directory reduced from 195 loose files to 12 curated demos; the
  remaining 183 personal-model files moved to `gallery/` where they no longer
  slow down `cargo build --examples`.

## [0.1.2] - 2024-05-18

Initial published release.
