use super::*;
use std::ops::{Bound, Range, RangeBounds};

// ========= //
// CellIndex //
// ========= //

pub trait CellIndex {
    fn index(self, size: Size<usize>) -> Point<usize>;
}

impl CellIndex for Point<usize> {
    fn index(self, _: Size<usize>) -> Point<usize> {
        self.into()
    }
}

impl<F: Fn(Size<usize>) -> Point<usize>> CellIndex for F {
    fn index(self, size: Size<usize>) -> Point<usize> {
        self(size)
    }
}

// ======== //
// RowIndex //
// ======== //

pub trait RowIndex {
    fn index(self, size: Size<usize>) -> (usize, Range<usize>);
}

impl RowIndex for usize {
    fn index(self, size: Size<usize>) -> (usize, Range<usize>) {
        (self, Range {
            start: 0,
            end:   size.width,
        })
    }
}

impl<T: RangeBounds<usize>> RowIndex for (usize, T) {
    fn index(self, size: Size<usize>) -> (usize, Range<usize>) {
        (self.0, bound_range_end(self.1, size.width))
    }
}

// ViewIndex

pub trait ViewIndex {
    fn index(self, size: Size<usize>) -> (Point<usize>, Size<usize>);
}

// Into<Rect>
// Fn(Size) -> Into<Rect>
// (RowIndex, ColIndex)

fn bound_range_end(range: impl RangeBounds<usize>, end_bound: usize) -> Range<usize> {
    Range {
        start: match range.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start + 1,
            Bound::Unbounded => 0,
        },
        end:   match range.end_bound() {
            Bound::Included(end) => *end + 1,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => end_bound,
        },
    }
}
