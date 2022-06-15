use std::cmp::Ordering;
use super::IterableSizeRange;
use std::fmt::{self, Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign
};

/// Size of something.
///
/// We must specify a unit to use Size.
/// And, to use `mm()`, we must `use SizeLiteral`.
/// ```
/// use typed_scad::geometory::{Size, SizeLiteral};
/// let size: Size = 1.mm();
/// ```
///
/// Basic operators are available for Size.
/// ```
/// # use typed_scad::geometory::SizeLiteral;
/// assert_eq!(1.mm() + 2.mm(), 3.mm());
/// assert_eq!(1.mm() * 2, 2.mm());
/// assert_eq!(2.mm() / 2, 1.mm());
/// assert_eq!(4.mm() / 2.mm(), 2.0);
/// ```
///
/// ## Note
/// Size implements PartialEq and PartialOrd.
/// They allows float-point arithmetic errors.
/// ```
/// # use typed_scad::geometory::SizeLiteral;
/// assert_ne!(0.1 * 3.0, 0.3);
/// assert_eq!(0.1.mm() * 3, 0.3.mm());
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Size(f64);

impl Size {
    /// Converts this size to a f64 value as millimeter
    pub fn to_millimeter(&self) -> f64 {
        self.0
    }

    /// Prepare to iterate [Size]s in the specified range.
    /// And [step](IterableSizeRange::step) returns an [Iterator] for Size.
    ///
    /// ```
    /// # use typed_scad::geometory::{IterableSizeRange, Size, SizeLiteral};
    /// let iter = Size::iterate(0.mm()..=3.mm()).step(1.mm());
    /// assert_eq!(iter.collect::<Vec<_>>(), vec![0.mm(), 1.mm(), 2.mm(), 3.mm()]);
    /// ```
    ///
    /// Negative steps are also available.
    /// ```
    /// # use typed_scad::geometory::{IterableSizeRange, Size, SizeLiteral};
    /// let iter = Size::iterate(3.mm()..=0.mm()).step(-1.mm());
    /// assert_eq!(iter.collect::<Vec<_>>(), vec![3.mm(), 2.mm(), 1.mm(), 0.mm()]);
    /// ```
    pub fn iterate<R>(size_range: R) -> R where R: IterableSizeRange {
        size_range
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}mm", self.0))
    }
}

const D: f64 = 1e-10;

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.0 < other.0 + D, self.0 > other.0 - D) {
            (false, false) => None,
            (false,  true) => Some(Ordering::Greater),
            ( true, false) => Some(Ordering::Less),
            ( true,  true) => Some(Ordering::Equal)
        }
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.0 > other.0 - D && self.0 < other.0 + D
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

/// Type that can make [Size] with `mm()` postfix.
///
/// Rust's primitive numbers are SizeLiteral.
/// ```
/// # use typed_scad::geometory::SizeLiteral;
/// 1.mm();
/// 2.0.mm();
/// ```
pub trait SizeLiteral {
    fn mm(self) -> Size;
    fn cm(self) -> Size;
}

impl<T> SizeLiteral for T where T: Into<f64> {
    fn mm(self) -> Size {
        Size(self.into())
    }

    fn cm(self) -> Size {
        Size(self.into() * 10.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{Size, SizeLiteral};
    use std::cmp::Ordering;

    #[test]
    fn eq() {
        assert_eq!(Size(42.0), Size(42.0));
        assert_ne!(Size(42.0), Size(43.0));

        assert_ne!(     42.0,       42.0 + 1e-12);
        assert_eq!(Size(42.0), Size(42.0 + 1e-12));
        assert_ne!(     42.0,       42.0 - 1e-12);
        assert_eq!(Size(42.0), Size(42.0 - 1e-12));
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
    fn to_millimeter() {
        assert_eq!(Size(42.0).to_millimeter(), 42.0);
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

        assert_eq!(
            Size(42.0).partial_cmp(&Size(f64::NAN)),
            None
        );
        assert_eq!(
            Size(f64::NAN).partial_cmp(&Size(42.0)),
            None
        );
        assert_eq!(
            Size(f64::NAN).partial_cmp(&Size(f64::NAN)),
            None
        );
        assert_eq!(
            Size(42.0).partial_cmp(&Size(42.0)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Size(42.0).partial_cmp(&Size(42.0 + 1e-12)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            Size(42.0).partial_cmp(&Size(42.0 - 1e-12)),
            Some(Ordering::Equal)
        );
    }
}
