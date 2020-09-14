use crossterm::style::Attribute;
use std::fmt::{Display, Error, Formatter};

macro_rules! attribute {
    (
        $(#[$inner:ident $($args:tt)*])?
        $Name:ident:
            $($variant:ident($xvariant:ident))* + $reset:ident($xreset:ident)
    ) => {
        $(#[$inner $($args)*])?
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub enum $Name {
            $($variant,)*
            $reset
        }
        pub use $Name::*;

        impl Default for $Name {
            fn default() -> Self {
                Self::$reset
            }
        }

        impl Display for $Name {
            fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                match self {
                    $($Name::$variant => write!(f, "{}", Attribute::$xvariant),)*
                    $Name::$reset => write!(f, "{}", Attribute::$xreset)
                }
            }
        }
    };
}

attribute!(
    /// `Weighted` text.
    Weighted: Bold(Bold) Light(Dim) + ResetWeight(NormalIntensity)
);

attribute!(
    /// `Slanted` text.
    Slanted: Italic(Italic) + ResetSlant(NoItalic)
);

attribute!(
    /// `Blinking` text.
    Blinking: Slow(SlowBlink) Fast(RapidBlink) + ResetBlink(NoBlink)
);

attribute!(
    /// `Inverted` text.
    Inverted: Invert(Reverse) + ResetInvert(NoReverse)
);

attribute!(
    /// `Striked` text.
    Striked: Strike(CrossedOut) + ResetStrike(NotCrossedOut)
);

attribute!(
    /// `Underlined` text.
    Underlined: Underline(Underlined) + ResetUnderline(NoUnderline)
);

attribute!(
    /// `Overlined` text.
    Overlined: Overline(OverLined) + ResetOverline(NotOverLined)
);

attribute!(
    /// `Bordered` text.
    Bordered: Frame(Framed) Circle(Encircled) + ResetBorder(NotFramedOrEncircled)
);
