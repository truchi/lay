use super::{Style, Styler};
use std::fmt::{Display, Error, Formatter};

/// `Display`able `Style`d content.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Styled<T: Display> {
    pub content: T,
    pub style:   Style,
}

impl<T: Display> Styled<T> {
    /// Retuns a new `Styled` with `content` and `style`.
    pub fn new(content: T, style: Style) -> Self {
        Self { content, style }
    }
}

impl<T: Display> From<(T, Style)> for Styled<T> {
    fn from((content, style): (T, Style)) -> Self {
        Self::new(content, style)
    }
}

impl<T: Display> From<T> for Styled<T> {
    fn from(content: T) -> Self {
        Self::new(content, Style::default())
    }
}

impl<T: Display> Styler for Styled<T> {
    impl_styler!(styled => styled.style);
}

impl_styler_ops!(Styled<T: Display,>);

impl<T: Display> Display for Styled<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}{}{}", self.style, self.content, !self.style)
    }
}
