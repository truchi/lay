/// Moves cursor [`To`](crate::To) `column, row`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct To(pub u16, pub u16);

/// Moves cursor [`Right`](crate::Right) `n` columns.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Right(pub u16);

/// Moves cursor [`Left`](crate::Left) `n` columns.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Left(pub u16);

/// Moves cursor [`Down`](crate::Down) `n` lines.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct Down(pub u16);

#[cfg(feature = "backend-crossterm")]
mod crossterm_backend {
    use crate::*;

    impl Display for To {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            let To(x, y) = *self;
            Display::fmt(&crossterm::cursor::MoveTo(x, y), f)
        }
    }

    impl Display for Right {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            let Right(cols) = *self;
            Display::fmt(&crossterm::cursor::MoveRight(cols), f)
        }
    }

    impl Display for Left {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            let Left(cols) = *self;
            Display::fmt(&crossterm::cursor::MoveLeft(cols), f)
        }
    }

    impl Display for Down {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            let Down(rows) = *self;
            Display::fmt(&crossterm::cursor::MoveDown(rows), f)
        }
    }
}
