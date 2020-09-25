use crossterm::style::Attribute;
use std::fmt::{Display, Error, Formatter};

macro_rules! attribute {
    (
        $(#[$inner:ident $($args:tt)*])?
        $Name:ident:
            $($variant:ident($xvariant:ident))* + $reset:ident($xreset:ident),
        $NoName:ident
    ) => {
        pub use $Name::*;

        $(#[$inner $($args)*])?
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub enum $Name {
            $($variant,)*
            $reset
        }

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

        doc!("Sets `Option<" stringify!($Name) ">` to `None`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub struct $NoName;
        );
    };
}

attribute!(
    /// `Weighted` text.
    Weighted: Bold(Bold) Light(Dim) + ResetWeight(NormalIntensity),
    NoWeight
);

attribute!(
    /// `Slanted` text.
    Slanted: Italic(Italic) + ResetSlant(NoItalic),
    NoSlant
);

attribute!(
    /// `Blinking` text.
    Blinking: Slow(SlowBlink) Fast(RapidBlink) + ResetBlink(NoBlink),
    NoBlink
);

attribute!(
    /// `Inverted` text.
    Inverted: Invert(Reverse) + ResetInvert(NoReverse),
    NoInvert
);

attribute!(
    /// `Striked` text.
    Striked: Strike(CrossedOut) + ResetStrike(NotCrossedOut),
    NoStrike
);

attribute!(
    /// `Underlined` text.
    Underlined: Underline(Underlined) + ResetUnderline(NoUnderline),
    NoUnderline
);

attribute!(
    /// `Overlined` text.
    Overlined: Overline(OverLined) + ResetOverline(NotOverLined),
    NoOverline
);

attribute!(
    /// `Bordered` text.
    Bordered: Frame(Framed) Circle(Encircled) + ResetBorder(NotFramedOrEncircled),
    NoBorder
);
