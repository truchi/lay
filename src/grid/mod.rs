// #![warn(missing_docs)]
// #![allow(unused)]

#[allow(unused)]
macro_rules! doc {
    ($($doc:expr)*, $item:item) => { doc!{ impl concat![" ", $($doc,)*], $item } };
    (impl $doc:expr, $item:item) => { #[doc = $doc] $item };
}

#[allow(unused)]
macro_rules! s { ($($tt:tt)*) => { stringify!($($tt)*) }; }

mod geo;
mod grid;
mod helpers;
mod majors;
mod num;
mod traits;

pub use geo::*;
pub use grid::*;
pub use helpers::*;
pub use majors::*;
pub use num::*;
pub use traits::*;
