use crate::geometry::size_iterator::{
   SizeIteratorBuilder, SizeParallelIteratorBuilder
};
use crate::math::conversion::ToN64;
use crate::math::rough_fp::{rough_cmp, rough_eq};
use crate::math::unit::{Exp, Unit};
use noisy_float::prelude::*;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::{
   Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign
};

/// Size of something.
///
/// We must specify a unit to use Size.
/// And, to use `mm()`, we must `use SizeLiteral`.
/// ```
/// use typed_scad::geometry::{Size, SizeLiteral};
/// let size: Size = 1.mm();
/// ```
///
/// Basic operators are available for Size.
/// ```
/// # use typed_scad::geometry::SizeLiteral;
/// assert_eq!(1.mm() + 2.mm(), 3.mm());
/// assert_eq!(1.mm() * 2, 2.mm());
/// assert_eq!(2.mm() / 2, 1.mm());
/// assert_eq!(4.mm() / 2.mm(), 2.0);
/// ```
///
/// ## Note
/// Size implements Eq and Ord. They allows float-point arithmetic errors.
/// ```
/// # use typed_scad::geometry::SizeLiteral;
/// assert_ne!(0.1 * 3.0, 0.3);
/// assert_eq!(0.1.mm() * 3, 0.3.mm());
/// ```
#[derive(Clone, Copy, Default)]
pub struct Size(
   pub(crate) N64
);

impl Size {
   pub const ZERO: Size = Size(N64::unchecked_new(0.0));
   pub const HAIRLINE: Size = Size(N64::unchecked_new(1e-8));
   pub const INFINITY: Size = Size(N64::unchecked_new(f64::INFINITY));

   pub const fn millimeter(millimeter: N64) -> Size {
      Size(millimeter)
   }

   /// Converts this size to a N64 value as millimeter
   pub const fn to_millimeter(self) -> N64 {
      self.0
   }

   pub fn is_infinity(self) -> bool {
      self.0.is_infinite()
   }

   /// Prepare to iterate [Size]s in the specified range.
   /// And [step](SizeIteratorBuilder::step) returns an [Iterator] for Size.
   ///
   /// ```
   /// # use typed_scad::geometry::{Size, SizeLiteral};
   /// let iter = Size::iterate(0.mm()..=3.mm()).step(1.mm());
   /// assert_eq!(iter.collect::<Vec<_>>(), vec![0.mm(), 1.mm(), 2.mm(), 3.mm()]);
   /// ```
   ///
   /// Negative steps are also available.
   /// ```
   /// # use typed_scad::geometry::{Size, SizeLiteral};
   /// let iter = Size::iterate(3.mm()..=0.mm()).step(-1.mm());
   /// assert_eq!(iter.collect::<Vec<_>>(), vec![3.mm(), 2.mm(), 1.mm(), 0.mm()]);
   /// ```
   pub fn iterate<R>(size_range: R) -> SizeIteratorBuilder<R> {
      SizeIteratorBuilder(size_range)
   }

   /// similar to [iterate], but par_iterate returns a [Rayon] ParallelIterator.
   ///
   /// [iterate]: Size::iterate
   /// [Rayon]: https://docs.rs/rayon/latest/rayon/
   pub fn par_iterate<R>(size_range: R) -> SizeParallelIteratorBuilder<R> {
      SizeParallelIteratorBuilder(size_range)
   }

   pub fn abs(self) -> Size {
      Size(self.0.abs())
   }

   pub fn clamp(self, min: Size, max: Size) -> Size {
      Size(self.0.clamp(min.0, max.0))
   }
}

impl<T: ToN64> From<T> for Size {
   fn from(value: T) -> Self {
      Self(value.to_n64())
   }
}

impl Display for Size {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      write!(f, "{:.2}mm", self.0)
   }
}

impl Debug for Size {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      Display::fmt(self, f)
   }
}

impl PartialOrd for Size {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(rough_cmp(self.0, other.0))
   }
}

impl Ord for Size {
   fn cmp(&self, other: &Self) -> Ordering {
      rough_cmp(self.0, other.0)
   }
}

impl PartialEq for Size {
   fn eq(&self, other: &Self) -> bool {
      rough_eq(self.0, other.0)
   }
}

impl Eq for Size {}

impl Add for Size {
   type Output = Size;
   fn add(self, rhs: Size) -> Size {
      Size(self.0 + rhs.0)
   }
}

impl AddAssign for Size {
   fn add_assign(&mut self, rhs: Size) {
      *self = *self + rhs;
   }
}

impl Sub for Size {
   type Output = Size;
   fn sub(self, rhs: Size) -> Size {
      Size(self.0 - rhs.0)
   }
}

impl SubAssign for Size {
   fn sub_assign(&mut self, rhs: Size) {
      *self = *self - rhs;
   }
}

macro_rules! mul {
   ($($t:ty),+) => ($(
      impl Mul<$t> for Size {
         type Output = Size;
         fn mul(self, rhs: $t) -> Size {
            Size(self.0 * rhs.to_n64())
         }
      }

      impl MulAssign<$t> for Size {
         fn mul_assign(&mut self, rhs: $t) {
            *self = *self * rhs;
         }
      }

      impl Mul<Size> for $t {
         type Output = Size;
         fn mul(self, rhs: Size) -> Size {
            rhs * self
         }
      }
   )+)
}

mul!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64,
   N32, N64, R32, R64);

macro_rules! div {
   ($($t:ty),+) => ($(
      impl Div<$t> for Size {
         type Output = Size;
         fn div(self, rhs: $t) -> Size {
            Size(self.0 / rhs.to_n64())
         }
      }

      impl DivAssign<$t> for Size {
         fn div_assign(&mut self, rhs: $t) {
            *self = *self / rhs;
         }
      }
   )+)
}

div!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64,
   N32, N64, R32, R64);

impl Div for Size {
   type Output = N64;
   fn div(self, rhs: Size) -> N64 {
      self.0 / rhs.0
   }
}

impl Neg for Size {
   type Output = Size;
   fn neg(self) -> Size {
      Size(-self.0)
   }
}

impl Unit for Size {}

impl Exp<Size, 2> {
   pub fn sqrt(self) -> Size {
      Size(self.0.sqrt())
   }
}

impl Mul<Size> for Size {
   type Output = Exp<Size, 2>;
   fn mul(self, rhs: Size) -> Exp<Size, 2> {
      unsafe { Exp::new(self.0 * rhs.0) }
   }
}

impl<const N: i32> Mul<Size> for Exp<Size, N>
   where Exp<Size, {N + 1}>: Sized
{
   type Output = Exp<Size, {N + 1}>;
   fn mul(self, rhs: Size) -> Self::Output {
      unsafe { Exp::new(self.0 * rhs.0) }
   }
}

impl<const N: i32> Div<Size> for Exp<Size, N>
   where Exp<Size, {N - 1}>: Sized
{
   type Output = Exp<Size, {N - 1}>;
   fn div(self, rhs: Size) -> Self::Output {
      unsafe { Exp::new(self.0 / rhs.0) }
   }
}

impl From<Exp<Size, 0>> for N64 {
   fn from(exp: Exp<Size, 0>) -> N64 {
      exp.0
   }
}

impl From<Exp<Size, 1>> for Size {
   fn from(exp: Exp<Size, 1>) -> Size {
      Size(exp.0)
   }
}

/// Type that can make [Size] with `mm()` postfix.
///
/// Rust's primitive numbers are SizeLiteral.
/// ```
/// # use typed_scad::geometry::SizeLiteral;
/// 1.mm();
/// 2.0.mm();
/// ```
pub trait SizeLiteral {
   fn mm(self) -> Size;
   fn cm(self) -> Size;
}

macro_rules! size_literal {
   ($($t:ty),+) => ($(
      impl SizeLiteral for $t {
         fn mm(self) -> Size {
            Size(self.to_n64())
         }

         fn cm(self) -> Size {
            Size((self.to_n64()) * 10.0)
         }
      }
   )+)
}

size_literal!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128,
   f32, f64, N32, N64, R32, R64);

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
   use super::{Size, SizeLiteral};
   use noisy_float::prelude::*;
   use std::cmp::Ordering;

   #[test]
   fn eq() {
      assert_eq!(Size::from(42.0), Size::from(42.0));
      assert_ne!(Size::from(42.0), Size::from(43.0));

      assert_ne!(           42.0,             42.0 + 1e-12);
      assert_eq!(Size::from(42.0), Size::from(42.0 + 1e-12));
      assert_ne!(           42.0,             42.0 - 1e-12);
      assert_eq!(Size::from(42.0), Size::from(42.0 - 1e-12));
   }

   #[test]
   fn display() {
      assert_eq!(
         format!("{}", Size::from(42.0)),
         "42.00mm".to_string()
      );
   }

   #[test]
   fn size_literal() {
      assert_eq!(42.mm(), Size::from(42.0));
      assert_eq!(42.cm(), Size::from(420.0));
      assert_eq!(42.0.mm(), Size::from(42.0));
      assert_eq!(42.0.cm(), Size::from(420.0));
   }

   #[test]
   fn to_millimeter() {
      assert_eq!(Size::from(42.0).to_millimeter(), n64(42.0));
   }

   #[test]
   fn operators() {
      assert_eq!(Size::from( 42.0) + Size::from( 1.5), Size::from(43.5));
      assert_eq!(Size::from( 42.0) + Size::from(-1.5), Size::from(40.5));
      assert_eq!(Size::from(-42.0) + Size::from( 1.5), Size::from(-40.5));
      assert_eq!(Size::from(-42.0) + Size::from(-1.5), Size::from(-43.5));

      assert_eq!(Size::from( 42.0) - Size::from( 1.5), Size::from(40.5));
      assert_eq!(Size::from( 42.0) - Size::from(-1.5), Size::from(43.5));
      assert_eq!(Size::from(-42.0) - Size::from( 1.5), Size::from(-43.5));
      assert_eq!(Size::from(-42.0) - Size::from(-1.5), Size::from(-40.5));

      assert_eq!(Size::from( 42.0) *  2, Size::from( 84.0));
      assert_eq!(Size::from( 42.0) * -2, Size::from(-84.0));
      assert_eq!(Size::from(-42.0) *  2, Size::from(-84.0));
      assert_eq!(Size::from(-42.0) * -2, Size::from( 84.0));
      assert_eq!(Size::from( 42.0) *  1.5, Size::from( 63.0));
      assert_eq!(Size::from( 42.0) * -1.5, Size::from(-63.0));
      assert_eq!(Size::from(-42.0) *  1.5, Size::from(-63.0));
      assert_eq!(Size::from(-42.0) * -1.5, Size::from( 63.0));

      assert_eq!( 2 * Size::from( 42.0), Size::from( 84.0));
      assert_eq!(-2 * Size::from( 42.0), Size::from(-84.0));
      assert_eq!( 2 * Size::from(-42.0), Size::from(-84.0));
      assert_eq!(-2 * Size::from(-42.0), Size::from( 84.0));
      assert_eq!( 1.5 * Size::from( 42.0), Size::from( 63.0));
      assert_eq!(-1.5 * Size::from( 42.0), Size::from(-63.0));
      assert_eq!( 1.5 * Size::from(-42.0), Size::from(-63.0));
      assert_eq!(-1.5 * Size::from(-42.0), Size::from( 63.0));

      assert_eq!(Size::from( 42.0) /  2, Size::from( 21.0));
      assert_eq!(Size::from( 42.0) / -2, Size::from(-21.0));
      assert_eq!(Size::from(-42.0) /  2, Size::from(-21.0));
      assert_eq!(Size::from(-42.0) / -2, Size::from( 21.0));
      assert_eq!(Size::from( 42.0) /  1.5, Size::from( 28.0));
      assert_eq!(Size::from( 42.0) / -1.5, Size::from(-28.0));
      assert_eq!(Size::from(-42.0) /  1.5, Size::from(-28.0));
      assert_eq!(Size::from(-42.0) / -1.5, Size::from( 28.0));

      assert_eq!(Size::from( 42.0) / Size::from( 1.5), n64( 28.0));
      assert_eq!(Size::from( 42.0) / Size::from(-1.5), n64(-28.0));
      assert_eq!(Size::from(-42.0) / Size::from( 1.5), n64(-28.0));
      assert_eq!(Size::from(-42.0) / Size::from(-1.5), n64( 28.0));

      assert_eq!(-Size::from(42.0), Size::from(-42.0));

      assert!(Size::from(42.0) > Size::from(41.0));
      assert!(Size::from(41.0) < Size::from(42.0));

      assert_eq!(
         Size::from(42.0).partial_cmp(&Size::from(42.0)),
         Some(Ordering::Equal)
      );
      assert_eq!(
         Size::from(42.0).partial_cmp(&Size::from(42.0 + 1e-12)),
         Some(Ordering::Equal)
      );
      assert_eq!(
         Size::from(42.0).partial_cmp(&Size::from(42.0 - 1e-12)),
         Some(Ordering::Equal)
      );
      assert_eq!(
         Size::from(42.0).cmp(&Size::from(42.0)),
         Ordering::Equal
      );
      assert_eq!(
         Size::from(42.0).cmp(&Size::from(42.0 + 1e-12)),
         Ordering::Equal
      );
      assert_eq!(
         Size::from(42.0).cmp(&Size::from(42.0 - 1e-12)),
         Ordering::Equal
      );
   }

   #[test]
   fn sum() {
      let sum: Size = (1..=10)
         .into_iter()
         .map(|i| Size::from(i))
         .sum();

      assert_eq!(sum, Size::from(55.0));
   }
}
