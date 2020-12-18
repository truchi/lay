use super::*;
use std::{convert::TryFrom, marker::PhantomData};

/// Error type for [`Grid`](crate::Grid) constructors.
pub enum GridError<T> {
    /// `width * height > usize::MAX`.
    Overflow(Size<usize>, T),
    /// `width * height != len`.
    Mismatch(Size<usize>, T),
}

/// 2D [`Grid`](crate::Grid).
#[derive(Eq, PartialEq, Debug)]
pub struct Grid<T, U> {
    size:    Size<usize>,
    elems:   U,
    phantom: PhantomData<T>,
}

/// ### Constructors
impl<T, U: AsRef<[T]>> Grid<T, U> {
    /// Creates a new [`Grid`](crate::Grid)
    /// or returns a [`GridError`](GridError).
    pub fn new<S: Into<Size<usize>>>(size: S, elems: U) -> Result<Self, GridError<U>> {
        let size = size.into();

        match size.checked_area() {
            None => Err(GridError::Overflow(size, elems)),
            Some(area) =>
                if area != elems.as_ref().len() {
                    Err(GridError::Mismatch(size, elems))
                } else {
                    Ok(Self {
                        size,
                        elems,
                        phantom: PhantomData,
                    })
                },
        }
    }
}

/// ### Methods
impl<T, U: AsRef<[T]>> Grid<T, U> {
    /// Returns the [`Size`](crate::Size).
    pub fn size(&self) -> Size<usize> {
        self.size
    }

    // pub fn cell(&self, point: Point<usize>) -> &T {
    // ()
    // }
}

impl<S: Into<Size<usize>>, T, U: AsRef<[T]>> TryFrom<(S, U)> for Grid<T, U> {
    type Error = GridError<U>;

    fn try_from((size, elems): (S, U)) -> Result<Self, Self::Error> {
        Self::new(size, elems)
    }
}

impl<T, U: AsRef<[T]>> AsRef<[T]> for Grid<T, U> {
    fn as_ref(&self) -> &[T] {
        self.elems.as_ref()
    }
}

impl<T, U: AsMut<[T]>> AsMut<[T]> for Grid<T, U> {
    fn as_mut(&mut self) -> &mut [T] {
        self.elems.as_mut()
    }
}
