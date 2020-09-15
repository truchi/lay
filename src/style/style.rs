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

/// `Option`al `Style`s.
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
                $(if let Some(field) = self.$field {
                    write!(f, "{}", field)?;
                })*
            };
        }

        display!(foreground background weighted slanted blinking inverted striked underlined overlined bordered);
        Ok(())
    }
}
