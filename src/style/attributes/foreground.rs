////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

use crate::*;

/// A `Foreground` `Color`.
///
/// Prints the corresponding CSI to the terminal when `Display`ed.  
/// `Default`s to `Foreground(Color::ResetColor)`, user's default terminal's
/// foreground color.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Foreground(pub Color);

/// `Default`s to `Foreground(Color::ResetColor)`.
impl Default for Foreground {
    /// `Default`s to `Foreground(Color::ResetColor)`.
    fn default() -> Self {
        Foreground(Color::ResetColor)
    }
}

/// Returns `Foreground(Color)`.
impl From<Color> for Foreground {
    /// Returns `Foreground(Color)`.
    fn from(color: Color) -> Self {
        Foreground(color)
    }
}

/// Returns `Some(Foreground(Color))`.
impl From<Color> for Option<Foreground> {
    /// Returns `Some(Foreground(Color))`.
    fn from(color: Color) -> Self {
        Some(Foreground(color))
    }
}
