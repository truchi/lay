// #![warn(missing_docs)]
// #![allow(unused)]

#[allow(unused)]
macro_rules! doc {
    ($($doc:expr)*, $item:item) => { doc!{ impl concat![" ", $($doc,)*], $item } };
    (impl $doc:expr, $item:item) => { #[doc = $doc] $item };
}

mod coords;
mod grid;
// mod index;
mod number;

pub use coords::*;
pub use grid::*;
// pub use index::*;
pub use number::*;
