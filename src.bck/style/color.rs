pub use Color::*;

/// A `Color` for `Foreground` & `Background`.
///
/// To be used with [`Foreground`][foreground] and [`Background`][background] (a
/// `Color` on its own does not `impl Display`).
///
/// Defaults to `ResetColor`.
///
/// [foreground]: struct.Foreground.html
/// [background]: struct.Background.html
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
        Self::ResetColor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        assert_eq!(Color::default(), ResetColor);
    }
}
