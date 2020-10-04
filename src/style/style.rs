use super::{Style, Styler};
use std::fmt::{Display, Error, Formatter};

#[macro_use]
macro_rules! style {
    ($(($attr:ident: $Attr:ident, $set_attr:ident, $Reset:expr))*) => {
        /// `Style`s.
        ///
        /// A straightforward implementation of `Styler`.
        ///
        /// `Display`s the corresponding CSIs to the terminal.
        ///
        /// `Default`s as an empty `Style` (all fields set to `None`).
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
                $($attr: Some($Reset),)*
            };
        }

        $(impl From<$Attr> for Style {
            fn from($attr: $Attr) -> Self {
                Style::NONE.$set_attr($attr)
            }
        })*

        impl_styler_index!((style: Style) {
            $(style.$attr,)*
        });
        impl_styler_index_mut!((style: Style) {
            $(&mut style.$attr,)*
        });
        impl_styler!((style: Style) {
            $($attr { style.$attr = $attr; style },)*
        });
        impl_styler_mut!((style: Style) {
            $($attr style.$attr = $attr,)*
        });

        #[cfg(feature = "styler-idx")]
        impl_styler_idx!((style: Style) {
            $(&style.$attr,)*
        });
        #[cfg(feature = "styler-idx")]
        impl_styler_mut_idx!((style: Style) {
            $(&mut style.$attr,)*
        });

        #[cfg(feature = "styler-ops")]
        impl_styler_ops!(Style);
        #[cfg(feature = "styler-ops")]
        impl_styler_mut_ops!(Style);
    };
}

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
        assert_eq!(Style::from(Foreground(Red)), Style {
            foreground: Some(Foreground(Red)),
            ..Style::default()
        });
        assert_eq!(Style::from(Background(Green)), Style {
            background: Some(Background(Green)),
            ..Style::default()
        });
        assert_eq!(Style::from(Bold), Style {
            weight: Some(Bold),
            ..Style::default()
        });
    }

    #[test]
    fn ops() {
        assert_eq!(Style::default() * Blue / Yellow + Bold + Italic, Style {
            foreground: Some(Foreground(Blue)),
            background: Some(Background(Yellow)),
            weight: Some(Bold),
            slant: Some(Italic),
            ..Style::default()
        });
    }
}
