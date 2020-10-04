//! Macros.

#![allow(unused_macros)]

#[macro_use]
mod doc;

#[macro_use]
mod impl_styler;
#[macro_use]
mod impl_styler_mut;
#[macro_use]
mod impl_styler_index;
#[macro_use]
mod impl_styler_index_mut;
#[macro_use]
mod impl_into_style;
#[cfg(feature = "styler-idx")]
#[macro_use]
mod impl_styler_idx;
#[cfg(feature = "styler-idx")]
#[macro_use]
mod impl_styler_mut_idx;
#[cfg(feature = "styler-ops")]
#[macro_use]
mod impl_styler_ops;
#[cfg(feature = "styler-ops")]
#[macro_use]
mod impl_styler_mut_ops;

#[macro_use]
mod impl_layer;
