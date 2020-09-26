use std::fmt::{Display, Error, Formatter};

macro_rules! attribute {
    ($(
        $(#[$inner:ident $($args:tt)*])*
        $Name:ident:
            $($variant:ident($str:literal))* + $reset:ident($reset_str:literal),
        $NoName:ident
    )*) => {
        $(
            doc!("Sets `Option<" stringify!($Name) ">` to `None`.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
            pub struct $NoName;);

            pub use $Name::*;

            $(#[$inner $($args)*])*
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum $Name {
                $($variant,)*
                $reset
            }

            doc!("Returns `" stringify!($Name) "::" stringify!($reset) "`.",
            impl Default for $Name {
                doc!("Returns `" stringify!($Name) "::" stringify!($reset) "`.",
                fn default() -> Self {
                    Self::$reset
                });
            });

            /// Prints the corresponding CSI to the terminal.
            impl Display for $Name {
                /// Prints the corresponding CSI to the terminal.
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    f.write_str("\x1B[")?;

                    match self {
                        $($Name::$variant => f.write_str($str),)*
                        $Name::$reset => f.write_str($reset_str)
                    }?;

                    f.write_str("m")
                }
            }
        )*
    };
}

attribute!(
    /// `Weighted` text (`Bold`, `Light`, `ResetBold`).
    ///
    /// Prints the corresponding CSI to the terminal when `Display`ed.
    ///
    /// `Default`s to `ResetWeight`, the weight unsetting CSI.
    Weighted: Bold("1") Light("2") + ResetWeight("22"), NoWeight
    /// `Slanted` text (`Italic`, `ResetSlant`).
    ///
    /// Prints the corresponding CSI to the terminal when `Display`ed.
    ///
    /// `Default`s to `ResetSlant`, the slant unsetting CSI.
    Slanted: Italic("3") + ResetSlant("23"), NoSlant
    /// `Blinking` text (`Slow`, `Fast`, `ResetBlink`).
    ///
    /// Prints the corresponding CSI to the terminal when `Display`ed.
    ///
    /// `Default`s to `ResetBlink`, the blink unsetting CSI.
    Blinking: Slow("5") Fast("6") + ResetBlink("25"), NoBlink
    /// `Inverted` text (`Invert`, `ResetInvert`).
    ///
    /// Prints the corresponding CSI to the terminal when `Display`ed.
    ///
    /// `Default`s to `ResetInvert`, the invert unsetting CSI.
    Inverted: Invert("7") + ResetInvert("27"), NoInvert
    /// `Striked` text (`Strike`, `ResetStrike`).
    ///
    /// Prints the corresponding CSI to the terminal when `Display`ed.
    ///
    /// `Default`s to `ResetStrike`, the strike unsetting CSI.
    Striked: Strike("9") + ResetStrike("29"), NoStrike
    /// `Underlined` text (`Underline`, `ResetUnderline`).
    ///
    /// Prints the corresponding CSI to the terminal when `Display`ed.
    ///
    /// `Default`s to `ResetUnderline`, the underline unsetting CSI.
    Underlined: Underline("4") + ResetUnderline("24"), NoUnderline
    /// `Overlined` text (`Overline`, `ResetOverline`).
    ///
    /// Prints the corresponding CSI to the terminal when `Display`ed.
    ///
    /// `Default`s to `ResetOverline`, the overline unsetting CSI.
    Overlined: Overline("53") + ResetOverline("55"), NoOverline
    /// `Bordered` text (`Frame`, `Circle`, `ResetBorder`).
    ///
    /// Prints the corresponding CSI to the terminal when `Display`ed.
    ///
    /// `Default`s to `ResetBorder`, the border unsetting CSI.
    Bordered: Frame("51") Circle("52") + ResetBorder("54"), NoBorder
);
