//! Terminal graphics lol.
//!
//! This crate defines styling and layering utilities to print "graphics" to the
//! terminal.
//!
//! See [`style`](style/index.html) and [`layer`](layer/index.html) modules for
//! more details.  
//! See [`examples`](https://github.com/truchi/lay/tree/master/examples) for examples.

#[macro_use]
mod macros;

mod layer;
mod style;

pub use layer::*;
pub use style::*;
