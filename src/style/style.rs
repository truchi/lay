use super::*;
use std::fmt::{Display, Error, Formatter};

/// `Style`s.
///
/// A straightforward implementation of `Styler`.
///
/// `Display`s the corresponding CSIs to the terminal.
///
/// `Default`s as an empty `Style` (all fields set to `None`).
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Style {
    pub foreground: Option<Foreground>,
    pub background: Option<Background>,
    pub weight:     Option<Weight>,
    pub slant:      Option<Slant>,
    pub blink:      Option<Blink>,
    pub invert:     Option<Invert>,
    pub strike:     Option<Strike>,
    pub underline:  Option<Underline>,
    pub overline:   Option<Overline>,
    pub border:     Option<Border>,
}

impl Style {
    /// A `Style` with fields set to `None`.
    pub const NONE: Self = Self {
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
    };
    /// A `Style` with fields set to their reset value.
    pub const RESET: Self = Self {
        foreground: Some(Foreground(Color::ResetColor)),
        background: Some(Background(Color::ResetColor)),
        weight:     Some(Weight::ResetWeight),
        slant:      Some(Slant::ResetSlant),
        blink:      Some(Blink::ResetBlink),
        invert:     Some(Invert::ResetInvert),
        strike:     Some(Strike::ResetStrike),
        underline:  Some(Underline::ResetUnderline),
        overline:   Some(Overline::ResetOverline),
        border:     Some(Border::ResetBorder),
    };
}

#[macro_use]
macro_rules! style {
    ($($attr:ident: $Attr:ident,)*) => {
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
    foreground: Foreground,
    background: Background,
    weight: Weight,
    slant: Slant,
    blink: Blink,
    invert: Invert,
    strike: Strike,
    underline: Underline,
    overline: Overline,
    border: Border,
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
