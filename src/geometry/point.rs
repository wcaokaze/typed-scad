use crate::geometry::{Angle, Line, Size, Vector};
use crate::math::Matrix;
use crate::transform::Transform;

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

impl Transform for Point {
   fn translated(&self, offset: &Vector) -> Point {
      Point {
         matrix: self.matrix + offset.matrix
      }
   }

   fn rotated(&self, axis: &Line, angle: Angle) -> Point {
      let rotation_origin = axis.point;

      let mut v = Vector::between(&rotation_origin, &self);
      v.rotate(&axis.vector, angle);

      rotation_origin.translated(&v)
   }
}

impl Default for Point {
   fn default() -> Point {
      Point::ORIGIN
   }
}
