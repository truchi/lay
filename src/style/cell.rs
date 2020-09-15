use super::Styled;

/// `Styled<char>`.
///
/// A `Cell` conceptually has 2 layers:
/// - a *foreground*: `char`, `Option<Foreground>` and other `Option`al text
///   attributes (e.g. `Weighted`),
/// - a *background*: `Option<Background>`.
///
/// With regards to colors, `None` denotes transparency.  
/// With regards to content, `char::default()` (NUL) is the only char to denotes
/// transparency. Hence, `' '` is a visible blank.
pub type Cell = Styled<Option<char>>;

/// See [`Cell`](type.Cell.html).
impl Cell {
    /// Whether `Cell` has a visible foreground.
    pub fn has_foreground(&self) -> bool {
        self.content != char::default() && self.style.foreground.is_some()
    }

    /// Whether `Cell` has a visible background.
    pub fn has_background(&self) -> bool {
        self.style.background.is_some()
    }

    /// Superimposes `above` above `self`.
    ///
    /// When `above` has a visible background, all we see is `above`.  
    /// Otherwise when `above` has a visible foreground, we see `above` with
    /// `below`'s background.  
    /// Otherwise we see `below`.
    pub fn above(&self, above: &Self) -> Self {
        if above.has_background() {
            // Cannot see through `above`
            return *above;
        } else if above.has_foreground() {
            // See through `above`'s backgroung
            let mut above = *above;
            above.style.background = self.style.background;

            return above;
        } else {
            // `above` is invisible
            *self
        }
    }

    /// Superimposes `below` below `self`.
    ///
    /// See [`Cell::above()`](#method.above).
    pub fn below(&self, below: &Self) -> Self {
        below.above(self)
    }
}
