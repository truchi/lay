use super::Style;
use std::fmt::{Display, Error, Formatter};

/// `Style`d `Display`able content.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Styled<T: Display> {
    pub content: T,
    pub style:   Style,
}

impl<T: Display> Styled<T> {
    pub fn new(content: T, style: Style) -> Self {
        Self { content, style }
    }
}

impl_styler!(Styled<T: Display,> style {
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
impl_styler_ops!(Styled<T: Display,>);

impl Styled<char> {
    /// Superimposes `above` above `self`.
    /// Returns `above` if its content is not `char::default()`, `self`
    /// otherwise.
    pub fn above(&self, above: &Self) -> Self {
        if above.content == char::default() {
            *self
        } else {
            *above
        }
    }

    /// Superimposes `below` below `self`.
    /// Returns `self` if its content is not `char::default()`, `below`
    /// otherwise.
    pub fn below(&self, below: &Self) -> Self {
        below.above(self)
    }
}

impl<T: Display> From<T> for Styled<T> {
    fn from(content: T) -> Self {
        Self::new(content, Default::default())
    }
}

impl<T: Display> From<(T, Style)> for Styled<T> {
    fn from((content, style): (T, Style)) -> Self {
        Self::new(content, style)
    }
}

impl<T: Display> Display for Styled<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}{}{}", self.style, self.content, Style::default())
    }
}
