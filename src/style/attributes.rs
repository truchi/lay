use std::fmt::{Display, Error, Formatter};

macro_rules! attribute {
    ($(
        $(#[$inner:ident $($args:tt)*])?
        $Name:ident:
            $($variant:ident($str:literal))* + $reset:ident($reset_str:literal),
        $NoName:ident
    )*) => {
        $(
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
                    f.write_str("\x1B[")?;

                    match self {
                        $($Name::$variant => f.write_str($str),)*
                        $Name::$reset => f.write_str($reset_str)
                    }?;

                    f.write_str("m")
                }
            }

            doc!("Sets `Option<" stringify!($Name) ">` to `None`.",
                #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
                pub struct $NoName;
            );
        )*
    };
}

attribute!(
    /// `Weighted` text.
    Weighted: Bold("1") Light("2") + ResetWeight("22"), NoWeight
    /// `Slanted` text.
    Slanted: Italic("3") + ResetSlant("23"), NoSlant
    /// `Blinking` text.
    Blinking: Slow("5") Fast("6") + ResetBlink("25"), NoBlink
    /// `Inverted` text.
    Inverted: Invert("7") + ResetInvert("27"), NoInvert
    /// `Striked` text.
    Striked: Strike("9") + ResetStrike("29"), NoStrike
    /// `Underlined` text.
    Underlined: Underline("4") + ResetUnderline("24"), NoUnderline
    /// `Overlined` text.
    Overlined: Overline("53") + ResetOverline("55"), NoOverline
    /// `Bordered` text.
    Bordered: Frame("51") Circle("52") + ResetBorder("54"), NoBorder
);
