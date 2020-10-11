use super::*;
use std::fmt::{Display, Error, Formatter};

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

macro_rules! colors {
    ($(
        $(#[$meta_ground:meta])*
        $Ground:ident($csi:literal $reset:literal)
    )*) => {
        $(
            $(#[$meta_ground])*
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub struct $Ground(pub Color);

            doc!("Returns `" stringify!($Ground) "(Color::ResetColor)`.",
            impl Default for $Ground {
                doc!("Returns `" stringify!($Ground) "(Color::ResetColor)`.",
                fn default() -> Self {
                    Self(Color::ResetColor)
                });
            });

            doc!("Returns `" stringify!($Ground) "(color)`.",
            impl From<Color> for $Ground {
                doc!("Returns `" stringify!($Ground) "(color)`.",
                fn from(color: Color) -> Self {
                    Self(color)
                });
            });

            /// Returns the inner `Color`.
            impl From<$Ground> for Color {
                /// Returns the inner `Color`.
                fn from($Ground(color): $Ground) -> Self {
                    color
                }
            }

            doc!("Returns `Some(" stringify!($Ground) "(color))`.",
            impl From<Color> for Option<$Ground> {
                doc!("Returns `Some(" stringify!($Ground) "(color))`.",
                fn from(color: Color) -> Self {
                    Some($Ground(color))
                });
            });

            impl_styler!((__: $Ground) -> Style {
                (foreground) Style::NONE.foreground(foreground),
                (background) Style::NONE.background(background),
                (weight)     Style::NONE.weight(weight),
                (slant)      Style::NONE.slant(slant),
                (blink)      Style::NONE.blink(blink),
                (invert)     Style::NONE.invert(invert),
                (strike)     Style::NONE.strike(strike),
                (underline)  Style::NONE.underline(underline),
                (overline)   Style::NONE.overline(overline),
                (border)     Style::NONE.border(border),
            });

            #[cfg(feature = "styler-ops")]
            impl_styler_ops!(($Ground) -> Style);

            /// Prints the corresponding CSI to the terminal.
            impl Display for $Ground {
                /// Prints the corresponding CSI to the terminal.
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    let color = self.0;

                    f.write_str("\x1B[")?;

                    if color == Color::ResetColor {
                        f.write_str($reset)?;
                    } else {
                        f.write_str($csi)?;

                        match color {
                            Color::Black => f.write_str("5;0"),
                            Color::DarkGrey => f.write_str("5;8"),
                            Color::Red => f.write_str("5;9"),
                            Color::DarkRed => f.write_str("5;1"),
                            Color::Green => f.write_str("5;10"),
                            Color::DarkGreen => f.write_str("5;2"),
                            Color::Yellow => f.write_str("5;11"),
                            Color::DarkYellow => f.write_str("5;3"),
                            Color::Blue => f.write_str("5;12"),
                            Color::DarkBlue => f.write_str("5;4"),
                            Color::Magenta => f.write_str("5;13"),
                            Color::DarkMagenta => f.write_str("5;5"),
                            Color::Cyan => f.write_str("5;14"),
                            Color::DarkCyan => f.write_str("5;6"),
                            Color::White => f.write_str("5;15"),
                            Color::Grey => f.write_str("5;7"),
                            Color::Rgb(r, g, b) => write!(f, "2;{};{};{}", r, g, b),
                            Color::Ansi(ansi) => write!(f, "5;{}", ansi),
                            _ => Ok(()) // NOTE: unreachable
                        }?;
                    }

                    f.write_str("m")
                }
            }
        )*
    };
}

colors!(
    /// A `Foreground` `Color`.
    ///
    /// Prints the corresponding CSI to the terminal when `Display`ed.
    ///
    /// `Default`s to `Foreground(Color::ResetColor)`, user's default terminal foreground color.
    Foreground("38;" "39")
    /// A `Background` `Color`.
    ///
    /// Prints the corresponding CSI to the terminal when `Display`ed.
    ///
    /// `Default`s to `Background(Color::ResetColor)`, user's default terminal background color.
    Background("48;" "49")
);

/// Returns `Foreground(color)`.
impl From<Background> for Foreground {
    /// Returns `Foreground(color)`.
    fn from(Background(color): Background) -> Self {
        Self(color)
    }
}

/// Returns `Background(color)`.
impl From<Foreground> for Background {
    /// Returns `Background(color)`.
    fn from(Foreground(color): Foreground) -> Self {
        Self(color)
    }
}

impl_styler_index!(
    (foreground: Foreground) {
        Some(*foreground), None, None, None, None, None, None, None, None, None,
    }
    (background: Background) {
        None, Some(*background), None, None, None, None, None, None, None, None,
    }
);

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        // Color defaults to ResetColor
        assert_eq!(Color::default(), ResetColor);

        // Foreground/Background default to Foreground/Background(ResetColor)
        assert_eq!(Foreground::default(), Foreground(ResetColor));
        assert_eq!(Background::default(), Background(ResetColor));
    }

    #[test]
    fn conversion() {
        // Foreground/Background <-> Color
        assert_eq!(Color::from(Foreground(Blue)), Blue);
        assert_eq!(Foreground::from(Green), Foreground(Green));
        assert_eq!(Color::from(Background(Red)), Red);
        assert_eq!(Background::from(Magenta), Background(Magenta));

        // Color -> Option<Foreground/Background>
        let foreground: Option<Foreground> = Magenta.into();
        assert_eq!(foreground, Some(Foreground(Magenta)));
        let background: Option<Background> = Magenta.into();
        assert_eq!(background, Some(Background(Magenta)));

        // Foreground <-> Background
        assert_eq!(Foreground::from(Background(Black)), Foreground(Black));
        assert_eq!(Background::from(Foreground(White)), Background(White));
    }
}
