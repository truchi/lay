macro_rules! ident {
    ($fmt:literal) => { ::quote::format_ident!($fmt) };
    ($fmt:expr) => { ::quote::format_ident!("{}", $fmt) };
    ($fmt:expr, $($rest:tt)*) => { ::quote::format_ident!($fmt, $($rest)*) };
}

macro_rules! doc {
    ($($arg:tt)+) => { $crate::generation::utils::Doc::new(format!($($arg)+), false) };
}

macro_rules! idoc {
    ($($arg:tt)+) => { $crate::generation::utils::Doc::new(format!($($arg)+), true) };
}

mod doc;

pub use doc::*;
