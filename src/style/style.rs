use super::{
    Background,
    Blinking,
    Bordered,
    Foreground,
    Inverted,
    Overlined,
    Slanted,
    Striked,
    Underlined,
    Weighted,
};
use std::fmt::{Display, Error, Formatter};

/// `Style`s.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Style {
    pub foreground: Foreground,
    pub background: Background,
    pub weight:     Weighted,
    pub slant:      Slanted,
    pub blink:      Blinking,
    pub invert:     Inverted,
    pub strike:     Striked,
    pub underline:  Underlined,
    pub overline:   Overlined,
    pub border:     Bordered,
}

impl_styler!(Style style {
    style.foreground,
    style.background,
    style.weight,
    style.slant,
    style.blink,
    style.invert,
    style.strike,
    style.underline,
    style.overline,
    style.border,
});

// impl_styler!(Style);
// impl_styler_ops!(Style);

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        macro_rules! display {
            ($($field:ident)*) => {
                $(write!(f, "{}", self.$field)?;)*
            };
        }

        display!(foreground background weight slant blink invert strike underline overline border);
        Ok(())
    }
}
