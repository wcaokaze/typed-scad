use crate::geometry::{Size, Vector};

/// 3D Point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
   pub offset_from_origin: Vector
}

impl Point {
   const ORIGIN: Point = Point { offset_from_origin: Vector::ZERO };

   pub const fn new(x: Size, y: Size, z: Size) -> Point {
      Point {
         offset_from_origin: Vector::new(x, y, z)
      }
   }

   pub fn x(&self) -> Size {
      self.offset_from_origin.x
   }

   pub fn y(&self) -> Size {
      self.offset_from_origin.y
   }

   pub fn z(&self) -> Size {
      self.offset_from_origin.z
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
