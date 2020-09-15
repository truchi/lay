use super::{Style, Styler};
use std::fmt::{Display, Error, Formatter};

/// `Display`able `Style`d content.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Styled<T, U: Styler = Style> {
    pub content: T,
    pub style:   U,
}

impl<T, U: Styler> Styled<T, U> {
    pub fn new(content: T, style: U) -> Self {
        Self { content, style }
    }
}

impl_styler!(Styled<T, U: Styler,> style {
    style.style.foreground,
    style.style.background,
    style.style.weighted,
    style.style.slanted,
    style.style.blinking,
    style.style.inverted,
    style.style.striked,
    style.style.underlined,
    style.style.overlined,
    style.style.bordered,
});

// impl_styler_ops!(Styled<T,>);

impl<T> From<T> for Styled<T> {
    fn from(content: T) -> Self {
        Self::new(content, Default::default())
    }
}

impl<T> From<(T, Style)> for Styled<T> {
    fn from((content, style): (T, Style)) -> Self {
        Self::new(content, style)
    }
}

impl<T: Display> Display for Styled<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}{}{}", self.style, self.content, Style::default()) // TODO
    }
}
