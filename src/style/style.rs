use super::{
    Background,
    Blinking,
    Bordered,
    Color,
    Foreground,
    Inverted,
    Overlined,
    Slanted,
    Striked,
    Styler,
    Underlined,
    Weighted,
};
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
    pub weighted:   Option<Weighted>,
    pub slanted:    Option<Slanted>,
    pub blinking:   Option<Blinking>,
    pub inverted:   Option<Inverted>,
    pub striked:    Option<Striked>,
    pub underlined: Option<Underlined>,
    pub overlined:  Option<Overlined>,
    pub bordered:   Option<Bordered>,
}

impl Style {
    /// A `Style` with fields set to their reset value.
    pub const RESET: Self = Self {
        foreground: Some(Foreground(Color::ResetColor)),
        background: Some(Background(Color::ResetColor)),
        weighted:   Some(Weighted::ResetWeight),
        slanted:    Some(Slanted::ResetSlant),
        blinking:   Some(Blinking::ResetBlink),
        inverted:   Some(Inverted::ResetInvert),
        striked:    Some(Striked::ResetStrike),
        underlined: Some(Underlined::ResetUnderline),
        overlined:  Some(Overlined::ResetOverline),
        bordered:   Some(Bordered::ResetBorder),
    };
}

impl_styler!((style: Style) {
    style.foreground,
    style.background,
    style.weighted,
    style.slanted,
    style.blinking,
    style.inverted,
    style.striked,
    style.underlined,
    style.overlined,
    style.bordered,
});

macro_rules! style {
    ($(($attr:ident : $Attr:ty),)*) => {
        $(impl From<$Attr> for Style {
            fn from($attr: $Attr) -> Self {
                Self::default() + $attr
            }
        })*
    };
}

style!(
    (foreground: Foreground),
    (background: Background),
    (weighted: Weighted),
    (slanted: Slanted),
    (blinking: Blinking),
    (inverted: Inverted),
    (striked: Striked),
    (underlined: Underlined),
    (overlined: Overlined),
    (bordered: Bordered),
);

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Styler::fmt(self, f)
    }
}
