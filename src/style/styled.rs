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

// impl Styled<char> {
//     /// Whether `Cell` has a visible foreground
//     pub fn has_foreground(&self) -> bool {
//         self.content != ' ' && self.style.foreground.is_some()
//     }
//
//     /// Whether `Cell` has a visible background
//     pub fn has_background(&self) -> bool {
//         self.style.background.is_some()
//     }
//
//     /// Merges `below` and `above`.
//     /// When `above` has a background `Color`, all we see is `above`.
//     /// When above has no background `Color` but a `char` and a foreground
//     /// `Color`, we see `above` with `below`'s background.
//     /// Otherwise we see `below`.
//     pub fn above(&self, above: &Self) -> Self {
//         if above.has_background() {
//             // Cannot see through `above`
//             return *above;
//         } else if above.has_foreground() {
//             // See through `above`'s backgroung
//             let mut above = *above;
//             above.style.background = self.style.background;
//
//             return above;
//         } else {
//             // `above` is invisible
//             *self
//         }
//     }
//
//     pub fn below(&self, below: &Self) -> Self {
//         below.above(self)
//     }
// }

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
