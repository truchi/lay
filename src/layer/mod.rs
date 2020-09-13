use crate::Styled;

mod canvas;
mod fill;
mod layer;
mod render;

pub use canvas::*;
pub use fill::*;
pub use layer::*;
pub use render::*;

pub type Cell = Styled<char>;
