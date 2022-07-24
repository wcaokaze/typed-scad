use crate::geometry::{Angle, Plane, Point, Vector};
use crate::geometry::operators::Intersection;
use crate::transform::Transform;

/// Line in 3D.
///
/// [Line::eq] returns `true` when 2 lines are equivalent.
/// ```
/// # use typed_scad::geometry::{Line, Point, SizeLiteral, Vector};
/// let a = Line::new(&Point::new(0.mm(), 0.mm(), 0.mm()), &Vector::X_UNIT_VECTOR);
/// let b = Line::new(&Point::new(1.mm(), 0.mm(), 0.mm()), &Vector::X_UNIT_VECTOR);
/// assert_eq!(a, b);
///
/// let a = Line::new(&Point::ORIGIN, & Vector::X_UNIT_VECTOR);
/// let b = Line::new(&Point::ORIGIN, &-Vector::X_UNIT_VECTOR);
/// assert_eq!(a, b);
/// ```
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

impl PartialEq for Line {
   fn eq(&self, other: &Line) -> bool {
      let same_direction = self.vector ==  other.vector
                        || self.vector == -other.vector;

      same_direction && self.point() == other.point()
   }
}

impl Transform for Line {
   fn translated(&self, offset: &Vector) -> Self {
      Line {
         point: self.point.translated(offset),
         vector: self.vector
      }
   }

   fn rotated(&self, axis: &Line, angle: Angle) -> Self {
      Line {
         point: self.point.rotated(axis, angle),
         vector: self.vector.rotated(&axis.vector, angle)
      }
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
   use crate::geometry::{Point, SizeLiteral, Vector};

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

   #[test]
   fn eq() {
      assert_eq!(
         Line::new(&Point::ORIGIN, &Vector::Z_UNIT_VECTOR),
         Line::new(&Point::new(0.mm(), 0.mm(), 4.mm()), &Vector::Z_UNIT_VECTOR)
      );

      assert_eq!(
         Line::new(&Point::ORIGIN, & Vector::Z_UNIT_VECTOR),
         Line::new(&Point::ORIGIN, &-Vector::Z_UNIT_VECTOR),
      );

      assert_ne!(
         Line::new(&Point::ORIGIN, &Vector::Z_UNIT_VECTOR),
         Line::new(&Point::new(1.mm(), 0.mm(), 4.mm()), &Vector::Z_UNIT_VECTOR)
      );

      assert_ne!(
         Line::new(&Point::ORIGIN, &Vector::Z_UNIT_VECTOR),
         Line::new(&Point::ORIGIN, &Vector::Y_UNIT_VECTOR)
      );
   }
}
