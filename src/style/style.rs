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
    pub weighted:   Weighted,
    pub slanted:    Slanted,
    pub blinking:   Blinking,
    pub inverted:   Inverted,
    pub striked:    Striked,
    pub underlined: Underlined,
    pub overlined:  Overlined,
    pub bordered:   Bordered,
}

impl_styler!(Style style {
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

impl_styler_ops!(Style);

impl Display for Style {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        macro_rules! display {
            ($($field:ident)*) => {
                $(write!(f, "{}", self.$field)?;)*
            };
        }

        display!(foreground background weighted slanted blinking inverted striked underlined overlined bordered);
        Ok(())
    }
}
