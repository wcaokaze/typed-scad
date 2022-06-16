use std::cmp::Ordering;
use super::{IterableAngleRange, Size};
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

   /// Converts this angle to a f64 value as radian
   pub fn to_radian(&self) -> f64 {
      self.0
   }

   /// Converts this angle to a f64 value as degree
   pub fn to_degree(&self) -> f64 {
      self.0.to_degrees()
   }

   fn sin(self) -> f64 {
      self.0.sin()
   }

   fn cos(self) -> f64 {
      self.0.cos()
   }

   fn tan(self) -> f64 {
      self.0.tan()
   }

   fn sin_cos(self) -> (f64, f64) {
      self.0.sin_cos()
   }

   fn asin(a: f64) -> Angle {
      Angle(f64::asin(a))
   }

   fn acos(a: f64) -> Angle {
      Angle(f64::acos(a))
   }

   fn atan(a: f64) -> Angle {
      Angle(f64::atan(a))
   }

   fn atan2(y: Size, x: Size) -> Angle {
      Angle(f64::atan2(y.to_millimeter(), x.to_millimeter()))
   }

   fn abs(self) -> Angle {
      Angle(self.0.abs())
   }

   fn clamp(self, min: Angle, max: Angle) -> Angle {
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

const D: f64 = 1e-10;

impl PartialOrd for Angle {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      match (self.0 < other.0 + D, self.0 > other.0 - D) {
         (false, false) => None,
         (false,  true) => Some(Ordering::Greater),
         ( true, false) => Some(Ordering::Less),
         ( true,  true) => Some(Ordering::Equal)
      }
   }
}

impl PartialEq for Angle {
   fn eq(&self, other: &Self) -> bool {
      self.0 > other.0 - D && self.0 < other.0 + D
   }
}

impl Add for Angle {
   type Output = Angle;
   fn add(self, rhs: Angle) -> Angle { Angle(self.0 + rhs.0) }
}

impl AddAssign for Angle {
   fn add_assign(&mut self, rhs: Angle) {
      *self = *self + rhs;
   }
}

impl Sub for Angle {
   type Output = Angle;
   fn sub(self, rhs: Angle) -> Angle { Angle(self.0 - rhs.0) }
}

impl SubAssign for Angle {
   fn sub_assign(&mut self, rhs: Angle) {
      *self = *self - rhs;
   }
}

impl<Rhs> Mul<Rhs> for Angle where Rhs: Into<f64> {
   type Output = Angle;
   fn mul(self, rhs: Rhs) -> Angle { Angle(self.0 * rhs.into()) }
}

impl<Rhs> MulAssign<Rhs> for Angle where Rhs: Into<f64> {
   fn mul_assign(&mut self, rhs: Rhs) {
      *self = *self * rhs;
   }
}

impl<Rhs> Div<Rhs> for Angle where Rhs: Into<f64> {
   type Output = Angle;
   fn div(self, rhs: Rhs) -> Angle { Angle(self.0 / rhs.into()) }
}

impl<Rhs> DivAssign<Rhs> for Angle where Rhs: Into<f64> {
   fn div_assign(&mut self, rhs: Rhs) {
      *self = *self / rhs;
   }
}

impl Div for Angle {
   type Output = f64;
   fn div(self, rhs: Angle) -> f64 { self.0 / rhs.0 }
}

impl Neg for Angle {
   type Output = Angle;
   fn neg(self) -> Angle { Angle(-self.0) }
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

impl<T> AngleLiteral for T where T: Into<f64> {
   fn deg(self) -> Angle {
      Angle(self.into().to_radians())
   }

   fn rad(self) -> Angle {
      Angle(self.into())
   }
}

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
