use crate::geometry::{Angle, Line, Size, Vector};
use crate::solid::{Location, Solid};
use crate::stl::{Facet, StlSolid};
use crate::transform::Transform;

pub struct Cube {
   location: Location,
   size: (Size, Size, Size)
}

impl Cube {
   pub fn new(location: Location, size: (Size, Size, Size)) -> Cube {
      Cube { location, size }
   }
}

pub fn cube(location: Location, size: (Size, Size, Size)) -> Cube {
   Cube::new(location, size)
}

impl Solid for Cube {
   fn generate_stl_solid(&self) -> StlSolid {
      let point = self.location.point();
      let right_vector = self.location.right_vector();
      let back_vector = self.location.back_vector();
      let top_vector = self.location.top_vector();

      let p = |x, y, z| {
         point
            .translated_toward(&right_vector, x)
            .translated_toward(&back_vector, y)
            .translated_toward(&top_vector, z)
      };

      let size_0 = Size::ZERO;
      let (size_x, size_y, size_z) = self.size;

      let left_front_bottom  = p(size_0, size_0, size_0);
      let right_front_bottom = p(size_x, size_0, size_0);
      let left_back_bottom   = p(size_0, size_y, size_0);
      let right_back_bottom  = p(size_x, size_y, size_0);
      let left_front_top     = p(size_0, size_0, size_z);
      let right_front_top    = p(size_x, size_0, size_z);
      let left_back_top      = p(size_0, size_y, size_z);
      let right_back_top     = p(size_x, size_y, size_z);

      StlSolid {
         facets: vec![
            Facet { vertexes: [left_front_bottom, left_back_bottom, right_back_bottom] },
            Facet { vertexes: [right_back_bottom, right_front_bottom, left_front_bottom] },
            Facet { vertexes: [left_front_bottom, right_front_bottom, right_front_top] },
            Facet { vertexes: [right_front_top, left_front_top, left_front_bottom] },
            Facet { vertexes: [right_front_bottom, right_back_bottom, right_front_top] },
            Facet { vertexes: [right_back_top, right_front_top, right_back_bottom] },
            Facet { vertexes: [right_back_bottom, left_back_bottom, right_back_top] },
            Facet { vertexes: [left_back_top, right_back_top, left_back_bottom] },
            Facet { vertexes: [left_back_bottom, left_front_bottom, left_back_top] },
            Facet { vertexes: [left_front_top, left_back_top, left_front_bottom] },
            Facet { vertexes: [left_front_top, right_front_top, right_back_top] },
            Facet { vertexes: [right_back_top, left_back_top, left_front_top] }
         ]
      }
   }
}

impl Transform for Cube {
   fn translated(&self, offset: &Vector) -> Cube {
      Cube {
         location: self.location.translated(offset),
         size: self.size
      }
   }

   fn rotated(&self, axis: &Line, angle: Angle) -> Cube {
      Cube {
         location: self.location.rotated(axis, angle),
         size: self.size
      }
   }
}

#[cfg(test)]
mod tests {
   use super::cube;
   use crate::geometry::{Point, SizeLiteral, Vector};
   use crate::solid::{Location, Solid};
   use crate::stl::Facet;

   #[test]
   fn planes() {
      fn assert_plane(
         facet: &Facet,
         expected_points: &[Point],
         expected_normal_vector: &Vector
      ) {
         assert!(
            facet.vertexes.iter().all(|v| expected_points.contains(v))
         );

         assert_eq!(
            facet.normal_vector(),
            *expected_normal_vector
         );
      }

      let cube = cube(Location::default(), (1.mm(), 1.mm(), 1.mm()));
      let solid = cube.generate_stl_solid();

      let expected_points = [
         Point::new(0.mm(), 0.mm(), 0.mm()),
         Point::new(1.mm(), 0.mm(), 0.mm()),
         Point::new(0.mm(), 1.mm(), 0.mm()),
         Point::new(1.mm(), 1.mm(), 0.mm())
      ];
      assert_plane(&solid.facets[0], &expected_points, &-Vector::Z_UNIT_VECTOR);
      assert_plane(&solid.facets[1], &expected_points, &-Vector::Z_UNIT_VECTOR);

      let expected_points = [
         Point::new(0.mm(), 0.mm(), 0.mm()),
         Point::new(1.mm(), 0.mm(), 0.mm()),
         Point::new(0.mm(), 0.mm(), 1.mm()),
         Point::new(1.mm(), 0.mm(), 1.mm())
      ];
      assert_plane(&solid.facets[2], &expected_points, &-Vector::Y_UNIT_VECTOR);
      assert_plane(&solid.facets[3], &expected_points, &-Vector::Y_UNIT_VECTOR);

      let expected_points = [
         Point::new(1.mm(), 0.mm(), 0.mm()),
         Point::new(1.mm(), 1.mm(), 0.mm()),
         Point::new(1.mm(), 0.mm(), 1.mm()),
         Point::new(1.mm(), 1.mm(), 1.mm())
      ];
      assert_plane(&solid.facets[4], &expected_points, &Vector::X_UNIT_VECTOR);
      assert_plane(&solid.facets[5], &expected_points, &Vector::X_UNIT_VECTOR);

      let expected_points = [
         Point::new(0.mm(), 1.mm(), 0.mm()),
         Point::new(1.mm(), 1.mm(), 0.mm()),
         Point::new(0.mm(), 1.mm(), 1.mm()),
         Point::new(1.mm(), 1.mm(), 1.mm())
      ];
      assert_plane(&solid.facets[6], &expected_points, &Vector::Y_UNIT_VECTOR);
      assert_plane(&solid.facets[7], &expected_points, &Vector::Y_UNIT_VECTOR);

      let expected_points = [
         Point::new(0.mm(), 0.mm(), 0.mm()),
         Point::new(0.mm(), 1.mm(), 0.mm()),
         Point::new(0.mm(), 0.mm(), 1.mm()),
         Point::new(0.mm(), 1.mm(), 1.mm())
      ];
      assert_plane(&solid.facets[8], &expected_points, &-Vector::X_UNIT_VECTOR);
      assert_plane(&solid.facets[9], &expected_points, &-Vector::X_UNIT_VECTOR);

      let expected_points = [
         Point::new(0.mm(), 0.mm(), 1.mm()),
         Point::new(1.mm(), 0.mm(), 1.mm()),
         Point::new(0.mm(), 1.mm(), 1.mm()),
         Point::new(1.mm(), 1.mm(), 1.mm())
      ];
      assert_plane(&solid.facets[10], &expected_points, &Vector::Z_UNIT_VECTOR);
      assert_plane(&solid.facets[11], &expected_points, &Vector::Z_UNIT_VECTOR);
   }
}
