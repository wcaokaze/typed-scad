use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Size(f64);

impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}mm", self.0))
    }
}

impl Add for Size {
    type Output = Size;
    fn add(self, rhs: Size) -> Size { Size(self.0 + rhs.0) }
}

impl Sub for Size {
    type Output = Size;
    fn sub(self, rhs: Size) -> Size { Size(self.0 - rhs.0) }
}

impl<Rhs> Mul<Rhs> for Size where Rhs: Into<f64> {
    type Output = Size;
    fn mul(self, rhs: Rhs) -> Size { Size(self.0 * rhs.into()) }
}

impl<Rhs> Div<Rhs> for Size where Rhs: Into<f64> {
    type Output = Size;
    fn div(self, rhs: Rhs) -> Size { Size(self.0 / rhs.into()) }
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

#[cfg(test)]
mod tests {
    use super::{Size, SizeLiteral};

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
}
