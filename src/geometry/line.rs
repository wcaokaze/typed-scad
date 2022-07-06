use crate::geometry::{Point, Vector};

#[derive(Clone, Copy, Debug)]
pub struct Line {
   point: Point,
   vector: Vector
}

impl Line {
   pub const X_AXIS: Line = Line::new(&Point::ORIGIN, &Vector::X_UNIT_VECTOR);
   pub const Y_AXIS: Line = Line::new(&Point::ORIGIN, &Vector::Y_UNIT_VECTOR);
   pub const Z_AXIS: Line = Line::new(&Point::ORIGIN, &Vector::Z_UNIT_VECTOR);

   pub const fn new(point: &Point, vector: &Vector) -> Line {
      Line {
         point: *point,
         vector: *vector
      }
   }

   pub fn from_2points(a: &Point, b: &Point) -> Line {
      Line {
         point: *a,
         vector: Vector::between(a, b)
      }
   }

   pub const fn vector(&self) -> &Vector {
      &self.vector
   }
}
