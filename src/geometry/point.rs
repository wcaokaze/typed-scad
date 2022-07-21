use crate::geometry::{Size, Vector};
use crate::math::Matrix;

/// 3D Point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
   pub matrix: Matrix<Size, 3, 1>
}

impl Point {
   pub const ORIGIN: Point = Point::new(Size::ZERO, Size::ZERO, Size::ZERO);

   pub const fn new(x: Size, y: Size, z: Size) -> Point {
      Point {
         matrix: Matrix([[x], [y], [z]])
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

   pub fn distance(&self, another: &Point) -> Size {
      Vector::between(self, another).norm()
   }
}

impl Default for Point {
   fn default() -> Point {
      Point::ORIGIN
   }
}
