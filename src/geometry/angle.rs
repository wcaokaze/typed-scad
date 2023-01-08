use crate::geometry::angle_iterator::{
   AngleIteratorBuilder, AngleParallelIteratorBuilder
};
use crate::geometry::Size;
use crate::math::conversion::ToN64;
use crate::math::rough_fp::{rough_cmp, rough_eq};
use crate::math::unit::{Exp, Unit};
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::{
   Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign
};
use noisy_float::prelude::*;

/// Angle.
///
/// We must specify a unit to use Angle.
/// And, to use `deg()`, we must `use AngleLiteral`.
/// ```
/// use typed_scad::geometry::{Angle, AngleLiteral};
/// let angle: Angle = 90.deg();
/// ```
///
/// Basic operators are available for Angle.
/// ```
/// # use typed_scad::geometry::AngleLiteral;
/// assert_eq!(1.deg() + 2.deg(), 3.deg());
/// assert_eq!(1.deg() * 2, 2.deg());
/// assert_eq!(2.deg() / 2, 1.deg());
/// assert_eq!(4.deg() / 2.deg(), 2.0);
/// ```
///
/// ### Note
/// Angle implements Eq and Ord. They allows float-point arithmetic errors.
/// ```
/// # use std::f64::consts::PI;
/// # use typed_scad::geometry::AngleLiteral;
/// assert_ne!(
///    PI + 1.0_f64.to_radians(),
///    181.0_f64.to_radians()
/// );
///
/// assert_eq!(
///    PI.rad() + 1.deg(),
///    181.deg()
/// );
/// ```
///
/// And also note that they don't consider circling.
/// ```
/// # use typed_scad::geometry::AngleLiteral;
/// assert_ne!(0.deg(), 360.deg());
/// ```
#[derive(Clone, Copy, Default)]
pub struct Angle(
   pub(crate) N64
);

pub fn sin(angle: Angle) -> N64 {
   angle.sin()
}

pub fn cos(angle: Angle) -> N64 {
   angle.cos()
}

pub fn tan(angle: Angle) -> N64 {
   angle.tan()
}

pub fn asin(a: N64) -> Angle {
   Angle::asin(a)
}

pub fn acos(a: N64) -> Angle {
   Angle::acos(a)
}

pub fn atan(a: N64) -> Angle {
   Angle::atan(a)
}

pub fn atan2(y: Size, x: Size) -> Angle {
   Angle::atan2(y, x)
}

impl Angle {
   /// PI radian. But `Angle::PI` is not enough readable.
   /// Also consider using `180.deg()`
   pub const PI: Angle = Angle(N64::unchecked_new(std::f64::consts::PI));

   pub const fn radian(radian: N64) -> Angle {
      Angle(radian)
   }

   /// Converts this angle to a N64 value as radian
   pub const fn to_radian(self) -> N64 {
      self.0
   }

   /// Converts this angle to a N64 value as degree
   pub fn to_degree(self) -> N64 {
      self.0.to_degrees()
   }

   pub fn sin(self) -> N64 {
      self.0.sin()
   }

   pub fn cos(self) -> N64 {
      self.0.cos()
   }

   pub fn tan(self) -> N64 {
      self.0.tan()
   }

   pub fn sin_cos(self) -> (N64, N64) {
      self.0.sin_cos()
   }

   pub fn asin(a: N64) -> Angle {
      Angle(N64::asin(a))
   }

   pub fn acos(a: N64) -> Angle {
      Angle(N64::acos(a))
   }

   pub fn atan(a: N64) -> Angle {
      Angle(N64::atan(a))
   }

   pub fn atan2(y: Size, x: Size) -> Angle {
      Angle(N64::atan2(y.0, x.0))
   }

   pub fn abs(self) -> Angle {
      Angle(self.0.abs())
   }

   pub fn clamp(self, min: Angle, max: Angle) -> Angle {
      Angle(self.0.clamp(min.0, max.0))
   }

   /// Prepare to iterate [Angle]s in the specified range.
   /// And [step][AngleIteratorBuilder::step] returns an [Iterator] for Angle.
   ///
   /// ```
   /// # use typed_scad::geometry::{Angle, AngleLiteral};
   /// let iter = Angle::iterate(0.deg()..=3.deg()).step(1.deg());
   /// assert_eq!(iter.collect::<Vec<_>>(), vec![0.deg(), 1.deg(), 2.deg(), 3.deg()]);
   /// ```
   ///
   /// Negative steps are also available.
   /// ```
   /// # use typed_scad::geometry::{Angle, AngleLiteral};
   /// let iter = Angle::iterate(3.deg()..=0.deg()).step(-1.deg());
   /// assert_eq!(iter.collect::<Vec<_>>(), vec![3.deg(), 2.deg(), 1.deg(), 0.deg()]);
   /// ```
   pub fn iterate<R>(angle_range: R) -> AngleIteratorBuilder<R> {
      AngleIteratorBuilder(angle_range)
   }

   /// similar to [iterate], but par_iterate returns a [Rayon] ParallelIterator.
   ///
   /// [iterate]: Angle::iterate
   /// [Rayon]: https://docs.rs/rayon/latest/rayon/
   pub fn par_iterate<R>(angle_range: R) -> AngleParallelIteratorBuilder<R> {
      AngleParallelIteratorBuilder(angle_range)
   }
}

impl<T: ToN64> From<T> for Angle {
   fn from(value: T) -> Self {
      Self(value.to_n64())
   }
}

impl Display for Angle {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      write!(f, "{:.2}°", self.0.to_degrees())
   }
}

impl Debug for Angle {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      Display::fmt(self, f)
   }
}

impl PartialOrd for Angle {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(rough_cmp(self.0, other.0))
   }
}

impl Ord for Angle {
   fn cmp(&self, other: &Self) -> Ordering {
      rough_cmp(self.0, other.0)
   }
}

impl PartialEq for Angle {
   fn eq(&self, other: &Self) -> bool {
      rough_eq(self.0, other.0)
   }
}

impl Eq for Angle {}

impl Add for Angle {
   type Output = Angle;
   fn add(self, rhs: Angle) -> Angle {
      Angle(self.0 + rhs.0)
   }
}

impl AddAssign for Angle {
   fn add_assign(&mut self, rhs: Angle) {
      *self = *self + rhs;
   }
}

impl Sub for Angle {
   type Output = Angle;
   fn sub(self, rhs: Angle) -> Angle {
      Angle(self.0 - rhs.0)
   }
}

impl SubAssign for Angle {
   fn sub_assign(&mut self, rhs: Angle) {
      *self = *self - rhs;
   }
}

macro_rules! mul {
   ($($t:ty),+) => ($(
      impl Mul<$t> for Angle {
         type Output = Angle;
         fn mul(self, rhs: $t) -> Angle {
            Angle(self.0 * rhs.to_n64())
         }
      }

      impl MulAssign<$t> for Angle {
         fn mul_assign(&mut self, rhs: $t) {
            *self = *self * rhs;
         }
      }

      impl Mul<Angle> for $t {
         type Output = Angle;
         fn mul(self, rhs: Angle) -> Angle {
            rhs * self
         }
      }
   )+)
}

mul!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64,
   N32, N64, R32, R64);

macro_rules! div {
   ($($t:ty),+) => ($(
      impl Div<$t> for Angle {
         type Output = Angle;
         fn div(self, rhs: $t) -> Angle {
            Angle(self.0 / rhs.to_n64())
         }
      }

      impl DivAssign<$t> for Angle {
         fn div_assign(&mut self, rhs: $t) {
            *self = *self / rhs;
         }
      }
   )+)
}

div!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64,
   N32, N64, R32, R64);

impl Div for Angle {
   type Output = N64;
   fn div(self, rhs: Angle) -> N64 {
      self.0 / rhs.0
   }
}

impl Neg for Angle {
   type Output = Angle;
   fn neg(self) -> Angle {
      Angle(-self.0)
   }
}

impl Unit for Angle {}

impl From<Exp<Angle, 0>> for N64 {
   fn from(exp: Exp<Angle, 0>) -> N64 {
      exp.0
   }
}

impl From<Exp<Angle, 1>> for Angle {
   fn from(exp: Exp<Angle, 1>) -> Angle {
      Angle(exp.0)
   }
}

/// Type that can make [Angle] with `deg()` postfix.
///
/// Rust's primitive numbers are AngleLiteral.
/// ```
/// # use typed_scad::geometry::AngleLiteral;
/// 1.deg();
/// 2.0.deg();
/// ```
pub trait AngleLiteral {
   fn deg(self) -> Angle;
   fn rad(self) -> Angle;
}

macro_rules! angle_literal {
   ($($t:ty),+) => ($(
      impl AngleLiteral for $t {
         fn deg(self) -> Angle {
            Angle(self.to_n64().to_radians())
         }

         fn rad(self) -> Angle {
            Angle(self.to_n64())
         }
      }
   )+)
}

angle_literal!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128,
   f32, f64, N32, N64, R32, R64);

#[cfg(test)]
mod tests {
   use super::{Angle, AngleLiteral};
   use noisy_float::prelude::*;
   use std::cmp::Ordering;
   use std::f64::consts::PI;

   #[test]
   fn eq() {
      assert_eq!(Angle::from(0.42), Angle::from(0.42));
      assert_ne!(Angle::from(0.42), Angle::from(0.43));

      assert_ne!(            0.42,              0.42 + 1e-12);
      assert_eq!(Angle::from(0.42), Angle::from(0.42 + 1e-12));
      assert_ne!(            0.42,              0.42 - 1e-12);
      assert_eq!(Angle::from(0.42), Angle::from(0.42 - 1e-12));

      assert_ne!(Angle::from(0.42), Angle::from(0.42 + 2.0 * PI));
   }

   #[test]
   fn display() {
      assert_eq!(
         format!("{}", Angle::from(PI)),
         "180.00°".to_string()
      );
   }

   #[test]
   fn angle_literal() {
      assert_eq!(2.rad(), Angle::from(2.0));
      assert_eq!(180.deg(), Angle::from(PI));
      assert_eq!(0.42.rad(), Angle::from(0.42));
      assert_eq!(180.0.deg(), Angle::from(PI));
   }

   #[test]
   fn to_radian() {
      assert_eq!(Angle::from(0.42).to_radian(), n64(0.42));
   }

   #[test]
   fn to_degree() {
      assert_eq!(Angle::from(PI).to_degree(), n64(180.0));
   }

   #[test]
   fn operators() {
      assert_eq!(Angle::from( 0.42) + Angle::from( 0.15), Angle::from(0.57));
      assert_eq!(Angle::from( 0.42) + Angle::from(-0.15), Angle::from(0.27));
      assert_eq!(Angle::from(-0.42) + Angle::from( 0.15), Angle::from(-0.27));
      assert_eq!(Angle::from(-0.42) + Angle::from(-0.15), Angle::from(-0.57));

      assert_eq!(Angle::from( 0.42) - Angle::from( 0.15), Angle::from(0.27));
      assert_eq!(Angle::from( 0.42) - Angle::from(-0.15), Angle::from(0.57));
      assert_eq!(Angle::from(-0.42) - Angle::from( 0.15), Angle::from(-0.57));
      assert_eq!(Angle::from(-0.42) - Angle::from(-0.15), Angle::from(-0.27));

      assert_eq!(Angle::from( 0.42) *  2, Angle::from( 0.84));
      assert_eq!(Angle::from( 0.42) * -2, Angle::from(-0.84));
      assert_eq!(Angle::from(-0.42) *  2, Angle::from(-0.84));
      assert_eq!(Angle::from(-0.42) * -2, Angle::from( 0.84));
      assert_eq!(Angle::from( 0.42) *  1.5, Angle::from( 0.63));
      assert_eq!(Angle::from( 0.42) * -1.5, Angle::from(-0.63));
      assert_eq!(Angle::from(-0.42) *  1.5, Angle::from(-0.63));
      assert_eq!(Angle::from(-0.42) * -1.5, Angle::from( 0.63));

      assert_eq!( 2 * Angle::from( 0.42), Angle::from( 0.84));
      assert_eq!(-2 * Angle::from( 0.42), Angle::from(-0.84));
      assert_eq!( 2 * Angle::from(-0.42), Angle::from(-0.84));
      assert_eq!(-2 * Angle::from(-0.42), Angle::from( 0.84));
      assert_eq!( 1.5 * Angle::from( 0.42), Angle::from( 0.63));
      assert_eq!(-1.5 * Angle::from( 0.42), Angle::from(-0.63));
      assert_eq!( 1.5 * Angle::from(-0.42), Angle::from(-0.63));
      assert_eq!(-1.5 * Angle::from(-0.42), Angle::from( 0.63));

      assert_eq!(Angle::from( 0.42) /  2, Angle::from( 0.21));
      assert_eq!(Angle::from( 0.42) / -2, Angle::from(-0.21));
      assert_eq!(Angle::from(-0.42) /  2, Angle::from(-0.21));
      assert_eq!(Angle::from(-0.42) / -2, Angle::from( 0.21));
      assert_eq!(Angle::from( 0.42) /  1.5, Angle::from( 0.28));
      assert_eq!(Angle::from( 0.42) / -1.5, Angle::from(-0.28));
      assert_eq!(Angle::from(-0.42) /  1.5, Angle::from(-0.28));
      assert_eq!(Angle::from(-0.42) / -1.5, Angle::from( 0.28));

      // 0.42 causes a float-point arithmetic error. We should use 2⁻ⁿ here
      assert_eq!(Angle::from( 0.25) / Angle::from( 0.5), n64( 0.5));
      assert_eq!(Angle::from( 0.25) / Angle::from(-0.5), n64(-0.5));
      assert_eq!(Angle::from(-0.25) / Angle::from( 0.5), n64(-0.5));
      assert_eq!(Angle::from(-0.25) / Angle::from(-0.5), n64( 0.5));

      assert_eq!(-Angle::from(0.42), Angle::from(-0.42));

      assert!(Angle::from(0.42) > Angle::from(0.41));
      assert!(Angle::from(0.41) < Angle::from(0.42));
      assert!(Angle::from(0.42) < Angle::from(0.42 + 2.0 * PI));

      assert_eq!(
         Angle::from(0.42).partial_cmp(&Angle::from(0.42)),
         Some(Ordering::Equal)
      );
      assert_eq!(
         Angle::from(0.42).partial_cmp(&Angle::from(0.42 + 1e-12)),
         Some(Ordering::Equal)
      );
      assert_eq!(
         Angle::from(0.42).partial_cmp(&Angle::from(0.42 - 1e-12)),
         Some(Ordering::Equal)
      );
      assert_eq!(
         Angle::from(0.42).cmp(&Angle::from(0.42)),
         Ordering::Equal
      );
      assert_eq!(
         Angle::from(0.42).cmp(&Angle::from(0.42 + 1e-12)),
         Ordering::Equal
      );
      assert_eq!(
         Angle::from(0.42).cmp(&Angle::from(0.42 - 1e-12)),
         Ordering::Equal
      );
   }
}
