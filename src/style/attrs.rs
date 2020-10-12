use super::*;
use std::fmt::{Display, Error, Formatter};

macro_rules! attributes {
    ($(
        $Attr:ident($attr:ident) {
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

            impl_styler!((a: $Attr) -> Style {
                (foreground) Style::NONE.$attr(Some(a)).foreground(foreground),
                (background) Style::NONE.$attr(Some(a)).background(background),
                (weight)     Style::NONE.$attr(Some(a)).weight(weight),
                (slant)      Style::NONE.$attr(Some(a)).slant(slant),
                (blink)      Style::NONE.$attr(Some(a)).blink(blink),
                (invert)     Style::NONE.$attr(Some(a)).invert(invert),
                (strike)     Style::NONE.$attr(Some(a)).strike(strike),
                (underline)  Style::NONE.$attr(Some(a)).underline(underline),
                (overline)   Style::NONE.$attr(Some(a)).overline(overline),
                (border)     Style::NONE.$attr(Some(a)).border(border),
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
    Weight(weight) {
        ResetWeight("22")
        Bold("1")
        Light("2")
    }
    Slant(slant) {
        ResetSlant("23")
        Italic("3")
    }
    Blink(blink) {
        ResetBlink("25")
        Slow("5")
        Fast("6")
    }
    Invert(invert) {
        ResetInvert("27")
        Inverted("7")
    }
    Strike(strike) {
        ResetStrike("29")
        Striked("9")
    }
    Underline(underline) {
        ResetUnderline("24")
        Underlined("4")
    }
    Overline(overline) {
        ResetOverline("55")
        Overlined("53")
    }
    Border(border) {
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

    #[test]
    fn styler_index() {
        let weight = Bold;
        let slant = ResetSlant;
        let blink = Slow;
        let invert = Inverted;
        let strike = Striked;
        let underline = ResetUnderline;
        let overline = Overlined;
        let border = Circle;

        macro_rules! styler_index {
            ($($var:ident $get_some:ident $($get_none:ident)*,)*) => {
                $(
                    assert_eq!($var.$get_some(), Some($var));
                    $(assert_eq!($var.$get_none(), None);)*
                )*
            };
        }

        styler_index!(
            weight get_weight get_foreground get_background get_slant get_blink
                get_invert get_strike get_underline get_overline get_border,
            slant get_slant get_foreground get_background get_weight get_blink
                get_invert get_strike get_underline get_overline get_border,
            blink get_blink get_foreground get_background get_weight get_slant
                get_invert get_strike get_underline get_overline get_border,
            invert get_invert get_foreground get_background get_weight get_slant
                get_blink get_strike get_underline get_overline get_border,
            strike get_strike get_foreground get_background get_weight get_slant
                get_blink get_invert get_underline get_overline get_border,
            underline get_underline get_foreground get_background get_weight get_slant
                get_blink get_invert get_strike get_overline get_border,
            overline get_overline get_foreground get_background get_weight get_slant
                get_blink get_invert get_strike get_underline get_border,
            border get_border get_foreground get_background get_weight get_slant
                get_blink get_invert get_strike get_underline get_overline,
        );
    }

    #[test]
    fn styler() {
        let weight = Light;
        let slant = Italic;
        let blink = Fast;
        let invert = ResetInvert;
        let strike = ResetStrike;
        let underline = Underlined;
        let overline = ResetOverline;
        let border = Frame;

        macro_rules! styler {
            ($($var:ident($Attr:expr))*) => {
                $(assert_eq!($var.$var($Attr), Style {
                    $var: Some($Attr),
                    ..Style::NONE
                });)*
            };
            ($($var:ident $(.$attr:ident($Attr:expr))*,)*) => {
                $($(assert_eq!($var.$attr($Attr), Style {
                    $var: Some($var),
                    $attr: Some($Attr),
                    ..Style::NONE
                });)*)*
            };
        }

        styler!(
            weight(Light)
            slant(ResetSlant)
            blink(Slow)
            invert(Inverted)
            strike(ResetStrike)
            underline(Underlined)
            overline(Overlined)
            border(Circle)
        );

        styler!(
            weight
                .foreground(Foreground(Black))
                .background(Background(White))
                .slant(Italic)
                .blink(Slow)
                .invert(Inverted)
                .strike(Striked)
                .underline(Underlined)
                .overline(Overlined)
                .border(Circle),
            slant
                .foreground(Foreground(Black))
                .background(Background(White))
                .weight(Bold)
                .blink(Slow)
                .invert(Inverted)
                .strike(Striked)
                .underline(Underlined)
                .overline(Overlined)
                .border(Circle),
            blink
                .foreground(Foreground(Black))
                .background(Background(White))
                .weight(Bold)
                .slant(Italic)
                .invert(Inverted)
                .strike(Striked)
                .underline(Underlined)
                .overline(Overlined)
                .border(Circle),
            invert
                .foreground(Foreground(Black))
                .background(Background(White))
                .weight(Bold)
                .slant(Italic)
                .blink(Slow)
                .strike(Striked)
                .underline(Underlined)
                .overline(Overlined)
                .border(Circle),
            strike
                .foreground(Foreground(Black))
                .background(Background(White))
                .weight(Bold)
                .slant(Italic)
                .blink(Slow)
                .invert(Inverted)
                .underline(Underlined)
                .overline(Overlined)
                .border(Circle),
            underline
                .foreground(Foreground(Black))
                .background(Background(White))
                .weight(Bold)
                .slant(Italic)
                .blink(Slow)
                .invert(Inverted)
                .strike(Striked)
                .overline(Overlined)
                .border(Circle),
            overline
                .foreground(Foreground(Black))
                .background(Background(White))
                .weight(Bold)
                .slant(Italic)
                .blink(Slow)
                .invert(Inverted)
                .strike(Striked)
                .underline(Underlined)
                .border(Circle),
            border
                .foreground(Foreground(Black))
                .background(Background(White))
                .weight(Bold)
                .slant(Italic)
                .blink(Slow)
                .invert(Inverted)
                .strike(Striked)
                .underline(Underlined)
                .overline(Overlined),
        );
    }

    // TODO test styler ops
}
