use crossterm::style::{SetBackgroundColor, SetForegroundColor};
use std::fmt::{Display, Error, Formatter};

pub use crossterm::style::Color;
pub use Color::*;

macro_rules! color {
    ($(#[$inner:ident $($args:tt)*])? $Name:ident $NoName:ident, $fmt:ident) => {
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
                write!(f, "{}", $fmt(self.0))
            }
        }
    };
}

color!(
    /// A `Foreground` `Color`.
    Foreground NoForeground,
    SetForegroundColor
);

color!(
    /// A `Background` `Color`.
    Background NoBackground,
    SetBackgroundColor
);

/// Sets `Option<Foreground>` or `Option<Background>` to `None`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct NoColor;
