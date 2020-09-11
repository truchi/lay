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

impl_styler!(Styled<T: Display,>, field: style);
impl_styler_ops!(Styled<T: Display,>);

impl<T: Display> From<T> for Styled<T> {
    fn from(content: T) -> Self {
        Self::new(content, Default::default())
    }
}

impl<T: Display> Display for Styled<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}{}{}", self.style, self.content, self.style.reset())
    }
}
