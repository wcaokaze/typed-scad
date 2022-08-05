use crate::geometry::{Angle, AngleLiteral, IterableAngleRange, Line, Size, Vector};
use crate::solid::{Location, Solid};
use crate::solid::precision::FRAGMENT_MINIMUM_ANGLE;
use crate::stl::{Facet, StlSolid};
use crate::transform::Transform;

pub struct Cone {
   pub location: Location,
   pub height: Size,
   pub bottom_radius: Size
}

impl Cone {
   pub fn new(location: Location, height: Size, bottom_radius: Size) -> Cone {
      Cone { location, height, bottom_radius }
   }
}

pub fn cone(location: Location, height: Size, bottom_radius: Size) -> Cone {
   Cone::new(location, height, bottom_radius)
}

impl Solid for Cone {
   fn generate_stl_solid(&self) -> StlSolid {
      let minimum_angle = *FRAGMENT_MINIMUM_ANGLE;

      let back = &self.location.back_vector();
      let top = &self.location.top_vector();
      let radius = self.bottom_radius;
      let height = self.height;
      let bottom_point = self.location.point();
      let top_point = bottom_point.translated_toward(top, height);

      let points: Vec<_>
         = Angle::iterate(0.deg()..360.deg()).step(minimum_angle)
         .map(|a| back.rotated(top, a))
         .map(|v| bottom_point.translated_toward(&v, radius))
         .collect();

      let first_point = points.first();
      let shifted_points = points.iter().skip(1).chain(first_point);
      let zipped_points = points.iter().zip(shifted_points);

      let bottom_facets = zipped_points.clone().map(|(a, b)|
         Facet { vertexes: [bottom_point, *b, *a] }
      );

      let side_facets = zipped_points.map(|(a, b)|
         Facet { vertexes: [*a, *b, top_point] }
      );

      StlSolid {
         facets: bottom_facets
            .chain(side_facets)
            .collect()
      }
   }
}

impl Transform for Cone {
   fn translated(&self, offset: &Vector) -> Self {
      Self {
         location: self.location.translated(offset),
         height: self.height,
         bottom_radius: self.bottom_radius
      }
   }

   fn rotated(&self, axis: &Line, angle: Angle) -> Self {
      Self {
         location: self.location.rotated(axis, angle),
         height: self.height,
         bottom_radius: self.bottom_radius
      }
   }
}

#[cfg(test)]
mod tests {
   use super::cone;
   use crate::geometry::{AngleLiteral, Point, SizeLiteral, Vector};
   use crate::solid::{Location, Solid};
   use crate::solid::builder::env;
   use crate::solid::precision::FRAGMENT_MINIMUM_ANGLE;

   fn fragment_count() -> usize {
      (360.deg() / *FRAGMENT_MINIMUM_ANGLE).ceil() as usize
   }

   #[test]
   fn fragment_minimum_angle() {
      env(&FRAGMENT_MINIMUM_ANGLE, 2.deg(), || {
         let cone = cone(Location::default(), 3.mm(), 5.mm());
         let solid = cone.generate_stl_solid();

         assert_eq!(solid.facets.len(), fragment_count() * 2);
      });

      env(&FRAGMENT_MINIMUM_ANGLE, 24.deg(), || {
         let cone = cone(Location::default(), 3.mm(), 5.mm());
         let solid = cone.generate_stl_solid();

         assert_eq!(solid.facets.len(), fragment_count() * 2);
      });

      env(&FRAGMENT_MINIMUM_ANGLE, 360.deg(), || {
         let cone = cone(Location::default(), 3.mm(), 5.mm());
         let solid = cone.generate_stl_solid();

         assert_eq!(solid.facets.len(), 2);
      });
   }

   #[test]
   fn normal_vector() {
      let cone = cone(Location::default(), 3.mm(), 5.mm());
      let solid = cone.generate_stl_solid();

      solid.facets[0..fragment_count()]
         .iter()
         .map(|f| f.normal_vector())
         .for_each(|v| assert_eq!(v, -Vector::Z_UNIT_VECTOR));

      solid.facets[fragment_count()..]
         .iter()
         .enumerate()
         .for_each(|(i, f)| {
            let [a, b, _] = f.vertexes;

            let expected = Vector::between(&a, &b)
               .vector_product(&Vector::new(-a.x(), -a.y(), 3.mm()))
               .to_unit_vector();
            let actual = f.normal_vector();
            assert_eq!(actual, expected, "at facet {i}");
         });
   }

   #[test]
   fn height() {
      let cone = cone(Location::default(), 3.mm(), 5.mm());
      let solid = cone.generate_stl_solid();

      solid.facets.iter()
         .flat_map(|f| f.vertexes)
         .for_each(|v| assert!(v.z() == 0.mm() || v.z() == 3.mm()));
   }

   #[test]
   fn radius() {
      let cone = cone(Location::default(), 3.mm(), 5.mm());
      let solid = cone.generate_stl_solid();

      solid.facets[0..fragment_count()]
         .iter()
         .flat_map(|f| f.vertexes)
         .filter(|&v| v != Point::ORIGIN)
         .map(|v| Vector::between(&Point::ORIGIN, &v))
         .for_each(|v| assert_eq!(v.norm(), 5.mm()));
   }
}
