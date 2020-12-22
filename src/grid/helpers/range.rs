use std::ops::{Bound, Range, RangeBounds};

/// Nicifies `range`.
///
/// Sort, bound and saturate.
pub fn nice_range(max: usize, range: impl RangeBounds<usize>) -> Range<usize> {
    let clamp = |x: &usize| if *x < max { *x } else { max };
    let add_clamp = |x: &usize| clamp(&x.saturating_add(1));

    let (start, end) = match (range.start_bound(), range.end_bound()) {
        (Bound::Included(start), Bound::Included(end)) =>
            if start <= end {
                (clamp(start), add_clamp(end))
            } else {
                (clamp(end), add_clamp(start))
            },
        (Bound::Included(start), Bound::Excluded(end)) =>
            if start <= end {
                (clamp(start), clamp(end))
            } else {
                (add_clamp(end), add_clamp(start))
            },
        (Bound::Included(start), Bound::Unbounded) => (clamp(start), max),
        (Bound::Excluded(start), Bound::Included(end)) =>
            if start <= end {
                (add_clamp(start), add_clamp(end))
            } else {
                (clamp(end), clamp(start))
            },
        (Bound::Excluded(start), Bound::Excluded(end)) =>
            if start < end {
                (add_clamp(start), clamp(end))
            } else {
                (
                    add_clamp(end),
                    if start == end {
                        add_clamp(start)
                    } else {
                        clamp(start)
                    },
                )
            },
        (Bound::Excluded(start), Bound::Unbounded) => (add_clamp(start), max),
        (Bound::Unbounded, Bound::Included(end)) => (0, add_clamp(end)),
        (Bound::Unbounded, Bound::Excluded(end)) => (0, clamp(end)),
        (Bound::Unbounded, Bound::Unbounded) => (0, max),
    };

    Range { start, end }
}

#[cfg(test)]
mod tests {
    use super::nice_range;
    use pretty_assertions::assert_eq;
    use std::ops::{
        Bound::{self, *},
        Range,
    };

    #[test]
    fn saturate() {
        const MAX: usize = usize::MAX;

        fn assert(a: Bound<usize>, b: Bound<usize>, expected: Range<usize>) {
            assert_eq!(
                nice_range(MAX, (a, b)),
                expected,
                "({:?}, {:?}) -> {:?}",
                a,
                b,
                expected
            );
        }

        assert(Included(1), Included(MAX), 1..MAX);
        assert(Excluded(1), Included(MAX), 2..MAX);
        assert(Unbounded, Included(MAX), 0..MAX);
        assert(Included(MAX), Included(1), 1..MAX);
        assert(Included(MAX), Excluded(1), 2..MAX);
        assert(Included(MAX), Unbounded, MAX..MAX);
    }

    #[test]
    fn bounded_and_sorted() {
        fn assert(max: usize, a: Bound<usize>, b: Bound<usize>) {
            let Range { start, end } = nice_range(max, (a, b));

            assert!(start <= end, "start ({:?}) <= end ({:?})", start, end);
            assert!(end <= max, "end ({:?}) <= max ({:?})", end, max);
        }

        for max in 0..10 {
            assert(max, Unbounded, Unbounded);

            for start in 0..10 {
                assert(max, Included(start), Unbounded);
                assert(max, Excluded(start), Unbounded);
                assert(max, Unbounded, Included(start));
                assert(max, Unbounded, Excluded(start));

                for end in 0..10 {
                    assert(max, Included(start), Included(end));
                    assert(max, Excluded(start), Included(end));
                    assert(max, Included(start), Excluded(end));
                    assert(max, Excluded(start), Excluded(end));
                }
            }
        }
    }

    #[test]
    fn values() {
        fn assert(max: usize, r: (Bound<usize>, Bound<usize>), expected: Range<usize>) {
            let res = nice_range(max, r);
            let Range { start, end } = res;

            assert_eq!(
                res.collect::<Vec<_>>(),
                expected.collect::<Vec<_>>(),
                "{:?} -> ({:?}, {:?})",
                r,
                start,
                end
            );
        };

        let max = 10;

        for end in 1..20 {
            for start in 1..end {
                let expected = start..end.min(max);
                let ranges = [
                    (Included(start), Excluded(end)),         // [start, end)
                    (Included(start), Included(end - 1)),     // [start, end - 1]
                    (Excluded(start - 1), Excluded(end)),     // (start - 1, end)
                    (Excluded(start - 1), Included(end - 1)), // (start - 1, end - 1]
                ];

                for (a, b) in &ranges {
                    assert(max, (*a, *b), expected.clone());
                    assert(max, (*b, *a), expected.clone());
                }
            }

            let xend = end.min(max);
            let xend1 = (end + 1).min(max);
            assert(max, (Included(end), Excluded(end)), end..xend);
            assert(max, (Included(end), Included(end)), end..xend1);
            assert(max, (Excluded(end), Excluded(end)), end + 1..xend);
            assert(max, (Excluded(end), Included(end)), end + 1..xend1);
        }
    }
}
