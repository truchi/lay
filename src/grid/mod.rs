#![warn(missing_docs)]

#[allow(unused)]
macro_rules! doc {
    ($($doc:expr)*, $item:item) => { doc!{ impl concat![" ", $($doc,)*], $item } };
    (impl $doc:expr, $item:item) => { #[doc = $doc] $item };
}

mod coord;
mod grid;
mod number;
mod size;

pub use coord::*;
pub use grid::*;
pub use number::*;
pub use size::*;
