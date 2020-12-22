// use crate::*;
// use std::ops::{Bound, Range, RangeBounds};

pub enum RowMajor {}
pub enum ColumnMajor {}

/*
impl RowMajor {
    pub fn cell(size: Size<usize>, cell: Point<usize>) -> Option<usize> {
        if cell < size.as_point() {
            Some(Self::cell_unchecked(size, cell))
        } else {
            None
        }
    }

    pub fn cell_unchecked(size: Size<usize>, cell: Point<usize>) -> usize {
        cell.y * size.width + cell.x
    }

    pub fn row(
        size: Size<usize>,
        row: usize,
        range: impl RangeBounds<usize>,
    ) -> Option<Range<usize>> {
        if row < size.height {
            let start = match range.start_bound() {
                Bound::Included(start) => *start,
                Bound::Excluded(start) => start.checked_add(1)?,
                Bound::Unbounded => 0,
            };
            let end = match range.end_bound() {
                Bound::Included(end) => end.checked_add(1)?,
                Bound::Excluded(end) => *end,
                Bound::Unbounded => size.width,
            };

            if start > end || end > size.width {
                return None;
            }

            Some(Self::row_unchecked(size, row, Range { start, end }))
        } else {
            None
        }
    }

    pub fn row_unchecked(size: Size<usize>, row: usize, range: Range<usize>) -> Range<usize> {
        let start = Self::cell_unchecked(size, Point {
            x: range.start,
            y: row,
        });

        Range {
            start,
            end: start + range.end,
        }
    }
}
*/
