macro_rules! ident {
    ($fmt:literal) => { ::quote::format_ident!($fmt) };
    ($fmt:expr) => { ::quote::format_ident!("{}", $fmt) };
    ($fmt:expr, $($rest:tt)*) => { ::quote::format_ident!($fmt, $($rest)*) };
}

#[macro_use]
mod doc;
mod ident;
mod write;

pub use doc::*;
pub use ident::*;
pub use write::*;
