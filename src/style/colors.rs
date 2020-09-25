use std::fmt::{Display, Error, Formatter};

pub use Color::*;

/// A `Color`.
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

/// Returns `Color::Rgb(r, g, b)`.
impl From<(u8, u8, u8)> for Color {
    /// Returns `Color::Rgb(r, g, b)`.
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::Rgb(r, g, b)
    }
}

/// Returns `Color::Ansi(ansi)`.
impl From<u8> for Color {
    /// Returns `Color::Ansi(ansi)`.
    fn from(ansi: u8) -> Self {
        Self::Ansi(ansi)
    }
}

/// Returns `Color::Reset`.
impl Default for Color {
    /// Returns `Color::Reset`.
    fn default() -> Self {
        Self::Reset
    }
}

macro_rules! color {
    ($($(#[$inner:ident $($args:tt)*])? $Name:ident $NoName:ident ($str:literal $reset:literal))*) => {
        $(
            $(#[$inner $($args)*])?
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub struct $Name(pub Color);

            doc!("Sets `Option<" stringify!($Name) ">` to `None`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub struct $NoName;);

            doc!("Returns `" stringify!($Name) "(color)`.",
            impl From<Color> for $Name {
                doc!("Returns `" stringify!($Name) "(color)`.",
                fn from(color: Color) -> Self {
                    Self(color)
                });
            });

            doc!("Returns `" stringify!($Name) "(Color::Rgb(r, g, b))`.",
            impl From<(u8, u8, u8)> for $Name {
                doc!("Returns `" stringify!($Name) "(Color::Rgb(r, g, b))`.",
                fn from((r, g, b): (u8, u8, u8)) -> Self {
                    Self(Color::Rgb(r, g, b))
                });
            });

            doc!("Returns `" stringify!($Name) "(Color::Ansi(ansi))`.",
            impl From<u8> for $Name {
                doc!("Returns `" stringify!($Name) "(Color::Ansi(ansi))`.",
                fn from(ansi: u8) -> Self {
                    Self(Color::Ansi(ansi))
                });
            });

            /// Returns the inner `Color`.
            impl From<$Name> for Color {
                /// Returns the inner `Color`.
                fn from($Name(color): $Name) -> Self {
                    color
                }
            }

            doc!("Returns `" stringify!($Name) "(Color::Reset)`.",
            impl Default for $Name {
                doc!("Returns `" stringify!($Name) "(Color::Reset)`.",
                fn default() -> Self {
                    Self(Color::Reset)
                });
            });

            /// Prints the corresponding CSI to the terminal.
            impl Display for $Name {
                /// Prints the corresponding CSI to the terminal.
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    let color = self.0;

                    f.write_str("\x1B[")?;

                    if color == Color::Reset {
                        f.write_str($reset)?;
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

color!(
    /// A `Foreground` `Color`.
    Foreground NoForeground ("38;" "39")
    /// A `Background` `Color`.
    Background NoBackground ("48;" "49")
);

/// Sets `Option<Foreground>` or `Option<Background>` to `None`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NoColor;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_and_default() {
        assert_eq!(Color::from((1, 2, 3)), Color::Rgb(1, 2, 3));
        assert_eq!(Color::from(1), Color::Ansi(1));
        assert_eq!(Color::default(), Color::Reset);

        assert_eq!(Color::from(Foreground(Blue)), Blue);
        assert_eq!(Color::from(Background(Red)), Red);
        assert_eq!(Foreground::from(Green), Foreground(Green));
        assert_eq!(Background::from(Magenta), Background(Magenta));
        assert_eq!(Foreground::default(), Foreground(Reset));
        assert_eq!(Background::default(), Background(Reset));
    }
}
