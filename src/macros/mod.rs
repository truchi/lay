//! Macros.

#![allow(unused_macros)]

#[macro_use]
mod doc;
#[macro_use]
mod impl_into_style;
#[macro_use]
mod impl_layer;
#[macro_use]
mod impl_styler;
#[cfg(feature = "styler-ops")]
#[macro_use]
mod impl_styler_ops;
