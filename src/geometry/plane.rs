use crate::geometry::{Angle, Line, Point, Size, Vector};
use crate::geometry::operators::Intersection;
use crate::math::rough_fp::rough_eq;
use crate::math::unit::Exp;
use crate::transform::Transform;
use noisy_float::prelude::*;

/// Plane in 3D.
///
/// [Plane::eq] returns `true` when 2 planes are equivalent.
/// ```
/// # use typed_scad::geometry::{Plane, Point, SizeLiteral, Vector};
/// let a = Plane::new(&Point::new(0.mm(), 0.mm(), 1.mm()), &Vector::X_UNIT_VECTOR);
/// let b = Plane::new(&Point::new(0.mm(), 2.mm(), 2.mm()), &Vector::X_UNIT_VECTOR);
/// assert_eq!(a, b);
///
/// let a = Plane::new(&Point::ORIGIN, & Vector::X_UNIT_VECTOR);
/// let b = Plane::new(&Point::ORIGIN, &-Vector::X_UNIT_VECTOR);
/// assert_eq!(a, b);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Plane {
   pub(in crate::geometry) point: Point,
   pub(in crate::geometry) normal_vector: Vector
}

impl Plane {
   pub const XY: Plane = Plane::new(&Point::ORIGIN, &Vector::Z_UNIT_VECTOR);
   pub const YZ: Plane = Plane::new(&Point::ORIGIN, &Vector::X_UNIT_VECTOR);
   pub const ZX: Plane = Plane::new(&Point::ORIGIN, &Vector::Y_UNIT_VECTOR);

   pub const fn new(point: &Point, normal_vector: &Vector) -> Plane {
      Plane {
         point: *point,
         normal_vector: *normal_vector
      }
   }

   pub fn from_3points(a: &Point, b: &Point, c: &Point) -> Plane {
      Plane {
         point: *a,
         normal_vector: Vector::between(a, b)
            .vector_product(&Vector::between(a, c))
      }
   }

   /// returns the point which is on this plane and the nearest from origin.
   pub fn point(&self) -> Point {
      Line::new(&Point::ORIGIN, &self.normal_vector)
         .intersection(self)
   }

   pub const fn normal_vector(&self) -> &Vector {
      &self.normal_vector
   }
}

impl PartialEq for Plane {
   fn eq(&self, other: &Plane) -> bool {
      let same_direction = self.normal_vector ==  other.normal_vector
                        || self.normal_vector == -other.normal_vector;

      same_direction && self.point() == other.point()
   }
}

impl Transform for Plane {
   fn translated(&self, offset: &Vector) -> Self {
      Plane {
         point: self.point.translated(offset),
         normal_vector: self.normal_vector
      }
   }

   fn rotated(&self, axis: &Line, angle: Angle) -> Self {
      Plane {
         point: self.point.rotated(axis, angle),
         normal_vector: self.normal_vector.rotated(&axis.vector, angle)
      }
   }
}

impl Intersection<Plane> for Plane {
   type Output = Line;

   fn intersection(&self, rhs: &Plane) -> Line {
      let sp = self.point;
      let sv = self.normal_vector;
      let rp = rhs.point;
      let rv = rhs.normal_vector;

      let vector = self.normal_vector.vector_product(&rhs.normal_vector);

      let point = if vector.x() != Size::ZERO {
         // When vector.x != 0, the line must pass X=0. So a point on the line
         // can solved from `self`, `rhs`, and X=0 as simultaneous equations.
         Point::new(
            Size::ZERO,
            Size::from(((sv.x() * rv.z() * sp.x()) - (sv.z() * rv.x() * rp.x()) + (sv.y() * rv.z() * sp.y()) - (sv.z() * rv.y() * rp.y()) + (sv.z() * rv.z() * sp.z()) - (sv.z() * rv.z() * rp.z())) /  (sv.y() * rv.z() - sv.z() * rv.y())),
            Size::from(((sv.z() * rv.y() * sp.z()) - (sv.y() * rv.z() * rp.z()) + (sv.x() * rv.y() * sp.x()) - (sv.y() * rv.x() * rp.x()) + (sv.y() * rv.y() * sp.y()) - (sv.y() * rv.y() * rp.y())) / -(sv.y() * rv.z() - sv.z() * rv.y()))
         )
      } else if vector.y() != Size::ZERO {
         Point::new(
            Size::from(((sv.x() * rv.z() * sp.x()) - (sv.z() * rv.x() * rp.x()) + (sv.y() * rv.z() * sp.y()) - (sv.z() * rv.y() * rp.y()) + (sv.z() * rv.z() * sp.z()) - (sv.z() * rv.z() * rp.z())) / -(sv.z() * rv.x() - sv.x() * rv.z())),
            Size::ZERO,
            Size::from(((sv.y() * rv.x() * sp.y()) - (sv.x() * rv.y() * rp.y()) + (sv.z() * rv.x() * sp.z()) - (sv.x() * rv.z() * rp.z()) + (sv.x() * rv.x() * sp.x()) - (sv.x() * rv.x() * rp.x())) /  (sv.z() * rv.x() - sv.x() * rv.z()))
         )
      } else if vector.z() != Size::ZERO {
         Point::new(
            Size::from(((sv.z() * rv.y() * sp.z()) - (sv.y() * rv.z() * rp.z()) + (sv.x() * rv.y() * sp.x()) - (sv.y() * rv.x() * rp.x()) + (sv.y() * rv.y() * sp.y()) - (sv.y() * rv.y() * rp.y())) /  (sv.x() * rv.y() - sv.y() * rv.x())),
            Size::from(((sv.y() * rv.x() * sp.y()) - (sv.x() * rv.y() * rp.y()) + (sv.z() * rv.x() * sp.z()) - (rv.z() * sv.x() * rp.z()) + (sv.x() * rv.x() * sp.x()) - (sv.x() * rv.x() * rp.x())) / -(sv.x() * rv.y() - sv.y() * rv.x())),
            Size::ZERO
         )
      } else {
         panic!("2 planes don't have an intersection.");
      };

      Line::new(&point, &vector)
   }
}

impl Intersection<Line> for Plane {
   type Output = Point;

   fn intersection(&self, rhs: &Line) -> Point {
      let inner_product: Exp<Size, 2>
         = self.normal_vector.inner_product(&rhs.vector);

      if rough_eq(inner_product.0, n64(0.0)) {
         panic!("The specified plane and line don't have an intersection.");
      }

      let t = N64::from(
         Vector::between(&rhs.point, &self.point)
            .inner_product(&self.normal_vector) / inner_product
      );

      Point {
         matrix: rhs.point.matrix + rhs.vector.matrix * t
      }
   }
}

#[cfg(test)]
mod tests {
   use super::Plane;
   use crate::geometry::{Line, Point, SizeLiteral, Vector};
   use crate::geometry::operators::Intersection;

   #[test]
   fn nearest_point_from_origin() {
      let actual = Plane::new(
            &Point::new(6.mm(), 2.mm(), 3.mm()),
            &Vector::X_UNIT_VECTOR
         )
         .point();

      assert_eq!(actual, Point::new(6.mm(), 0.mm(), 0.mm()));

      let actual = Plane::new(
            &Point::new(0.mm(), 0.mm(), 1.mm()),
            &Vector::new(0.mm(), 1.mm(), 1.mm())
         )
         .point();

      assert_eq!(actual, Point::new(0.mm(), 0.5.mm(), 0.5.mm()));
   }

   #[test]
   fn eq() {
      assert_eq!(
         Plane::new(&Point::ORIGIN, &Vector::Z_UNIT_VECTOR),
         Plane::new(&Point::new(3.mm(), 5.mm(), 0.mm()), &Vector::Z_UNIT_VECTOR)
      );

      assert_eq!(
         Plane::new(&Point::ORIGIN, & Vector::Z_UNIT_VECTOR),
         Plane::new(&Point::ORIGIN, &-Vector::Z_UNIT_VECTOR)
      );

      assert_ne!(
         Plane::new(&Point::ORIGIN, &Vector::Z_UNIT_VECTOR),
         Plane::new(&Point::new(3.mm(), 5.mm(), 1.mm()), &Vector::Z_UNIT_VECTOR)
      );

      assert_ne!(
         Plane::new(&Point::ORIGIN, &Vector::Z_UNIT_VECTOR),
         Plane::new(&Point::ORIGIN, &Vector::Y_UNIT_VECTOR)
      );
   }

   #[test]
   fn intersection_plane() {
      assert_eq!(
         Plane::XY.intersection(&Plane::YZ),
         Line::Y_AXIS
      );

      assert_eq!(
         Plane::YZ.intersection(&Plane::ZX),
         Line::Z_AXIS
      );

      assert_eq!(
         Plane::ZX.intersection(&Plane::XY),
         Line::X_AXIS
      );

      let z_3mm_plane = Plane::new(
         &Point::new(0.mm(), 0.mm(), 3.mm()),
         &Vector::Z_UNIT_VECTOR
      );

      let xy_45deg_plane = Plane::new(
         &Point::new(1.mm(), 0.mm(), 0.mm()),
         &Vector::new(-1.mm(), 1.mm(), 0.mm())
      );

      let actual = z_3mm_plane.intersection(&xy_45deg_plane);
      let expected = Line::new(
         &Point::new(1.mm(), 0.mm(), 3.mm()),
         &Vector::new(1.mm(), 1.mm(), 0.mm())
      );
      assert_eq!(actual, expected);
   }

   #[test]
   #[should_panic]
   fn intersection_same_planes() {
      Plane::XY.intersection(&Plane::XY);
   }

   #[test]
   #[should_panic]
   fn intersection_same_direction() {
      let a = Plane::new(&Point::new(1.mm(), 2.mm(), 3.mm()), &Vector::X_UNIT_VECTOR);
      let b = Plane::new(&Point::new(4.mm(), 5.mm(), 6.mm()), &Vector::X_UNIT_VECTOR);
      a.intersection(&b);
   }

   #[test]
   fn intersection_line() {
      assert_eq!(
         Plane::XY.intersection(&Line::Z_AXIS),
         Point::ORIGIN
      );

      let line = Line::new(
         &Point::new(1.mm(), 0.mm(), 0.mm()),
         &Vector::new(1.mm(), 1.mm(), -1.mm())
      );

      assert_eq!(
         Plane::YZ.intersection(&line),
         Point::new(0.mm(), -1.mm(), 1.mm())
      );
   }

   #[test]
   #[should_panic]
   fn intersection_line_on_plane() {
      Plane::XY.intersection(&Line::X_AXIS);
   }

   #[test]
   #[should_panic]
   fn intersection_same_direction_line() {
      let line = Line::new(
         &Point::new(0.mm(), 0.mm(), 3.mm()),
         &Vector::X_UNIT_VECTOR
      );

      Plane::XY.intersection(&line);
   }
}
