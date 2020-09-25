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
    Rgb { r: u8, g: u8, b: u8 },
    AnsiValue(u8),
    Reset,
}

macro_rules! color {
    ($($(#[$inner:ident $($args:tt)*])? $Name:ident $NoName:ident ($str:literal $reset:literal))*) => {
        $(
            $(#[$inner $($args)*])?
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub struct $Name(pub Color);

            doc!("Sets `Option<" stringify!($Name) ">` to `None`.",
                #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
                pub struct $NoName;
            );

            impl From<Color> for $Name {
                fn from(color: Color) -> Self {
                    Self(color)
                }
            }

            impl From<$Name> for Color {
                fn from($Name(color): $Name) -> Self {
                    color
                }
            }

            impl Default for $Name {
                fn default() -> Self {
                    Self(Color::Reset)
                }
            }

            impl Display for $Name {
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
                            Color::Rgb { r, g, b } => write!(f, "2;{};{};{}", r, g, b),
                            Color::AnsiValue(value) => write!(f, "5;{}", value),
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
