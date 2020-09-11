macro_rules! attributes {
    ($(#[$inner:ident $($args:tt)*])? $Name:ident: $($variant:ident($xvariant:ident))* + $reset:ident($xreset:ident)) => {
        $(#[$inner $($args)*])?
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
        pub enum $Name {
            $($variant,)*
            $reset
        }
        pub use $Name::*;

        impl ::std::default::Default for $Name {
            fn default() -> Self {
                Self::$reset
            }
        }

        impl ::std::fmt::Display for $Name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                match self {
                    $($Name::$variant => write!(f, "{}", ::crossterm::style::Attribute::$xvariant),)*
                    $Name::$reset => write!(f, "{}", ::crossterm::style::Attribute::$xreset)
                }
            }
        }
    };
}

attributes!(
    /// `Weighted` text.
    Weighted: Bold(Bold) Light(Dim) + ResetWeight(NormalIntensity)
);
attributes!(
    /// `Slanted` text.
    Slanted: Italic(Italic) + ResetSlant(NoItalic)
);
attributes!(
    /// `Blinking` text.
    Blinking: Slow(SlowBlink) Fast(RapidBlink) + ResetBlink(NoBlink)
);
attributes!(
    /// `Inverted` text.
    Inverted: Invert(Reverse) + ResetInvert(NoReverse)
);
attributes!(
    /// `Striked` text.
    Striked: Strike(CrossedOut) + ResetStrike(NotCrossedOut)
);
attributes!(
    /// `Underlined` text.
    Underlined: Underline(Underlined) + ResetUnderline(NoUnderline)
);
attributes!(
    /// `Overlined` text.
    Overlined: Overline(OverLined) + ResetOverline(NotOverLined)
);
attributes!(
    /// `Bordered` text.
    Bordered: Frame(Framed) Circle(Encircled) + ResetBorder(NotFramedOrEncircled)
);
