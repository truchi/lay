////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

pub use Invert::*;

/// `Invert` (`Inverted`, `ResetInvert`).
///
/// Prints the corresponding CSI to the terminal when `Display`ed.
/// `Default`s to `Invert::ResetInvert`, the unsetting CSI.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Invert {
    Inverted,
    ResetInvert,
}

/// Returns `Invert::ResetInvert`.
impl Default for Invert {
    /// Returns `Invert::ResetInvert`.
    fn default() -> Self {
        Invert::ResetInvert
    }
}
