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
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Style {
    foreground: Option<Foreground>,
    background: Option<Background>,
    weighted:   Option<Weighted>,
    slanted:    Option<Slanted>,
    blinking:   Option<Blinking>,
    inverted:   Option<Inverted>,
    striked:    Option<Striked>,
    underlined: Option<Underlined>,
    overlined:  Option<Overlined>,
    bordered:   Option<Bordered>,
}

impl Style {
    /// An `OptionalStyle` with fields set to `None`.
    pub const EMPTY: Self = Self {
        foreground: None,
        background: None,
        weighted:   None,
        slanted:    None,
        blinking:   None,
        inverted:   None,
        striked:    None,
        underlined: None,
        overlined:  None,
        bordered:   None,
    };
    /// A `Style` with fields set to their reset variant.
    pub const RESET: Self = Self {
        foreground: Some(Foreground(Color::Reset)),
        background: Some(Background(Color::Reset)),
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

impl Styler for Style {
    impl_styler!(style {
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
}

impl_styler_ops!(Style);

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        <Style as Styler>::fmt(self, f)
    }
}
