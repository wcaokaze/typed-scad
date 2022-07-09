use crate::geometry::{Plane, Point, Vector};
use crate::geometry::operators::Intersection;

#[derive(Clone, Copy, Debug)]
pub struct Line {
   pub(in crate::geometry) point: Point,
   pub(in crate::geometry) vector: Vector
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

   /// returns the point which is on this line and the nearest from origin.
   pub fn point(&self) -> Point {
      Plane::new(&Point::ORIGIN, &self.vector)
         .intersection(self)
   }

   pub const fn vector(&self) -> &Vector {
      &self.vector
   }
}

impl Intersection<&Plane> for &Line {
   type Output = Point;
   fn intersection(self, rhs: &Plane) -> Point {
      rhs.intersection(self)
   }
}

impl Intersection<&Plane> for Line {
   type Output = Point;
   fn intersection(self, rhs: &Plane) -> Point {
      rhs.intersection(self)
   }
}

impl Intersection<Plane> for &Line {
   type Output = Point;
   fn intersection(self, rhs: Plane) -> Point {
      rhs.intersection(self)
   }
}

impl Intersection<Plane> for Line {
   type Output = Point;
   fn intersection(self, rhs: Plane) -> Point {
      rhs.intersection(self)
   }
}

#[cfg(test)]
mod tests {
   use super::Line;
   use crate::geometry::{Point, Size, SizeLiteral, Vector};

   #[test]
   fn nearest_point_from_origin() {
      let actual = Line::new(
            &Point::new(6.mm(), 2.mm(), 0.mm()),
            &Vector::Y_UNIT_VECTOR
         )
         .point();

      assert_eq!(actual, Point::new(6.mm(), 0.mm(), 0.mm()));

      let actual = Line::new(
            &Point::new(0.mm(), 0.mm(), 1.mm()),
            &Vector::new(0.mm(), 1.mm(), -1.mm())
         )
         .point();

      assert_eq!(actual, Point::new(0.mm(), 0.5.mm(), 0.5.mm()));
   }
}
