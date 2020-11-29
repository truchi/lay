mod canvas;
mod cell;
mod fill;
mod gen;
mod layer;
mod view;

pub use canvas::*;
pub use cell::*;
pub use fill::*;
pub use layer::*;
pub use view::*;

/// A `(x, y)` [`Position`](crate::Position).
pub type Position = (usize, usize);

/// A `(width, height)` [`Size`](crate::Size).
pub type Size = (usize, usize);

/// A `(Position, Size)` [`Rect`](crate::Rect).
pub type Rect = (Position, Size);
