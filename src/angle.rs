//! Angle types. Each one converts to and from the others and supports the same arithmetic and
//! trigonometry; only the size of a full turn differs.

mod ops;

mod rad;
pub use rad::Rad;

mod deg;
pub use deg::Deg;

mod grad;
pub use grad::Grad;

mod tau;
pub use tau::Tau;
