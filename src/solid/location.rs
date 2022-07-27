use crate::geometry::{Angle, AngleLiteral, Line, Point, Vector};
use crate::solid::LocationBuilder;
use crate::transform::Transform;

/// [Point] and Direction in 3D.
///
/// Typically used for the location of a [Solid][crate::solid::Solid].
///
/// Directions are
/// relative X-Axis([Location::left_vector], [Location::right_vector]),
/// relative Y-Axis([Location::front_vector], [Location::back_vector]),
/// and relative Z-Axis([Location::bottom_vector], [Location::top_vector]).
/// These methods return a unit vector.
///
/// Use chain notation to instantiate.
/// ```
/// # use typed_scad::geometry::{Point, Vector};
/// # use typed_scad::solid::Location;
/// # let point = Point::ORIGIN;
/// # let left_vector = Vector::X_UNIT_VECTOR;
/// # let back_vector = Vector::Y_UNIT_VECTOR;
/// let location = Location::build(point)
///    .left_vector(left_vector)
///    .back_vector(back_vector);
/// ```
///
/// Location can also be built from any other axes.
/// ```
/// # use typed_scad::geometry::{Point, Vector};
/// # use typed_scad::solid::Location;
/// # let point = Point::ORIGIN;
/// # let bottom_vector = -Vector::Z_UNIT_VECTOR;
/// # let front_vector = -Vector::Y_UNIT_VECTOR;
/// let location = Location::build(point)
///    .bottom_vector(bottom_vector)
///    .front_vector(front_vector);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
   point: Point,
   right_vector: Vector,
   back_vector: Vector
}

impl Location {
   pub(in crate::solid) fn new(
      point: Point,
      right_vector: Vector,
      back_vector: Vector
   ) -> Location {
      if right_vector.angle_with(&back_vector) != 90.deg() {
         panic!("The angle formed by 2 vectors must be 90 degrees.");
      }

      Location {
         point,
         right_vector: right_vector.to_unit_vector(),
         back_vector: back_vector.to_unit_vector()
      }
   }

   pub fn build(point: Point) -> LocationBuilder<false, false, false> {
      LocationBuilder::new(point)
   }

   pub fn point(&self) -> Point {
      self.point
   }

   pub fn left_vector(&self) -> Vector {
      -self.right_vector
   }

   pub fn right_vector(&self) -> Vector {
      self.right_vector
   }

   pub fn front_vector(&self) -> Vector {
      -self.back_vector
   }

   pub fn back_vector(&self) -> Vector {
      self.back_vector
   }

   pub fn bottom_vector(&self) -> Vector {
      self.back_vector.vector_product(&self.right_vector)
   }

   pub fn top_vector(&self) -> Vector {
      self.right_vector.vector_product(&self.back_vector)
   }
}

impl Default for Location {
   fn default() -> Location {
      Location {
         point: Point::ORIGIN,
         right_vector: Vector::X_UNIT_VECTOR,
         back_vector: Vector::Y_UNIT_VECTOR
      }
   }
}

impl Transform for Location {
   fn translated(&self, offset: &Vector) -> Location {
      Location {
         point: self.point.translated(offset),
         right_vector: self.right_vector,
         back_vector: self.back_vector
      }
   }

   fn rotated(&self, axis: &Line, angle: Angle) -> Location {
      Location {
         point: self.point.rotated(axis, angle),
         right_vector: self.right_vector.rotated(axis.vector(), angle),
         back_vector: self.back_vector.rotated(axis.vector(), angle)
      }
   }
}
