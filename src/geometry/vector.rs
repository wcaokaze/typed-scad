use crate::geometry::{Angle, Size, SizeLiteral, Point, sin, acos, cos};
use crate::math::Matrix;
use crate::math::conversion::ToN64;
use crate::math::unit::Exp;
use noisy_float::prelude::*;
use std::fmt::{self, Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::{
   Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign
};

/// 3D Vector.
#[derive(Clone, Copy, PartialEq)]
pub struct Vector {
   pub matrix: Matrix<Size, 3, 1>
}

impl Vector {
   pub const ZERO: Vector = Vector::new(Size::ZERO, Size::ZERO, Size::ZERO);
   pub const X_UNIT_VECTOR: Vector = Vector::new(
      Size::millimeter(N64::unchecked_new(1.0)),
      Size::ZERO,
      Size::ZERO
   );
   pub const Y_UNIT_VECTOR: Vector = Vector::new(
      Size::ZERO,
      Size::millimeter(N64::unchecked_new(1.0)),
      Size::ZERO
   );
   pub const Z_UNIT_VECTOR: Vector = Vector::new(
      Size::ZERO,
      Size::ZERO,
      Size::millimeter(N64::unchecked_new(1.0)),
   );

   pub const fn new(x: Size, y: Size, z: Size) -> Vector {
      Vector {
         matrix: Matrix([[x], [y], [z]])
      }
   }

   pub fn between(point_a: &Point, point_b: &Point) -> Vector {
      Vector {
         matrix: point_b.matrix - point_a.matrix
      }
   }

   #[inline]
   pub const fn x(&self) -> Size {
      self.matrix.0[0][0]
   }

   #[inline]
   pub const fn y(&self) -> Size {
      self.matrix.0[1][0]
   }

   #[inline]
   pub const fn z(&self) -> Size {
      self.matrix.0[2][0]
   }

   pub fn norm(&self) -> Size {
      (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
   }

   pub fn to_unit_vector(&self) -> Vector {
      let norm = self.norm();
      if norm == 0.mm() {
         panic!("cannot convert to a unit vector \
                 since this vector does not point any direction.");
      }

      Vector {
         matrix: self.matrix / norm.0
      }
   }

   pub fn vector_product(&self, other: &Vector) -> Vector {
      unsafe {
         Vector::new(
            (self.y() * other.z() - self.z() * other.y()).operate_as::<Size, 1>().into(),
            (self.z() * other.x() - self.x() * other.z()).operate_as::<Size, 1>().into(),
            (self.x() * other.y() - self.y() * other.x()).operate_as::<Size, 1>().into()
         )
      }
   }

   pub fn inner_product(&self, other: &Vector) -> Exp<Size, 2> {
      (self.matrix * other.matrix.transpose()).0[0][0]
   }

   pub fn angle_with(&self, other: &Vector) -> Angle {
      acos(
         (self.inner_product(other) / (self.norm() * other.norm())).into()
      )
   }

   pub fn rotate(&mut self, axis: &Vector, angle: Angle) {
      *self = self.rotated(axis, angle);
   }

   pub fn rotated(&self, axis: &Vector, angle: Angle) -> Vector {
      let axis_unit_vector = axis.to_unit_vector();

      let axis_vector = {
         let inner_product: Size = unsafe {
            self.inner_product(&axis_unit_vector)
               .operate_as::<Size, 1>().into()
         };
         axis.matrix * (inner_product / axis.norm())
      };

      Vector {
         matrix: self.matrix * cos(angle)
            + (n64(1.0) - cos(angle)) * axis_vector
            + axis_unit_vector.vector_product(&self).matrix * sin(angle)
      }
   }
}

impl Display for Vector {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
   }
}

impl Debug for Vector {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      write!(f, "Vector{}", self)
   }
}

impl Add for Vector {
   type Output = Vector;
   fn add(self, rhs: Vector) -> Vector {
      Vector {
         matrix: self.matrix + rhs.matrix
      }
   }
}

impl AddAssign for Vector {
   fn add_assign(&mut self, rhs: Vector) {
      *self = *self + rhs;
   }
}

impl Sub for Vector {
   type Output = Vector;
   fn sub(self, rhs: Vector) -> Vector {
      Vector {
         matrix: self.matrix - rhs.matrix
      }
   }
}

impl SubAssign for Vector {
   fn sub_assign(&mut self, rhs: Vector) {
      *self = *self - rhs;
   }
}

macro_rules! mul {
   ($($t:ty),+) => ($(
      impl Mul<$t> for Vector {
         type Output = Vector;
         fn mul(self, rhs: $t) -> Vector {
            Vector {
               matrix: self.matrix * rhs.to_n64()
            }
         }
      }

      impl MulAssign<$t> for Vector {
         fn mul_assign(&mut self, rhs: $t) {
            *self = *self * rhs;
         }
      }

      impl Mul<Vector> for $t {
         type Output = Vector;
         fn mul(self, rhs: Vector) -> Vector {
            rhs * self
         }
      }
   )+)
}

mul!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64,
   N32, N64, R32, R64);

macro_rules! div {
   ($($t:ty),+) => ($(
      impl Div<$t> for Vector {
         type Output = Vector;
         fn div(self, rhs: $t) -> Vector {
            Vector {
               matrix: self.matrix / rhs.to_n64()
            }
         }
      }

      impl DivAssign<$t> for Vector {
         fn div_assign(&mut self, rhs: $t) {
            *self = *self / rhs;
         }
      }
   )+)
}

div!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64,
   N32, N64, R32, R64);

impl Neg for Vector {
   type Output = Vector;
   fn neg(self) -> Self::Output {
      Vector {
         matrix: self.matrix * -1
      }
   }
}

impl Sum for Vector {
   fn sum<I>(iter: I) -> Vector where I: Iterator<Item = Vector> {
      let mut sum = Vector::ZERO;
      for v in iter {
         sum += v;
      }
      sum
   }
}

#[cfg(test)]
mod tests {
   use crate::geometry::{AngleLiteral, Point, SizeLiteral};
   use super::Vector;

   fn vector(x: f64, y: f64, z: f64) -> Vector {
      Vector::new(x.mm(), y.mm(), z.mm())
   }

   #[test]
   fn between() {
      let actual = Vector::between(
         &Point::new(1.mm(), 2.mm(), 3.mm()),
         &Point::new(3.mm(), 5.mm(), 7.mm())
      );
      assert_eq!(actual, Vector::new(2.mm(), 3.mm(), 4.mm()));

      let actual = Vector::between(
         &Point::new(3.mm(), 5.mm(), 7.mm()),
         &Point::new(1.mm(), 2.mm(), 3.mm())
      );
      assert_eq!(actual, Vector::new(-2.mm(), -3.mm(), -4.mm()));
   }

   #[test]
   fn norm() {
      assert_eq!(Vector::new(0.mm(), 3.mm(), 4.mm()).norm(), 5.mm());
   }

   #[test]
   fn to_unit_vector() {
      assert_eq!(
         Vector::new(42.mm(), 0.mm(), 0.mm()).to_unit_vector(),
         Vector::X_UNIT_VECTOR
      );

      assert_eq!(
         Vector::new(1.mm(), 2.mm(), 3.mm()).to_unit_vector().norm(),
         1.mm()
      );
   }

   #[test]
   #[should_panic]
   fn to_unit_vector_panic() {
      Vector::new(0.mm(), 0.mm(), 0.mm()).to_unit_vector();
   }

   #[test]
   fn operators() {
      assert_eq!(vector( 1.0,  2.0,  3.0) + vector( 1.5,  1.5,  1.5), vector( 2.5,  3.5,  4.5));
      assert_eq!(vector( 1.0,  2.0,  3.0) + vector(-1.5, -1.5, -1.5), vector(-0.5,  0.5,  1.5));
      assert_eq!(vector(-1.0, -2.0, -3.0) + vector( 1.5,  1.5,  1.5), vector( 0.5, -0.5, -1.5));
      assert_eq!(vector(-1.0, -2.0, -3.0) + vector(-1.5, -1.5, -1.5), vector(-2.5, -3.5, -4.5));

      assert_eq!(vector( 1.0,  2.0,  3.0) - vector( 1.5,  1.5,  1.5), vector(-0.5,  0.5,  1.5));
      assert_eq!(vector( 1.0,  2.0,  3.0) - vector(-1.5, -1.5, -1.5), vector( 2.5,  3.5,  4.5));
      assert_eq!(vector(-1.0, -2.0, -3.0) - vector( 1.5,  1.5,  1.5), vector(-2.5, -3.5, -4.5));
      assert_eq!(vector(-1.0, -2.0, -3.0) - vector(-1.5, -1.5, -1.5), vector( 0.5, -0.5, -1.5));

      assert_eq!(vector( 1.0,  2.0,  3.0) *  2, vector( 2.0,  4.0,  6.0));
      assert_eq!(vector( 1.0,  2.0,  3.0) * -2, vector(-2.0, -4.0, -6.0));
      assert_eq!(vector(-1.0, -2.0, -3.0) *  2, vector(-2.0, -4.0, -6.0));
      assert_eq!(vector(-1.0, -2.0, -3.0) * -2, vector( 2.0,  4.0,  6.0));
      assert_eq!(vector( 1.0,  2.0,  3.0) *  1.5, vector( 1.5,  3.0,  4.5));
      assert_eq!(vector( 1.0,  2.0,  3.0) * -1.5, vector(-1.5, -3.0, -4.5));
      assert_eq!(vector(-1.0, -2.0, -3.0) *  1.5, vector(-1.5, -3.0, -4.5));
      assert_eq!(vector(-1.0, -2.0, -3.0) * -1.5, vector( 1.5,  3.0,  4.5));

      assert_eq!(vector( 1.0,  2.0,  3.0) /  2, vector( 0.5,  1.0,  1.5));
      assert_eq!(vector( 1.0,  2.0,  3.0) / -2, vector(-0.5, -1.0, -1.5));
      assert_eq!(vector(-1.0, -2.0, -3.0) /  2, vector(-0.5, -1.0, -1.5));
      assert_eq!(vector(-1.0, -2.0, -3.0) / -2, vector( 0.5,  1.0,  1.5));
      assert_eq!(vector( 3.0,  6.0,  9.0) /  1.5, vector( 2.0,  4.0,  6.0));
      assert_eq!(vector( 3.0,  6.0,  9.0) / -1.5, vector(-2.0, -4.0, -6.0));
      assert_eq!(vector(-3.0, -6.0, -9.0) /  1.5, vector(-2.0, -4.0, -6.0));
      assert_eq!(vector(-3.0, -6.0, -9.0) / -1.5, vector( 2.0,  4.0,  6.0));

      assert_eq!(-vector(1.0, 2.0, 3.0), vector(-1.0, -2.0, -3.0));
   }

   #[test]
   fn vector_product() {
      assert_eq!(
         Vector::X_UNIT_VECTOR.vector_product(&Vector::Y_UNIT_VECTOR),
         Vector::Z_UNIT_VECTOR
      );

      assert_eq!(
         Vector::Y_UNIT_VECTOR.vector_product(&Vector::Z_UNIT_VECTOR),
         Vector::X_UNIT_VECTOR
      );

      assert_eq!(
         Vector::Z_UNIT_VECTOR.vector_product(&Vector::X_UNIT_VECTOR),
         Vector::Y_UNIT_VECTOR
      );

      assert_eq!(
         Vector::Y_UNIT_VECTOR.vector_product(&-Vector::X_UNIT_VECTOR),
         Vector::Z_UNIT_VECTOR
      );

      assert_eq!(
         Vector::Y_UNIT_VECTOR.vector_product(&Vector::X_UNIT_VECTOR),
         -Vector::Z_UNIT_VECTOR
      );

      assert_eq!(
         vector(1.0, 0.0, 0.0).vector_product(&vector(2.0, 3.0, 0.0)),
         vector(0.0, 0.0, 3.0)
      );
   }

   #[test]
   fn angle_with() {
      assert_eq!(
         Vector::X_UNIT_VECTOR.angle_with(&Vector::Y_UNIT_VECTOR),
         90.deg()
      );

      assert_eq!(
         Vector::X_UNIT_VECTOR.angle_with(&vector(3.0, 3.0, 0.0)),
         45.deg()
      );

      assert_eq!(
         Vector::X_UNIT_VECTOR.angle_with(&vector(0.0, 4.0, -3.0)),
         90.deg()
      );
   }

   #[test]
   fn sum() {
      let sum: Vector = (1..=10)
         .into_iter()
         .map(|i| Vector::new(i.mm(), i.mm(), i.mm()))
         .sum();

      assert_eq!(sum, Vector::new(55.mm(), 55.mm(), 55.mm()));
   }

   #[test]
   fn rotate() {
      assert_eq!(
         Vector::X_UNIT_VECTOR.rotated(&Vector::Y_UNIT_VECTOR, 90.deg()),
         -Vector::Z_UNIT_VECTOR
      );

      assert_eq!(
         Vector::Y_UNIT_VECTOR.rotated(&Vector::Z_UNIT_VECTOR, 90.deg()),
         -Vector::X_UNIT_VECTOR
      );

      assert_eq!(
         Vector::Z_UNIT_VECTOR.rotated(&Vector::X_UNIT_VECTOR, 90.deg()),
         -Vector::Y_UNIT_VECTOR
      );

      let actual = Vector::X_UNIT_VECTOR
         .rotated(&Vector::Z_UNIT_VECTOR, 45.deg());

      let expected = Vector::new(
         (1.0 / f64::sqrt(2.0)).mm(),
         (1.0 / f64::sqrt(2.0)).mm(),
         0.mm()
      );

      assert_eq!(actual, expected);
   }
}
