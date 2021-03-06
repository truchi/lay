use crate::*;

/// `Display`able [`Style`](crate::Style)d content.
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Styled<T> {
    pub content: T,
    pub style:   Style,
}

/// ### Constructors
impl<T> Styled<T> {
    /// Retuns a new [`Styled`](crate::Styled) with `content` and `style`.
    pub fn new(content: T, style: impl Into<Style>) -> Self {
        Self {
            content,
            style: style.into(),
        }
    }

    /// Retuns a new [`Styled`](crate::Styled) with `content` and
    /// [`Style::NONE`](crate::Style::NONE).
    pub fn with_default(content: T) -> Self {
        Self {
            content,
            style: Style::NONE,
        }
    }

    /// Retuns a new [`Styled`](crate::Styled) with `content` and
    /// [`Style::RESET`](crate::Style::RESET).
    pub fn with_reset(content: T) -> Self {
        Self {
            content,
            style: Style::RESET,
        }
    }
}

/// ### Methods
impl<T> Styled<T> {
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

/// `Display`s the `content` with `style`s, then resets `style`s.
impl<T: Display> Display for Styled<T> {
    /// `Display`s the `content` with `style`s, then resets `style`s.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.style, f)?;
        T::fmt(&self.content, f)?;
        Display::fmt(&self.style.reset(), f)
    }
}

impl<T: Debug> Debug for Styled<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_tuple("Styled")
            .field(&self.content)
            .field(&self.style)
            .finish()
    }
}

// =========== //
// Conversions //
// =========== //

/// Retuns a new [`Styled`](crate::Styled) with `content` and `style`.
impl<T, U: Into<Style>> From<(T, U)> for Styled<T> {
    /// Retuns a new [`Styled`](crate::Styled) with `content` and `style`.
    fn from((content, style): (T, U)) -> Self {
        Self::new(content, style)
    }
}

/// Retuns a new [`Styled`](crate::Styled) with `content` and
/// [`Style::NONE`](crate::Style::NONE).
impl<T> From<T> for Styled<T> {
    /// Retuns a new [`Styled`](crate::Styled) with `content` and
    /// [`Style::NONE`](crate::Style::NONE).
    fn from(content: T) -> Self {
        Self::with_default(content)
    }
}

/// Retuns a new [`Styled`](crate::Styled) with `content` and
/// [`Style::RESET`](crate::Style::RESET).
impl<T> From<(T, Reset)> for Styled<T> {
    /// Retuns a new [`Styled`](crate::Styled) with `content` and
    /// [`Style::RESET`](crate::Style::RESET).
    fn from((content, _): (T, Reset)) -> Self {
        Self::with_reset(content)
    }
}
