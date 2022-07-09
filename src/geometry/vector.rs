use crate::geometry::{Angle, Size, SizeLiteral, Point};
use crate::geometry::unit::Exp;
use std::iter::Sum;
use std::ops::{
   Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign
};

/// 3D Vector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
   pub x: Size,
   pub y: Size,
   pub z: Size
}

impl Vector {
   pub const ZERO: Vector = Vector::new(Size::ZERO, Size::ZERO, Size::ZERO);
   pub const X_UNIT_VECTOR: Vector = Vector::new(
      Size::millimeter(1.0), Size::millimeter(0.0), Size::millimeter(0.0));
   pub const Y_UNIT_VECTOR: Vector = Vector::new(
      Size::millimeter(0.0), Size::millimeter(1.0), Size::millimeter(0.0));
   pub const Z_UNIT_VECTOR: Vector = Vector::new(
      Size::millimeter(0.0), Size::millimeter(0.0), Size::millimeter(1.0));

   pub const fn new(x: Size, y: Size, z: Size) -> Vector {
      Vector { x, y, z }
   }

   pub fn between(point_a: &Point, point_b: &Point) -> Vector {
      Vector {
         x: point_b.x() - point_a.x(),
         y: point_b.y() - point_a.y(),
         z: point_b.z() - point_a.z()
      }
   }

   pub fn norm(&self) -> Size {
      let x = self.x.to_millimeter();
      let y = self.y.to_millimeter();
      let z = self.z.to_millimeter();

      let norm = f64::sqrt(x * x + y * y + z * z);
      Size::millimeter(norm)
   }

   pub fn to_unit_vector(&self) -> Vector {
      let norm = self.norm();
      if norm == 0.mm() {
         panic!("cannot convert to a unit vector \
                 since this vector does not point any direction.");
      }

      Vector::new(
         Size::millimeter(self.x.to_millimeter() / norm.to_millimeter()),
         Size::millimeter(self.y.to_millimeter() / norm.to_millimeter()),
         Size::millimeter(self.z.to_millimeter() / norm.to_millimeter())
      )
   }

   pub fn vector_product(&self, other: &Vector) -> Vector {
      let ax = self.x.to_millimeter();
      let ay = self.y.to_millimeter();
      let az = self.z.to_millimeter();
      let bx = other.x.to_millimeter();
      let by = other.y.to_millimeter();
      let bz = other.z.to_millimeter();

      Vector::new(
         Size::millimeter(ay * bz - az * by),
         Size::millimeter(az * bx - ax * bz),
         Size::millimeter(ax * by - ay * bx)
      )
   }

   pub fn inner_product(&self, other: &Vector) -> Exp<Size, 2> {
      unsafe {
         Exp::new(
            self.x.to_millimeter() * other.x.to_millimeter() +
            self.y.to_millimeter() * other.y.to_millimeter() +
            self.z.to_millimeter() * other.z.to_millimeter()
         )
      }
   }

   pub fn angle_with(&self, other: &Vector) -> Angle {
      Angle::acos(
         self.inner_product(other).0
            / (self.norm().to_millimeter() * other.norm().to_millimeter())
      )
   }
}

impl Add for Vector {
   type Output = Vector;
   fn add(self, rhs: Vector) -> Vector {
      Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
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
      Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
            Vector::new(
               self.x * rhs as f64,
               self.y * rhs as f64,
               self.z * rhs as f64
            )
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

mul!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

macro_rules! div {
   ($($t:ty),+) => ($(
      impl Div<$t> for Vector {
         type Output = Vector;
         fn div(self, rhs: $t) -> Vector {
            Vector::new(
               self.x / rhs as f64,
               self.y / rhs as f64,
               self.z / rhs as f64
            )
         }
      }

      impl DivAssign<$t> for Vector {
         fn div_assign(&mut self, rhs: $t) {
            *self = *self / rhs;
         }
      }
   )+)
}

div!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

impl Neg for Vector {
   type Output = Vector;
   fn neg(self) -> Self::Output {
      Vector::new(-self.x, -self.y, -self.z)
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
}