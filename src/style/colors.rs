use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};
use std::fmt::{Display, Error, Formatter};

macro_rules! color {
    ($(#[$inner:ident $($args:tt)*])? $Name:ident, $fmt:ident) => {
        $(#[$inner $($args)*])?
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub struct $Name(pub Color);

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
    Foreground,
    SetForegroundColor
);

color!(
    /// A `Background` `Color`.
    Background,
    SetBackgroundColor
);
