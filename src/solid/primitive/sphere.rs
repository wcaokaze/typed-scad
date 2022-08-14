use crate::geometry::{Angle, AngleLiteral, Line, Point, Size, Vector};
use crate::solid::{Location, Solid};
use crate::solid::precision::FRAGMENT_MINIMUM_ANGLE;
use crate::stl::{Facet, StlSolid};
use crate::transform::Transform;
use rayon::prelude::{
   IndexedParallelIterator, IntoParallelIterator, ParallelIterator
};
use std::{array, ptr, slice};

pub struct Sphere {
   pub location: Location,
   pub radius: Size
}

impl Sphere {
   pub fn new(location: Location, radius: Size) -> Sphere {
      Sphere { location, radius }
   }
}

pub fn sphere(location: Location, radius: Size) -> Sphere {
   Sphere::new(location, radius)
}

impl Solid for Sphere {
   fn generate_stl_solid(&self) -> StlSolid {
      let angles = Angle::par_iterate(0.deg()..90.deg())
         .step(*FRAGMENT_MINIMUM_ANGLE);
      let shifted_angles = angles.clone().skip(1).chain([90.deg()]);
      let zipped_angles = angles.zip(shifted_angles);

      let p = Point::ORIGIN
         .translated_toward(&Vector::Z_UNIT_VECTOR, self.radius);
      let mut facets: Vec<_> = zipped_angles.clone()
         .flat_map(|(yz_angle_a, yz_angle_b)| {
            let a = p.rotated(&Line::X_AXIS, yz_angle_a);
            let b = p.rotated(&Line::X_AXIS, yz_angle_b);

            zipped_angles.clone().flat_map(move |(zx_angle_a, zx_angle_b)| {
               let aa = a.rotated(&Line::Y_AXIS, zx_angle_a);
               let ab = a.rotated(&Line::Y_AXIS, zx_angle_b);
               let ba = b.rotated(&Line::Y_AXIS, zx_angle_a);
               let bb = b.rotated(&Line::Y_AXIS, zx_angle_b);

               let aa_facet = Some(Facet { vertexes: [aa, ba, ab] });

               let bb_facet = if ba != bb { // when `b` is not on Y axis
                  Some(Facet { vertexes: [bb, ab, ba] })
               } else {
                  None
               };

               [aa_facet, bb_facet].into_par_iter().flatten()
            })
         })
         .collect();

      copy_elements::<_, 7>(&mut facets)
         .into_par_iter()
         .enumerate()
         .flat_map(|(i, facets)|
            facets.into_par_iter().map(move |f| (i, f))
         )
         .for_each(|(i, f)| {
            let x_negative = i & 0b001 != 0;
            let y_negative = i & 0b010 != 0;
            let z_negative = i & 0b100 != 0;

            negative(f, x_negative, y_negative, z_negative);

            if x_negative ^ y_negative ^ z_negative {
               reverse(f);
            }

            locate(f, &self.location);
         });

      StlSolid { facets }
   }
}

fn copy_elements<T, const COUNT: usize>(
   vec: &mut Vec<T>
) -> [&mut [T]; COUNT + 1] {
   let len = vec.len();
   vec.reserve(len * COUNT);
   unsafe {
      vec.set_len(len * (COUNT + 1));
   }

   let mut slices = unsafe {
      array::from_fn(|i| {
         slice::from_raw_parts_mut(vec.as_mut_ptr().add(i * len), len)
      })
   };

   let src = slices[0].as_ptr();

   for dest in &mut slices[1..] {
      unsafe {
         ptr::copy_nonoverlapping::<T>(src, dest.as_mut_ptr(), len);
      }
   }

   slices
}

fn negative(facet: &mut Facet, x: bool, y: bool, z: bool) {
   for v in &mut facet.vertexes {
      if x {
         v.matrix.0[0][0] = -v.matrix.0[0][0];
      }
      if y {
         v.matrix.0[1][0] = -v.matrix.0[1][0];
      }
      if z {
         v.matrix.0[2][0] = -v.matrix.0[2][0];
      }
   }
}

fn reverse(facet: &mut Facet) {
   let v = facet.vertexes[1];
   facet.vertexes[1] = facet.vertexes[2];
   facet.vertexes[2] = v;
}

fn locate(facet: &mut Facet, location: &Location) {
   let offset = Vector::between(&Point::ORIGIN, &location.point());
   for v in &mut facet.vertexes {
      v.translate(&offset);
   }
}

impl Transform for Sphere {
   fn translated(&self, offset: &Vector) -> Self {
      Self {
         location: self.location.translated(offset),
         radius: self.radius
      }
   }

   fn rotated(&self, axis: &Line, angle: Angle) -> Self {
      Self {
         location: self.location.rotated(axis, angle),
         radius: self.radius
      }
   }
}

#[cfg(test)]
mod tests {
   use super::sphere;
   use crate::geometry::{AngleLiteral, Point, SizeLiteral, Vector};
   use crate::solid::{Location, Solid};

   #[test]
   fn normal_vector() {
      let sphere = sphere(Location::default(), 3.mm());
      let solid = sphere.generate_stl_solid();

      for f in solid.facets {
         let expected = Vector::between(&Point::ORIGIN, &f.vertexes[0]);
         let actual = f.normal_vector();
         assert!(
            expected.angle_with(&actual) < 10.deg(),
            "expected: {:?}, actual: {:?} at {:?}",
            expected.to_unit_vector(), actual, f.vertexes[0]
         );
      }
   }

   #[test]
   fn radius() {
      let sphere = sphere(Location::default(), 3.mm());
      let solid = sphere.generate_stl_solid();

      solid.facets.iter()
         .flat_map(|f| f.vertexes)
         .for_each(|v|
            assert_eq!(Point::ORIGIN.distance(&v), 3.mm())
         );
   }
}
