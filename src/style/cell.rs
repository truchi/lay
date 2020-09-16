use super::Styled;

/// `Styled<char>`.
///
/// `Cell`s without `Foreground` have transparent foreground.  
/// `Cell`s without `Background` have transparent background.
pub type Cell = Styled<char>;

/// See [`Cell`](type.Cell.html).
impl Cell {
    /// Whether this `Cell` has a `Some(Foreground)`.
    pub fn has_foreground(&self) -> bool {
        self.style.foreground.is_some()
    }

    /// Whether this `Cell` has a `Some(Background)`.
    pub fn has_background(&self) -> bool {
        self.style.background.is_some()
    }

    /// Superimposes `above` above `self`.
    pub fn above(&self, above: &Self) -> Self {
        if above.has_background() {
            *above
        } else if above.has_foreground() {
            let mut above = *above;
            above.style.background = self.style.background;

            above
        } else {
            *self
        }
    }

    /// Superimposes `below` below `self`.
    pub fn below(&self, below: &Self) -> Self {
        below.above(self)
    }
}
