use std::fmt::{self, Display, Formatter};
use std::iter::Sum;
use std::ops::{
    Add, AddAssign, Bound, Div, DivAssign, Mul, MulAssign, Neg, Range,
    RangeBounds, RangeFrom, RangeInclusive, Sub, SubAssign
};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Size(f64);

impl Size {
    pub fn iterate<R>(size_range: R) -> R where R: IterableSizeRange {
        size_range
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}mm", self.0))
    }
}

impl Add for Size {
    type Output = Size;
    fn add(self, rhs: Size) -> Size { Size(self.0 + rhs.0) }
}

impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Size) {
        *self = *self + rhs;
    }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, rhs: Size) -> Size { Size(self.0 - rhs.0) }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, rhs: Size) {
        *self = *self - rhs;
    }
}

impl<Rhs> Mul<Rhs> for Size where Rhs: Into<f64> {
    type Output = Size;
    fn mul(self, rhs: Rhs) -> Size { Size(self.0 * rhs.into()) }
}

impl<Rhs> MulAssign<Rhs> for Size where Rhs: Into<f64> {
    fn mul_assign(&mut self, rhs: Rhs) {
        *self = *self * rhs;
    }
}

impl<Rhs> Div<Rhs> for Size where Rhs: Into<f64> {
    type Output = Size;
    fn div(self, rhs: Rhs) -> Size { Size(self.0 / rhs.into()) }
}

impl<Rhs> DivAssign<Rhs> for Size where Rhs: Into<f64> {
    fn div_assign(&mut self, rhs: Rhs) {
        *self = *self / rhs;
    }
}

impl Div for Size {
    type Output = f64;
    fn div(self, rhs: Size) -> f64 { self.0 / rhs.0 }
}

impl Neg for Size {
    type Output = Size;
    fn neg(self) -> Size { Size(-self.0) }
}

trait SizeLiteral {
    fn mm(self) -> Size;
    fn cm(self) -> Size;
}

impl<T> SizeLiteral for T
    where T: Into<f64> + From<i32> + Mul<T>,
          T::Output: Into<f64>
{
    fn mm(self) -> Size {
        Size(self.into())
    }

    fn cm(self) -> Size {
        Size((self * From::from(10)).into())
    }
}

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
        if step > Size(0.0) {
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
        let mut sum = Size(0.0);
        for s in iter {
            sum += s;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::{IterableSizeRange, Size, SizeLiteral};

    #[test]
    fn eq() {
        assert_eq!(Size(42.0), Size(42.0));
        assert_ne!(Size(42.0), Size(43.0));
    }

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", Size(42.0)),
            "42mm".to_string()
        );
    }

    #[test]
    fn size_literal() {
        assert_eq!(42.mm(), Size(42.0));
        assert_eq!(42.cm(), Size(420.0));
        assert_eq!(42.0.mm(), Size(42.0));
        assert_eq!(42.0.cm(), Size(420.0));
    }

    #[test]
    fn operators() {
        assert_eq!(Size( 42.0) + Size( 1.5), Size(43.5));
        assert_eq!(Size( 42.0) + Size(-1.5), Size(40.5));
        assert_eq!(Size(-42.0) + Size( 1.5), Size(-40.5));
        assert_eq!(Size(-42.0) + Size(-1.5), Size(-43.5));

        assert_eq!(Size( 42.0) - Size( 1.5), Size(40.5));
        assert_eq!(Size( 42.0) - Size(-1.5), Size(43.5));
        assert_eq!(Size(-42.0) - Size( 1.5), Size(-43.5));
        assert_eq!(Size(-42.0) - Size(-1.5), Size(-40.5));

        assert_eq!(Size( 42.0) *  2, Size( 84.0));
        assert_eq!(Size( 42.0) * -2, Size(-84.0));
        assert_eq!(Size(-42.0) *  2, Size(-84.0));
        assert_eq!(Size(-42.0) * -2, Size( 84.0));
        assert_eq!(Size( 42.0) *  1.5, Size( 63.0));
        assert_eq!(Size( 42.0) * -1.5, Size(-63.0));
        assert_eq!(Size(-42.0) *  1.5, Size(-63.0));
        assert_eq!(Size(-42.0) * -1.5, Size( 63.0));

        assert_eq!(Size( 42.0) /  2, Size( 21.0));
        assert_eq!(Size( 42.0) / -2, Size(-21.0));
        assert_eq!(Size(-42.0) /  2, Size(-21.0));
        assert_eq!(Size(-42.0) / -2, Size( 21.0));
        assert_eq!(Size( 42.0) /  1.5, Size( 28.0));
        assert_eq!(Size( 42.0) / -1.5, Size(-28.0));
        assert_eq!(Size(-42.0) /  1.5, Size(-28.0));
        assert_eq!(Size(-42.0) / -1.5, Size( 28.0));

        assert_eq!(Size( 42.0) / Size( 1.5),  28.0);
        assert_eq!(Size( 42.0) / Size(-1.5), -28.0);
        assert_eq!(Size(-42.0) / Size( 1.5), -28.0);
        assert_eq!(Size(-42.0) / Size(-1.5),  28.0);

        assert_eq!(-Size(42.0), Size(-42.0));

        assert!(Size(42.0) > Size(41.0));
        assert!(Size(41.0) < Size(42.0));
    }

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
