use super::{Style, Styler};
use std::fmt::{Display, Error, Formatter};

/// `Display`able `Style`d content.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Styled<T> {
    pub content: T,
    pub style:   Style,
}

impl<T> Styled<T> {
    pub fn new(content: T, style: Style) -> Self {
        Self { content, style }
    }
}

impl<T> Styler for Styled<T> {
    impl_styler!(style => style.style);
}

impl_styler_ops!(Styled<T,>);

impl<T> From<(T, Style)> for Styled<T> {
    fn from((content, style): (T, Style)) -> Self {
        Self::new(content, style)
    }
}

impl<T: Display> Display for Styled<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}{}{}",
            self.style,
            self.content,
            self.style.and(&Style::default())
        )
    }
}
