use super::*;
use std::fmt::{Display, Error, Formatter};

/// `Reset`s all terminal attributes.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub struct Reset;

macro_rules! reset {
    (
        Colors { $($Color:ident,)* }
        Attributes { $($Attr:ident $reset_attr:ident,)* }
    ) => {
        $(reset!([From $Color]
            stringify!($Color) "(Color::ResetColor)",
            $Color(Color::ResetColor)
        );)*

        $(reset!([From $Attr]
            stringify!($Attr) "::" stringify!($reset_attr),
            $Attr::$reset_attr
        );)*
    };
    ([From $Self:ident] $($doc:expr)*, $body:expr) => {
        doc!("Returns `Some(" $($doc)* ")`.",
        impl From<Reset> for Option<$Self> {
            doc!("Returns `Some(" $($doc)* ")`.",
            fn from(_: Reset) -> Self {
                Some($body)
            });
        });
    };
}

reset!(
    Colors {
        Foreground,
        Background,
    }
    Attributes {
        Weight ResetWeight,
        Slant ResetSlant,
        Blink ResetBlink,
        Invert ResetInvert,
        Strike ResetStrike,
        Underline ResetUnderline,
        Overline ResetOverline,
        Border ResetBorder,
    }
);

impl_styler_index!(
    (reset: Reset) {
        Some(Foreground(Color::ResetColor)),
        Some(Background(Color::ResetColor)),
        Some(Weight::ResetWeight),
        Some(Slant::ResetSlant),
        Some(Blink::ResetBlink),
        Some(Invert::ResetInvert),
        Some(Strike::ResetStrike),
        Some(Underline::ResetUnderline),
        Some(Overline::ResetOverline),
        Some(Border::ResetBorder),
    }
);

impl_styler!((__: Reset) -> Style {
    (foreground) Style::RESET.foreground(foreground),
    (background) Style::RESET.background(background),
    (weight)     Style::RESET.weight(weight),
    (slant)      Style::RESET.slant(slant),
    (blink)      Style::RESET.blink(blink),
    (invert)     Style::RESET.invert(invert),
    (strike)     Style::RESET.strike(strike),
    (underline)  Style::RESET.underline(underline),
    (overline)   Style::RESET.overline(overline),
    (border)     Style::RESET.border(border),
});

/// Prints the "Reset" CSI to the terminal.
impl Display for Reset {
    /// Prints the "Reset" CSI to the terminal.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("\x1B[0m")
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn conversion() {
        macro_rules! conversion {
            ($($Attr:ident $reset:expr)*) => {
                $(assert_eq!(Option::<$Attr>::from(Reset), Some($reset));)*
            };
        }

        conversion!(
            Foreground Foreground(ResetColor)
            Background Background(ResetColor)
            Weight ResetWeight
            Slant ResetSlant
            Blink ResetBlink
            Invert ResetInvert
            Strike ResetStrike
            Underline ResetUnderline
            Overline ResetOverline
            Border ResetBorder
        );
    }

    #[test]
    fn styler_index() {
        macro_rules! styler_index {
            ($($get:ident $reset:expr)*) => {
                $(assert_eq!(Reset.$get(), Some($reset));)*
            };
        }

        styler_index!(
            get_foreground Foreground(ResetColor)
            get_background Background(ResetColor)
            get_weight ResetWeight
            get_slant ResetSlant
            get_blink ResetBlink
            get_invert ResetInvert
            get_strike ResetStrike
            get_underline ResetUnderline
            get_overline ResetOverline
            get_border ResetBorder
        );
    }

    #[test]
    fn styler() {
        macro_rules! styler {
            ($($attr:ident($Attr:expr))*) => {
                $(assert_eq!(Reset.$attr($Attr), Style {
                    $attr: Some($Attr),
                    ..Style::RESET
                });)*
            };
        }

        styler!(
            foreground(Foreground(Black))
            background(Background(White))
            weight(Light)
            slant(Italic)
            blink(Slow)
            invert(Inverted)
            strike(Striked)
            underline(Underlined)
            overline(Overlined)
            border(Circle)
        );
    }
}
