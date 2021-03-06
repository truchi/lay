////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

pub use Overline::*;

/// [`Overline`](crate::Overline) (`Overlined`, `ResetOverline`).
///
/// Prints the corresponding CSI to the terminal when `Display`ed.
///
/// `Default`s to `Overline::ResetOverline`, the unsetting CSI.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Overline {
    Overlined,
    ResetOverline,
}

/// Returns `Overline::ResetOverline`.
impl Default for Overline {
    /// Returns `Overline::ResetOverline`.
    fn default() -> Self {
        Overline::ResetOverline
    }
}
