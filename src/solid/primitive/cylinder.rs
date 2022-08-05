use crate::geometry::{Angle, AngleLiteral, IterableAngleRange, Line, Size, Vector};
use crate::solid::{Location, Solid};
use crate::solid::precision::FRAGMENT_MINIMUM_ANGLE;
use crate::stl::{Facet, StlSolid};
use crate::transform::Transform;

pub struct Cylinder {
   pub location: Location,
   pub height: Size,
   pub radius: Size
}

impl Cylinder {
   pub fn new(location: Location, height: Size, radius: Size) -> Cylinder {
      Cylinder { location, height, radius }
   }
}

pub fn cylinder(location: Location, height: Size, radius: Size) -> Cylinder {
   Cylinder::new(location, height, radius)
}

impl Solid for Cylinder {
   fn generate_stl_solid(&self) -> StlSolid {
      let minimum_angle = *FRAGMENT_MINIMUM_ANGLE;

      let back = &self.location.back_vector();
      let top = &self.location.top_vector();
      let radius = self.radius;
      let height = self.height;
      let bottom_point = self.location.point();
      let top_point = bottom_point.translated_toward(top, height);

      let bottom_points: Vec<_>
         = Angle::iterate(0.deg()..360.deg()).step(minimum_angle)
         .map(|a| back.rotated(top, a))
         .map(|v| bottom_point.translated_toward(&v, radius))
         .collect();

      let top_points: Vec<_>
         = bottom_points.iter()
         .map(|p| p.translated_toward(top, height))
         .collect();

      let first_bottom = bottom_points.first();
      let shifted_bottom = bottom_points.iter().skip(1).chain(first_bottom);
      let zipped_bottom_points = bottom_points.iter().zip(shifted_bottom);

      let first_top = top_points.first();
      let shifted_top = top_points.iter().skip(1).chain(first_top);
      let zipped_top_points = top_points.iter().zip(shifted_top);

      let bottom_facets = zipped_bottom_points.clone().map(|(a, b)|
         Facet { vertexes: [bottom_point, *b, *a] }
      );

      let top_facets = zipped_top_points.clone().map(|(a, b)|
         Facet { vertexes: [top_point, *a, *b] }
      );

      let side_facets
         = zipped_bottom_points.zip(zipped_top_points)
         .flat_map(|((bottom_a, bottom_b), (top_a, top_b))|
            [
               Facet { vertexes: [*bottom_a, *top_b, *top_a] },
               Facet { vertexes: [*top_b, *bottom_a, *bottom_b] }
            ]
         );

      StlSolid {
         facets: bottom_facets
            .chain(side_facets)
            .chain(top_facets)
            .collect()
      }
   }
}

impl Transform for Cylinder {
   fn translated(&self, offset: &Vector) -> Self {
      Cylinder {
         location: self.location.translated(offset),
         height: self.height,
         radius: self.radius
      }
   }

   fn rotated(&self, axis: &Line, angle: Angle) -> Self {
      Cylinder {
         location: self.location.rotated(axis, angle),
         height: self.height,
         radius: self.radius
      }
   }
}

#[cfg(test)]
mod tests {
   use crate::geometry::{AngleLiteral, Point, SizeLiteral, Vector};
   use crate::solid::{cylinder, Location, Solid};
   use crate::solid::builder::env;
   use crate::solid::precision::FRAGMENT_MINIMUM_ANGLE;

   fn fragment_count() -> usize {
      (360.deg() / *FRAGMENT_MINIMUM_ANGLE).ceil() as usize
   }

   #[test]
   fn fragment_minimum_angle() {
      env(&FRAGMENT_MINIMUM_ANGLE, 2.deg(), || {
         let cylinder = cylinder(Location::default(), 3.mm(), 5.mm());
         let solid = cylinder.generate_stl_solid();

         assert_eq!(solid.facets.len(), fragment_count() * 4);
      });

      env(&FRAGMENT_MINIMUM_ANGLE, 24.deg(), || {
         let cylinder = cylinder(Location::default(), 3.mm(), 5.mm());
         let solid = cylinder.generate_stl_solid();

         assert_eq!(solid.facets.len(), fragment_count() * 4);
      });

      env(&FRAGMENT_MINIMUM_ANGLE, 360.deg(), || {
         let cylinder = cylinder(Location::default(), 3.mm(), 5.mm());
         let solid = cylinder.generate_stl_solid();

         assert_eq!(solid.facets.len(), 4);
      });
   }

   #[test]
   fn normal_vector() {
      let cylinder = cylinder(Location::default(), 3.mm(), 5.mm());
      let solid = cylinder.generate_stl_solid();

      solid.facets[0..fragment_count()]
         .iter()
         .map(|f| f.normal_vector())
         .for_each(|v| assert_eq!(v, -Vector::Z_UNIT_VECTOR));

      solid.facets[(fragment_count() * 3)..]
         .iter()
         .map(|f| f.normal_vector())
         .for_each(|v| assert_eq!(v, Vector::Z_UNIT_VECTOR));

      solid.facets[fragment_count()..(fragment_count() * 3)]
         .iter()
         .enumerate()
         .for_each(|(i, facet)| {
            let (a, b, pos) = if i % 2 == 0 {
               let [_, b, a] = facet.vertexes;
               (a, b, format!("{i} top"))
            } else {
               let [_, a, b] = facet.vertexes;
               (a, b, format!("{i} bottom"))
            };

            let expected = Vector::between(&a, &b)
               .vector_product(&Vector::Z_UNIT_VECTOR)
               .to_unit_vector();
            let actual = facet.normal_vector();
            assert_eq!(actual, expected, "at facet {pos}");
         });
   }

   #[test]
   fn height() {
      let cylinder = cylinder(Location::default(), 3.mm(), 5.mm());
      let solid = cylinder.generate_stl_solid();

      solid.facets.iter()
         .flat_map(|f| f.vertexes)
         .for_each(|v| assert!(v.z() == 0.mm() || v.z() == 3.mm()));
   }

   #[test]
   fn radius() {
      let cylinder = cylinder(Location::default(), 3.mm(), 5.mm());
      let solid = cylinder.generate_stl_solid();

      let (bottom_vertexes, top_vertexes): (Vec<_>, Vec<_>)
         = solid.facets.iter()
         .flat_map(|f| f.vertexes)
         .partition(|v| v.z() == 0.mm());

      let bottom_center = Point::ORIGIN;
      bottom_vertexes.iter()
         .filter(|&&v| v != bottom_center)
         .map(|v| Vector::between(&bottom_center, v))
         .for_each(|v| assert_eq!(v.norm(), 5.mm()));

      let top_center = Point::new(0.mm(), 0.mm(), 3.mm());
      top_vertexes.iter()
         .filter(|&&v| v != top_center)
         .map(|v| Vector::between(&top_center, v))
         .for_each(|v| assert_eq!(v.norm(), 5.mm()));
   }
}
