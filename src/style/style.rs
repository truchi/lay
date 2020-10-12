use super::*;
use std::fmt::{Display, Error, Formatter};

#[macro_use]
macro_rules! style {
    ($(#[$meta:meta])* $($attr:ident: $Attr:ident $reset:expr,)*) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
        pub struct Style {
            $(pub $attr: Option<$Attr>,)*
        }

        impl Style {
            /// A `Style` with fields set to `None`.
            pub const NONE: Self = Self {
                $($attr: None,)*
            };
            /// A `Style` with fields set to their reset value.
            pub const RESET: Self = Self {
                $($attr: Some($reset),)*
            };
        }

        $(doc!("Returns an empty `Style` with `Some(" stringify!($attr) ")`",
        impl From<$Attr> for Style {
            doc!("Returns an empty `Style` with `Some(" stringify!($attr) ")`",
            fn from($attr: $Attr) -> Self {
                Style::NONE.$attr($attr)
            });
        });)*

        impl_styler_index!(
                (style: Style) { $(style.$attr,)* }
            mut (style: Style) { $(&mut style.$attr,)* }
        );

        impl_styler!(
                (style: Style) -> Self { $(($attr) { style.$attr = $attr; style },)* }
            mut (style: Style) { $(($attr) style.$attr = $attr,)* }
        );

        #[cfg(feature = "styler-idx")]
        impl_styler_idx!(
                (style: Style) { $(&style.$attr,)* }
            mut (style: Style) { $(&mut style.$attr,)* }
        );

        #[cfg(feature = "styler-ops")]
        impl_styler_ops!(
                (Style)
            mut (Style)
        );
    };
}

style!(
    /// `Style`s.
    ///
    /// A straightforward implementation of `Styler`.
    ///
    /// `Display`s the corresponding CSIs to the terminal.
    ///
    /// `Default`s as an empty `Style` (all fields set to `None`).
    foreground: Foreground Foreground(Color::ResetColor),
    background: Background Background(Color::ResetColor),
    weight: Weight Weight::ResetWeight,
    slant: Slant Slant::ResetSlant,
    blink: Blink Blink::ResetBlink,
    invert: Invert Invert::ResetInvert,
    strike: Strike Strike::ResetStrike,
    underline: Underline Underline::ResetUnderline,
    overline: Overline Overline::ResetOverline,
    border: Border Border::ResetBorder,
);

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Styler::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn consts() {
        assert_eq!(Style::NONE, Style::default());
        assert_eq!(Style::RESET, Style {
            foreground: Some(Foreground(ResetColor)),
            background: Some(Background(ResetColor)),
            weight:     Some(ResetWeight),
            slant:      Some(ResetSlant),
            blink:      Some(ResetBlink),
            invert:     Some(ResetInvert),
            strike:     Some(ResetStrike),
            underline:  Some(ResetUnderline),
            overline:   Some(ResetOverline),
            border:     Some(ResetBorder),
        });
    }

    #[test]
    fn default() {
        assert_eq!(Style::default(), Style {
            foreground: None,
            background: None,
            weight:     None,
            slant:      None,
            blink:      None,
            invert:     None,
            strike:     None,
            underline:  None,
            overline:   None,
            border:     None,
        });
    }

    #[test]
    fn conversion() {
        let foreground = Foreground(Grey);
        let background = Background(Red);
        let weight = Bold;
        let slant = ResetSlant;
        let blink = Slow;
        let invert = Inverted;
        let strike = Striked;
        let underline = ResetUnderline;
        let overline = Overlined;
        let border = Circle;

        macro_rules! conversion {
            ($($var:ident)*) => {
                $(assert_eq!(Style::from($var), Style {
                    $var: Some($var),
                    ..Style::NONE
                });)*
            };
        }

        conversion!(foreground background weight slant blink invert strike underline overline border);
    }

    #[test]
    fn styler_index() {
        macro_rules! styler_index {
            ($($attr:ident $get:ident $get_mut:ident [$idx:ident] $Attr:expr, $NewAttr:expr)*) => {
                let mut style = Style {
                    $($attr: $Attr,)*
                };

                $(
                    assert_eq!(style.$get(), $Attr);
                    assert_eq!(style[$idx], $Attr);

                    *style.$get_mut() = $NewAttr;
                    assert_eq!(style.$attr, $NewAttr);
                    style[$idx] = $Attr;
                    assert_eq!(style.$attr, $Attr);
                )*
            };
        }

        styler_index!(
            foreground get_foreground get_foreground_mut [Fg] Some(Foreground(DarkCyan)), Some(Foreground(Blue))
            background get_background get_background_mut [Bg] Some(Background(DarkMagenta)), None
            weight get_weight get_weight_mut [Wgt] Some(Bold), Some(Light)
            slant get_slant get_slant_mut [Slt] Some(Italic), Some(Italic)
            blink get_blink get_blink_mut [Blk] None, Some(Slow)
            invert get_invert get_invert_mut [Inv] Some(Inverted), Some(ResetInvert)
            strike get_strike get_strike_mut [Stk] Some(ResetStrike), None
            underline get_underline get_underline_mut [Udl] Some(Underlined), Some(ResetUnderline)
            overline get_overline get_overline_mut [Ovl] Some(ResetOverline), None
            border get_border get_border_mut [Brd] None, Some(Circle)
        );
    }

    #[test]
    fn styler() {
        let mut style = Style {
            foreground: Some(Foreground(DarkBlue)),
            background: Some(Background(DarkRed)),
            weight:     Some(Bold),
            slant:      None,
            blink:      Some(ResetBlink),
            invert:     Some(Inverted),
            strike:     Some(ResetStrike),
            underline:  None,
            overline:   Some(ResetOverline),
            border:     None,
        };

        macro_rules! styler {
            ($($attr:ident($Attr:expr) $attr_mut:ident($NewAttr:expr))*) => {
                $(
                    assert_eq!(style.$attr($Attr), Style {
                        $attr: $Attr,
                        ..style
                    });

                    style.$attr_mut($NewAttr);
                    assert_eq!(style, Style {
                        $attr: $NewAttr,
                        ..style
                    });
                )*
            };
        }

        styler!(
            foreground(Some(Foreground(Black))) foreground_mut(Some(Foreground(Black)))
            background(None) background_mut(Some(Background(White)))
            weight(Some(Light)) weight_mut(None)
            slant(None) slant_mut(None)
            blink(Some(Slow)) blink_mut(Some(Slow))
            invert(Some(Inverted)) invert_mut(Some(Inverted))
            strike(None) strike_mut(Some(Striked))
            underline(Some(Underlined)) underline_mut(Some(Underlined))
            overline(Some(Overlined)) overline_mut(Some(Overlined))
            border(Some(Circle)) border_mut(None)
        );
    }

    // TODO test styler ops
}
