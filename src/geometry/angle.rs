use crate::foundation::rough_fp::{rough_partial_cmp, rough_partial_eq};
use crate::geometry::{IterableAngleRange, Size};
use crate::geometry::unit::{Exp, Unit};
use std::cmp::Ordering;
use std::fmt::{self, Display, Formatter};
use std::ops::{
   Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign
};

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
/// Angle implements PartialEq and PartialOrd.
/// They allows float-point arithmetic errors.
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
#[derive(Clone, Copy, Debug, Default)]
pub struct Angle(f64);

impl Angle {
   /// PI radian. But `Angle::PI` is not enough readable.
   /// Also consider using `180.deg()`
   pub const PI: Angle = Angle(std::f64::consts::PI);

   pub const fn radian(radian: f64) -> Angle {
      Angle(radian)
   }

   /// Converts this angle to a f64 value as radian
   pub const fn to_radian(self) -> f64 {
      self.0
   }

   /// Converts this angle to a f64 value as degree
   pub fn to_degree(self) -> f64 {
      self.0.to_degrees()
   }

   pub fn sin(self) -> f64 {
      self.0.sin()
   }

   pub fn cos(self) -> f64 {
      self.0.cos()
   }

   pub fn tan(self) -> f64 {
      self.0.tan()
   }

   pub fn sin_cos(self) -> (f64, f64) {
      self.0.sin_cos()
   }

   pub fn asin(a: f64) -> Angle {
      Angle(f64::asin(a))
   }

   pub fn acos(a: f64) -> Angle {
      Angle(f64::acos(a))
   }

   pub fn atan(a: f64) -> Angle {
      Angle(f64::atan(a))
   }

   pub fn atan2(y: Size, x: Size) -> Angle {
      Angle(f64::atan2(y.0, x.0))
   }

   pub fn abs(self) -> Angle {
      Angle(self.0.abs())
   }

   pub fn clamp(self, min: Angle, max: Angle) -> Angle {
      Angle(self.0.clamp(min.0, max.0))
   }

   /// Prepare to iterate [Angle]s in the specified range.
   /// And [step][IterableAngleRange::step] returns an [Iterator] for Angle.
   ///
   /// ```
   /// # use typed_scad::geometry::{Angle, AngleLiteral, IterableAngleRange};
   /// let iter = Angle::iterate(0.deg()..=3.deg()).step(1.deg());
   /// assert_eq!(iter.collect::<Vec<_>>(), vec![0.deg(), 1.deg(), 2.deg(), 3.deg()]);
   /// ```
   ///
   /// Negative steps are also available.
   /// ```
   /// # use typed_scad::geometry::{Angle, AngleLiteral, IterableAngleRange};
   /// let iter = Angle::iterate(3.deg()..=0.deg()).step(-1.deg());
   /// assert_eq!(iter.collect::<Vec<_>>(), vec![3.deg(), 2.deg(), 1.deg(), 0.deg()]);
   /// ```
   pub fn iterate<R>(angle_range: R) -> R where R: IterableAngleRange {
      angle_range
   }
}

impl Display for Angle {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      f.write_fmt(format_args!("{}°", self.0.to_degrees()))
   }
}

impl PartialOrd for Angle {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      rough_partial_cmp(self.0, other.0)
   }
}

impl PartialEq for Angle {
   fn eq(&self, other: &Self) -> bool {
      rough_partial_eq(self.0, other.0)
   }
}

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
            Angle(self.0 * rhs as f64)
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

mul!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

macro_rules! div {
   ($($t:ty),+) => ($(
      impl Div<$t> for Angle {
         type Output = Angle;
         fn div(self, rhs: $t) -> Angle {
            Angle(self.0 / rhs as f64)
         }
      }

      impl DivAssign<$t> for Angle {
         fn div_assign(&mut self, rhs: $t) {
            *self = *self / rhs;
         }
      }
   )+)
}

div!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

impl Div for Angle {
   type Output = f64;
   fn div(self, rhs: Angle) -> f64 {
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

impl Into<f64> for Exp<Angle, 0> {
   fn into(self) -> f64 {
      self.0
   }
}

impl Into<Angle> for Exp<Angle, 1> {
   fn into(self) -> Angle {
      Angle(self.0)
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
            Angle((self as f64).to_radians())
         }

         fn rad(self) -> Angle {
            Angle(self as f64)
         }
      }
   )+)
}

angle_literal!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

#[cfg(test)]
mod tests {
   use super::{Angle, AngleLiteral};
   use std::cmp::Ordering;
   use std::f64::consts::PI;

   #[test]
   fn eq() {
      assert_eq!(Angle(0.42), Angle(0.42));
      assert_ne!(Angle(0.42), Angle(0.43));

      assert_ne!(      0.42,        0.42 + 1e-12);
      assert_eq!(Angle(0.42), Angle(0.42 + 1e-12));
      assert_ne!(      0.42,        0.42 - 1e-12);
      assert_eq!(Angle(0.42), Angle(0.42 - 1e-12));

      assert_ne!(Angle(0.42), Angle(0.42 + 2.0 * PI));
   }

   #[test]
   fn display() {
      assert_eq!(
         format!("{}", Angle(PI)),
         "180°".to_string()
      );
   }

   #[test]
   fn angle_literal() {
      assert_eq!(2.rad(), Angle(2.0));
      assert_eq!(180.deg(), Angle(PI));
      assert_eq!(0.42.rad(), Angle(0.42));
      assert_eq!(180.0.deg(), Angle(PI));
   }

   #[test]
   fn to_radian() {
      assert_eq!(Angle(0.42).to_radian(), 0.42);
   }

   #[test]
   fn to_degree() {
      assert_eq!(Angle(PI).to_degree(), 180.0);
   }

   #[test]
   fn operators() {
      assert_eq!(Angle( 0.42) + Angle( 0.15), Angle(0.57));
      assert_eq!(Angle( 0.42) + Angle(-0.15), Angle(0.27));
      assert_eq!(Angle(-0.42) + Angle( 0.15), Angle(-0.27));
      assert_eq!(Angle(-0.42) + Angle(-0.15), Angle(-0.57));

      assert_eq!(Angle( 0.42) - Angle( 0.15), Angle(0.27));
      assert_eq!(Angle( 0.42) - Angle(-0.15), Angle(0.57));
      assert_eq!(Angle(-0.42) - Angle( 0.15), Angle(-0.57));
      assert_eq!(Angle(-0.42) - Angle(-0.15), Angle(-0.27));

      assert_eq!(Angle( 0.42) *  2, Angle( 0.84));
      assert_eq!(Angle( 0.42) * -2, Angle(-0.84));
      assert_eq!(Angle(-0.42) *  2, Angle(-0.84));
      assert_eq!(Angle(-0.42) * -2, Angle( 0.84));
      assert_eq!(Angle( 0.42) *  1.5, Angle( 0.63));
      assert_eq!(Angle( 0.42) * -1.5, Angle(-0.63));
      assert_eq!(Angle(-0.42) *  1.5, Angle(-0.63));
      assert_eq!(Angle(-0.42) * -1.5, Angle( 0.63));

      assert_eq!( 2 * Angle( 0.42), Angle( 0.84));
      assert_eq!(-2 * Angle( 0.42), Angle(-0.84));
      assert_eq!( 2 * Angle(-0.42), Angle(-0.84));
      assert_eq!(-2 * Angle(-0.42), Angle( 0.84));
      assert_eq!( 1.5 * Angle( 0.42), Angle( 0.63));
      assert_eq!(-1.5 * Angle( 0.42), Angle(-0.63));
      assert_eq!( 1.5 * Angle(-0.42), Angle(-0.63));
      assert_eq!(-1.5 * Angle(-0.42), Angle( 0.63));

      assert_eq!(Angle( 0.42) /  2, Angle( 0.21));
      assert_eq!(Angle( 0.42) / -2, Angle(-0.21));
      assert_eq!(Angle(-0.42) /  2, Angle(-0.21));
      assert_eq!(Angle(-0.42) / -2, Angle( 0.21));
      assert_eq!(Angle( 0.42) /  1.5, Angle( 0.28));
      assert_eq!(Angle( 0.42) / -1.5, Angle(-0.28));
      assert_eq!(Angle(-0.42) /  1.5, Angle(-0.28));
      assert_eq!(Angle(-0.42) / -1.5, Angle( 0.28));

      // 0.42 causes a float-point arithmetic error. We should use 2⁻ⁿ here
      assert_eq!(Angle( 0.25) / Angle( 0.5),  0.5);
      assert_eq!(Angle( 0.25) / Angle(-0.5), -0.5);
      assert_eq!(Angle(-0.25) / Angle( 0.5), -0.5);
      assert_eq!(Angle(-0.25) / Angle(-0.5),  0.5);

      assert_eq!(-Angle(0.42), Angle(-0.42));

      assert!(Angle(0.42) > Angle(0.41));
      assert!(Angle(0.41) < Angle(0.42));
      assert!(Angle(0.42) < Angle(0.42 + 2.0 * PI));

      assert_eq!(
         Angle(0.42).partial_cmp(&Angle(f64::NAN)),
         None
      );
      assert_eq!(
         Angle(f64::NAN).partial_cmp(&Angle(0.42)),
         None
      );
      assert_eq!(
         Angle(f64::NAN).partial_cmp(&Angle(f64::NAN)),
         None
      );
      assert_eq!(
         Angle(0.42).partial_cmp(&Angle(0.42)),
         Some(Ordering::Equal)
      );
      assert_eq!(
         Angle(0.42).partial_cmp(&Angle(0.42 + 1e-12)),
         Some(Ordering::Equal)
      );
      assert_eq!(
         Angle(0.42).partial_cmp(&Angle(0.42 - 1e-12)),
         Some(Ordering::Equal)
      );
   }
}
