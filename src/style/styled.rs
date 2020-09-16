use super::{Style, Styler};
use std::fmt::{Display, Error, Formatter};

/// `Display`able `Style`d content.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Styled<T, U> {
    pub content: T,
    pub style:   U,
}

impl<T, U> Styled<T, U> {
    pub fn new(content: T, style: U) -> Self {
        Self { content, style }
    }
}

impl<T, U: Styler> Styler for Styled<T, U> {
    impl_styler!(style => style.style);
}

impl_styler_ops!(Styled<T, U: Styler,>);

impl<T, U> From<(T, U)> for Styled<T, U> {
    fn from((content, style): (T, U)) -> Self {
        Self::new(content, style)
    }
}

impl<T: Display, U: Styler + Clone> Display for Styled<T, U> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.style.fmt(f)?;
        self.content.fmt(f)?;
        self.style.clone().and(&Style::default()).fmt(f)?;

        Ok(())
    }
}
