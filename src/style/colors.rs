use super::{Background, Foreground};

pub use Color::*;

/// A `Color`.
///
/// To be used with [`Foreground`][foreground] and [`Background`][background] (a
/// `Color` on its own does not `impl Display`).
///
/// Defaults to `Reset`.
///
/// [foreground]: struct.Foreground.html
/// [background]: struct.Background.html
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Color {
    Black,
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
    White,
    Grey,
    Rgb(u8, u8, u8),
    Ansi(u8),
    Reset,
}

/// Returns `Color::Reset`.
impl Default for Color {
    /// Returns `Color::Reset`.
    fn default() -> Self {
        Self::Reset
    }
}

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

/// Sets `Option<Foreground>` or `Option<Background>` to `None`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NoColor;

macro_rules! colors {
    ($($(#[$meta:meta])* $Name:ident ($str:literal $reset_str:literal))*) => {
        $(
            $(#[$meta])*
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub struct $Name(pub Color);

            doc!("Returns `" stringify!($Name) "(Color::Reset)`.",
            impl Default for $Name {
                doc!("Returns `" stringify!($Name) "(Color::Reset)`.",
                fn default() -> Self {
                    Self(Color::Reset)
                });
            });

            doc!("Returns `" stringify!($Name) "(color)`.",
            impl From<Color> for $Name {
                doc!("Returns `" stringify!($Name) "(color)`.",
                fn from(color: Color) -> Self {
                    Self(color)
                });
            });

            /// Returns the inner `Color`.
            impl From<$Name> for Color {
                /// Returns the inner `Color`.
                fn from($Name(color): $Name) -> Self {
                    color
                }
            }

            impl Into<Option<$Name>> for Color {
                fn into(self) -> Option<$Name> {
                    Some($Name(self))
                }
            }

            /// Prints the corresponding CSI to the terminal.
            impl Display for $Name {
                /// Prints the corresponding CSI to the terminal.
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    let color = self.0;

                    f.write_str("\x1B[")?;

                    if color == Color::Reset {
                        f.write_str($reset_str)?;
                    } else {
                        f.write_str($str)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_and_default() {
        // Color defaults to Reset
        assert_eq!(Color::default(), Reset);

        // (u8, u8, u8) -> Rgb(u8, u8, u8)
        assert_eq!(Color::from((1, 2, 3)), Rgb(1, 2, 3));

        // u8 -> Ansi(u8)
        assert_eq!(Color::from(1), Ansi(1));

        // Foreground/Background default to Foreground/Background(Reset)
        assert_eq!(Foreground::default(), Foreground(Reset));
        assert_eq!(Background::default(), Background(Reset));

        // Foreground/Background <-> Color
        assert_eq!(Color::from(Foreground(Blue)), Blue);
        assert_eq!(Foreground::from(Green), Foreground(Green));
        assert_eq!(Color::from(Background(Red)), Red);
        assert_eq!(Background::from(Magenta), Background(Magenta));

        // Foreground <-> Background
        assert_eq!(Foreground::from(Background(Black)), Foreground(Black));
        assert_eq!(Background::from(Foreground(White)), Background(White));
    }
}
