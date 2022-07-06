use crate::geometry::{Line, Point, Size, Vector};
use crate::geometry::operators::Intersection;

#[derive(Clone, Copy, Debug)]
pub struct Plane {
   point: Point,
   normal_vector: Vector
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

   pub const fn normal_vector(&self) -> &Vector {
      &self.normal_vector
   }
}

impl Intersection<Plane> for Plane {
   type Output = Line;

   fn intersection(self, rhs: Plane) -> Line {
      let p1 = (
         self.point.x().to_millimeter(),
         self.point.y().to_millimeter(),
         self.point.z().to_millimeter()
      );

      let v1 = (
         self.normal_vector.x.to_millimeter(),
         self.normal_vector.y.to_millimeter(),
         self.normal_vector.z.to_millimeter()
      );

      let p2 = (
         rhs.point.x().to_millimeter(),
         rhs.point.y().to_millimeter(),
         rhs.point.z().to_millimeter()
      );

      let v2 = (
         rhs.normal_vector.x.to_millimeter(),
         rhs.normal_vector.y.to_millimeter(),
         rhs.normal_vector.z.to_millimeter()
      );

      let vector = self.normal_vector.vector_product(&rhs.normal_vector);

      let point = if vector.x != Size::ZERO {
         // When vector.x != 0, the line must pass X=0. So a point on the line
         // can solved from `self`, `rhs`, and X=0 as simultaneous equations.
         Point::new(
            Size::ZERO,
            Size::millimeter(((v1.0 * v2.2 * p1.0) - (v1.2 * v2.0 * p2.0) + (v1.1 * v2.2 * p1.1) - (v1.2 * v2.1 * p2.1) + (v1.2 * v2.2 * p1.2) - (v1.2 * v2.2 * p2.2)) /  (v1.1 * v2.2 - v1.2 * v2.1)),
            Size::millimeter(((v1.2 * v2.1 * p1.2) - (v1.1 * v2.2 * p2.2) + (v1.0 * v2.1 * p1.0) - (v1.1 * v2.0 * p2.0) + (v1.1 * v2.1 * p1.1) - (v1.1 * v2.1 * p2.1)) / -(v1.1 * v2.2 - v1.2 * v2.1))
         )
      } else if vector.y != Size::ZERO {
         Point::new(
            Size::millimeter(((v1.0 * v2.2 * p1.0) - (v1.2 * v2.0 * p2.0) + (v1.1 * v2.2 * p1.1) - (v1.2 * v2.1 * p2.1) + (v1.2 * v2.2 * p1.2) - (v1.2 * v2.2 * p2.2)) / -(v1.2 * v2.0 - v1.0 * v2.2)),
            Size::ZERO,
            Size::millimeter(((v1.1 * v2.0 * p1.1) - (v1.0 * v2.1 * p2.1) + (v1.2 * v2.0 * p1.2) - (v1.0 * v2.2 * p2.2) + (v1.0 * v2.0 * p1.0) - (v1.0 * v2.0 * p2.0)) /  (v1.2 * v2.0 - v1.0 * v2.2))
         )
      } else if vector.z != Size::ZERO {
         Point::new(
            Size::millimeter(((v1.2 * v2.1 * p1.2) - (v1.1 * v2.2 * p2.2) + (v1.0 * v2.1 * p1.0) - (v1.1 * v2.0 * p2.0) + (v1.1 * v2.1 * p1.1) - (v1.1 * v2.1 * p2.1)) /  (v1.0 * v2.1 - v1.1 * v2.0)),
            Size::millimeter(((v1.1 * v2.0 * p1.1) - (v1.0 * v2.1 * p2.1) + (v1.2 * v2.0 * p1.2) - (v2.2 * v1.0 * p2.2) + (v1.0 * v2.0 * p1.0) - (v1.0 * v2.0 * p2.0)) / -(v1.0 * v2.1 - v1.1 * v2.0)),
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

   fn intersection(self, rhs: Line) -> Point {
      let p1 = (
         self.point.x().to_millimeter(),
         self.point.y().to_millimeter(),
         self.point.z().to_millimeter()
      );

      let v1 = (
         self.normal_vector.x.to_millimeter(),
         self.normal_vector.y.to_millimeter(),
         self.normal_vector.z.to_millimeter()
      );

      let p2 = (
         rhs.point().x().to_millimeter(),
         rhs.point().y().to_millimeter(),
         rhs.point().z().to_millimeter()
      );

      let v2 = (
         rhs.vector().x.to_millimeter(),
         rhs.vector().y.to_millimeter(),
         rhs.vector().z.to_millimeter()
      );

      let t = ((p1.0 - p2.0) * v1.0 + (p1.1 - p2.1) * v1.1 + (p1.2 - p2.2) * v1.2) /
         (v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2);

      Point::new(
         Size::millimeter(p2.0 + t * v2.0),
         Size::millimeter(p2.1 + t * v2.1),
         Size::millimeter(p2.2 + t * v2.2)
      )
   }
}
