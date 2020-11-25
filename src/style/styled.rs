use crate::*;
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

    /// Sets `content`.
    pub fn content(mut self, content: T) -> Self {
        self.content_mut(content);
        self
    }

    /// Sets `content`, mutably.
    pub fn content_mut(&mut self, content: T) {
        self.content = content;
    }
}

/// Retuns a new `Styled` with `content` and `style`.
impl<T: Display> From<(T, Style)> for Styled<T> {
    /// Retuns a new `Styled` with `content` and `style`.
    fn from((content, style): (T, Style)) -> Self {
        Self::new(content, style)
    }
}

/// Retuns a new `Styled` with `content` and
/// [`Style::NONE`](crate::Style::NONE).
impl<T: Display> From<T> for Styled<T> {
    /// Retuns a new `Styled` with `content` and
    /// [`Style::NONE`](crate::Style::NONE).
    fn from(content: T) -> Self {
        Self::new(content, Style::NONE)
    }
}

/// `Display`s the `content` with `style`s, then resets `style`s.
impl<T: Display> Display for Styled<T> {
    /// `Display`s the `content` with `style`s, then resets `style`s.
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Style::fmt(&self.style, f)?;
        T::fmt(&self.content, f)?;
        Style::fmt(&self.style.reset(), f)
    }
}
