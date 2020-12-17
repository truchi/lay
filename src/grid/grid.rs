use super::Size;
use std::{convert::TryFrom, ops::Deref};

/// Error type for [`Grid`](crate::Grid) constructors.
pub enum GridError<T> {
    /// `width * height > usize::MAX`.
    Overflow(Size<usize>, T),
    /// `width * height != len`.
    Mismatch(Size<usize>, T),
}

/// 2D [`Grid`](crate::Grid).
#[derive(Eq, PartialEq, Debug)]
pub struct Grid<T, U: Deref<Target = [T]>> {
    size:  Size<usize>,
    elems: U,
}

/// ### Constructors
impl<T, U: Deref<Target = [T]>> Grid<T, U> {
    /// Creates a new [`Grid`](crate::Grid)
    /// or returns a [`GridError`](GridError).
    pub fn new<S: Into<Size<usize>>>(size: S, elems: U) -> Result<Self, GridError<U>> {
        let size = size.into();

        match size.width.checked_mul(size.height) {
            None => Err(GridError::Overflow(size, elems)),
            Some(len) =>
                if len != elems.len() {
                    Err(GridError::Mismatch(size, elems))
                } else {
                    Ok(Self { size, elems })
                },
        }
    }
}

impl<S: Into<Size<usize>>, T, U: Deref<Target = [T]>> TryFrom<(S, U)> for Grid<T, U> {
    type Error = GridError<U>;

    fn try_from((size, elems): (S, U)) -> Result<Self, Self::Error> {
        let size = size.into();

        match size.width.checked_mul(size.height) {
            None => Err(GridError::Overflow(size, elems)),
            Some(len) =>
                if len != elems.len() {
                    Err(GridError::Mismatch(size, elems))
                } else {
                    Ok(Self { size, elems })
                },
        }
    }
}

// impl<T> Grid<T> {
// pub fn width(&self) -> usize {
// self.width
// }
//
// pub fn height(&self) -> usize {
// self.height
// }
//
// pub fn len(&self) -> usize {
// self.width * self.height
// }
// }
