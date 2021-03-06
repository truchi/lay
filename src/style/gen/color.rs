////////////////////////////////////////////////////////////////////////////////
// 🚨🚨🚨🚨🚨🚨🚨🚨 This file is @generated by build script. 🚨🚨🚨🚨🚨🚨🚨🚨 //
// 🚧🚧🚧🚧🚧🚧🚧🚧           ⛔ DO NOT MODIFY! ⛔           🚧🚧🚧🚧🚧🚧🚧🚧 //
////////////////////////////////////////////////////////////////////////////////

pub use Color::*;

/// A [`Color`](crate::Color) for [`Foreground`](crate::Foreground) and
/// [`Background`](crate::Background).
///
/// To be used with [`Foreground`](crate::Foreground) and
/// [`Background`](crate::Background), as a [`Color`](crate::Color) does not
/// `Display` on its own.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    White,
    Black,
    Grey,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    Rgb(u8, u8, u8),
    Ansi(u8),
    ResetColor,
}

/// Returns `Color::ResetColor`.
impl Default for Color {
    /// Returns `Color::ResetColor`.
    fn default() -> Self {
        Color::ResetColor
    }
}
