use super::{Size, SizeLiteral};
use std::iter::Sum;
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeInclusive};

pub struct SizeIterator {
    next: Option<Size>,
    step: Size,
    end_bound: Bound<Size>
}

impl SizeIterator {
    fn new(
        start_value: Size, step: Size, end_bound: Bound<Size>
    ) -> SizeIterator {
        let is_iterable = Self::is_available_value(start_value, end_bound, step);

        SizeIterator {
            next: if is_iterable { Some(start_value) } else { None },
            step: step,
            end_bound: end_bound
        }
    }

    fn calc_next(&self, current: Size) -> Option<Size> {
        let next = current + self.step;
        let has_next = Self::is_available_value(next, self.end_bound, self.step);
        if has_next { Some(next) } else { None }
    }

    fn is_available_value(
        value: Size,
        end_bound: Bound<Size>,
        step: Size
    ) -> bool {
        match end_bound {
            Bound::Included(end) => Self::is_not_ended(value, end + step / 16, step),
            Bound::Excluded(end) => Self::is_not_ended(value, end,             step),
            Bound::Unbounded => true
        }
    }

    fn is_not_ended(value: Size, end_value: Size, step: Size) -> bool {
        if step > 0.mm() {
            value < end_value
        } else {
            value > end_value
        }
    }
}

impl Iterator for SizeIterator {
    type Item = Size;

    fn next(&mut self) -> Option<Size> {
        let next = self.next?;
        self.next = self.calc_next(next);
        Some(next)
    }
}

pub trait IterableSizeRange: RangeBounds<Size> {
    fn start_value(&self) -> Size;

    fn step(self, step: Size) -> SizeIterator where Self: Sized {
        SizeIterator::new(
            self.start_value(),
            step,
            self.end_bound().cloned()
        )
    }
}

impl IterableSizeRange for Range<Size> {
    fn start_value(&self) -> Size {
        self.start
    }
}

impl IterableSizeRange for RangeInclusive<Size> {
    fn start_value(&self) -> Size {
        *self.start()
    }
}

impl IterableSizeRange for RangeFrom<Size> {
    fn start_value(&self) -> Size {
        self.start
    }
}

impl Sum for Size {
    fn sum<I>(iter: I) -> Size where I: Iterator<Item = Size> {
        let mut sum = 0.mm();
        for s in iter {
            sum += s;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Size, SizeLiteral};
    use super::IterableSizeRange;

    #[test]
    fn iterate() {
        let expected = vec![42.mm(), 43.5.mm(), 45.mm()];
        let actual: Vec<_> = Size::iterate(42.mm()..=45.mm()).step(1.5.mm())
            .collect();
        assert_eq!(actual, expected);

        let expected = vec![42.mm(), 43.5.mm()];
        let actual: Vec<_> = Size::iterate(42.mm()..45.mm()).step(1.5.mm())
            .collect();
        assert_eq!(actual, expected);

        let expected = vec![42.mm(), 43.5.mm(), 45.mm()];
        let actual: Vec<_> = Size::iterate(42.mm()..).step(1.5.mm())
            .take(3)
            .collect();
        assert_eq!(actual, expected);

        let actual: Vec<_> = Size::iterate(45.mm()..=42.mm()).step(1.5.mm())
            .collect();
        assert_eq!(actual, vec![]);

        let actual: Vec<_> = Size::iterate(45.mm()..42.mm()).step(1.5.mm())
            .collect();
        assert_eq!(actual, vec![]);
    }

    #[test]
    fn iterate_down() {
        let expected = vec![45.mm(), 43.5.mm(), 42.mm()];
        let actual: Vec<_> = Size::iterate(45.mm()..=42.mm()).step(-1.5.mm())
            .collect();
        assert_eq!(actual, expected);

        let expected = vec![45.mm(), 43.5.mm()];
        let actual: Vec<_> = Size::iterate(45.mm()..42.mm()).step(-1.5.mm())
            .collect();
        assert_eq!(actual, expected);

        let expected = vec![45.mm(), 43.5.mm(), 42.mm()];
        let actual: Vec<_> = Size::iterate(45.mm()..).step(-1.5.mm())
            .take(3)
            .collect();
        assert_eq!(actual, expected);

        let actual: Vec<_> = Size::iterate(42.mm()..=45.mm()).step(-1.5.mm())
            .collect();
        assert_eq!(actual, vec![]);

        let actual: Vec<_> = Size::iterate(42.mm()..45.mm()).step(-1.5.mm())
            .collect();
        assert_eq!(actual, vec![]);
    }

    #[test]
    fn sum_by_iter() {
        let sum: Size = Size::iterate(1.mm()..=10.mm()).step(1.mm()).sum();
        assert_eq!(sum, 55.mm());
    }
}
