use super::*;
use std::fmt::{Display, Error, Formatter};

macro_rules! attributes {
    ($(
        $Attr:ident {
            $reset:ident($csi_reset:literal)
            $($variant:ident($csi_variant:literal))*
        }
    )*) => {
        $(
            pub use $Attr::*;

            doc!(
            "`" stringify!($Attr) "` (`" $(stringify!($variant) "`, `")* stringify!($reset) "`).\n\n"
            "Prints the corresponding CSI to the terminal when `Display`ed.\n\n"
            "`Default`s to `" stringify!($reset) "`, the unsetting CSI.",
            #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
            pub enum $Attr {
                $($variant,)*
                $reset
            });

            doc!("Returns `" stringify!($Attr) "::" stringify!($reset) "`.",
            impl Default for $Attr {
                doc!("Returns `" stringify!($Attr) "::" stringify!($reset) "`.",
                fn default() -> Self {
                    Self::$reset
                });
            });

            impl_styler!((__: $Attr) -> Style {
                (foreground) Style::NONE.foreground(foreground),
                (background) Style::NONE.background(background),
                (weight)     Style::NONE.weight(weight),
                (slant)      Style::NONE.slant(slant),
                (blink)      Style::NONE.blink(blink),
                (invert)     Style::NONE.invert(invert),
                (strike)     Style::NONE.strike(strike),
                (underline)  Style::NONE.underline(underline),
                (overline)   Style::NONE.overline(overline),
                (border)     Style::NONE.border(border),
            });

            #[cfg(feature = "styler-ops")]
            impl_styler_ops!(($Attr) -> Style);

            /// Prints the corresponding CSI to the terminal.
            impl Display for $Attr {
                /// Prints the corresponding CSI to the terminal.
                fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
                    f.write_str("\x1B[")?;

                    match self {
                        $Attr::$reset => f.write_str($csi_reset),
                        $($Attr::$variant => f.write_str($csi_variant),)*
                    }?;

                    f.write_str("m")
                }
            }
        )*
    };
}

attributes!(
    Weight {
        ResetWeight("22")
        Bold("1")
        Light("2")
    }
    Slant {
        ResetSlant("23")
        Italic("3")
    }
    Blink {
        ResetBlink("25")
        Slow("5")
        Fast("6")
    }
    Invert {
        ResetInvert("27")
        Inverted("7")
    }
    Strike {
        ResetStrike("29")
        Striked("9")
    }
    Underline {
        ResetUnderline("24")
        Underlined("4")
    }
    Overline {
        ResetOverline("55")
        Overlined("53")
    }
    Border {
        ResetBorder("54")
        Frame("51")
        Circle("52")
    }
);

impl_styler_index!(
    (weight: Weight) {
        None, None, Some(*weight), None, None, None, None, None, None, None,
    }
    (slant: Slant) {
        None, None, None, Some(*slant), None, None, None, None, None, None,
    }
    (blink: Blink) {
        None, None, None, None, Some(*blink), None, None, None, None, None,
    }
    (invert: Invert) {
        None, None, None, None, None, Some(*invert), None, None, None, None,
    }
    (strike: Strike) {
        None, None, None, None, None, None, Some(*strike), None, None, None,
    }
    (underline: Underline) {
        None, None, None, None, None, None, None, Some(*underline), None, None,
    }
    (overline: Overline) {
        None, None, None, None, None, None, None, None, Some(*overline), None,
    }
    (border: Border) {
        None, None, None, None, None, None, None, None, None, Some(*border),
    }
);

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn default() {
        assert_eq!(Weight::default(), ResetWeight);
        assert_eq!(Slant::default(), ResetSlant);
        assert_eq!(Blink::default(), ResetBlink);
        assert_eq!(Invert::default(), ResetInvert);
        assert_eq!(Strike::default(), ResetStrike);
        assert_eq!(Underline::default(), ResetUnderline);
        assert_eq!(Overline::default(), ResetOverline);
        assert_eq!(Border::default(), ResetBorder);
    }
}
