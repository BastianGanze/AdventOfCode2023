use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum RangeRelation<T: Ord + PartialOrd + Copy> {
    NoIntersect,
    Includes,
    Intersect(Range<T>, Range<T>),
    IntersectTwice(Range<T>, Range<T>, Range<T>),
}

pub fn check_range_overlap<T: Ord + PartialOrd + Copy>(
    a: &Range<T>,
    b: &Range<T>,
) -> RangeRelation<T> {
    if a.end <= b.start || b.end <= a.start {
        // No intersection
        RangeRelation::NoIntersect
    } else if b.start <= a.start && b.end >= a.end {
        // range2 (B) includes range1 (A)
        RangeRelation::Includes
    } else if (a.start < b.start && a.end > b.end) || (b.start < a.start && b.end > a.end) {
        // range1 or range2 starts before the other and ends after the other, intersecting twice
        RangeRelation::IntersectTwice(
            a.start.min(b.start)..a.start.max(b.start),
            a.start.max(b.start)..a.end.min(b.end),
            a.end.min(b.end)..a.end.max(b.end),
        )
    } else {
        // All other cases - single intersection
        let outside_part = if a.start < b.start {
            a.start..b.start
        } else {
            b.end..a.end
        };
        let inside_part = a.start.max(b.start)..a.end.min(b.end);

        RangeRelation::Intersect(outside_part, inside_part)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::check_range_overlap::{check_range_overlap, RangeRelation};

    #[test]
    pub fn no_intersect() {
        assert_eq!(
            check_range_overlap(&(1..5), &(6..10)),
            RangeRelation::NoIntersect
        );
        assert_eq!(
            check_range_overlap(&(1..5), &(5..10)),
            RangeRelation::NoIntersect
        );
        assert_eq!(
            check_range_overlap(&(6..10), &(1..3)),
            RangeRelation::NoIntersect
        );
        assert_eq!(
            check_range_overlap(&(6..10), &(1..6)),
            RangeRelation::NoIntersect
        );
    }

    #[test]
    pub fn includes() {
        assert_eq!(
            check_range_overlap(&(3..7), &(1..10)),
            RangeRelation::Includes
        );
    }

    #[test]
    pub fn intersect() {
        assert_eq!(
            check_range_overlap(&(1..5), &(3..7)),
            RangeRelation::Intersect(1..3, 3..5)
        );
        assert_eq!(
            check_range_overlap(&(3..7), &(1..5)),
            RangeRelation::Intersect(5..7, 3..5)
        );
        assert_eq!(
            check_range_overlap(&(5..9), &(3..7)),
            RangeRelation::Intersect(7..9, 5..7)
        );
        assert_eq!(
            check_range_overlap(&(3..7), &(5..9)),
            RangeRelation::Intersect(3..5, 5..7)
        );
    }

    #[test]
    pub fn intersect_twice() {
        assert_eq!(
            check_range_overlap(&(1..7), &(3..5)),
            RangeRelation::IntersectTwice(1..3, 3..5, 5..7)
        );
        assert_eq!(
            check_range_overlap(&(1..7), &(3..7)),
            RangeRelation::Intersect(1..3, 3..7)
        );
        assert_eq!(
            check_range_overlap(&(3..9), &(3..7)),
            RangeRelation::Intersect(7..9, 3..7)
        );
    }
}
