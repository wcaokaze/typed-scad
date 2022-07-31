use crate::solid::Solid;
use std::mem;

pub trait SolidParent: Solid {
   fn push<S: Solid + 'static>(&mut self, child: S) -> &mut S;
}

pub trait PushBorrowing {
   fn push_borrowing<S: Solid + 'static>(&mut self, value: S) -> &mut S;
}

impl PushBorrowing for Vec<Box<dyn Solid>> {
   fn push_borrowing<S: Solid + 'static>(&mut self, value: S) -> &mut S {
      self.push(Box::new(value));
      let trait_object_ref: &mut Box<dyn Solid> = self.last_mut().unwrap();
      let trait_object: &mut dyn Solid = trait_object_ref.as_mut();
      let (solid_ref, _): (&mut S, usize) = unsafe { mem::transmute(trait_object) };
      solid_ref
   }
}

#[cfg(test)]
mod test {
   use super::PushBorrowing;
   use crate::geometry::{Point, SizeLiteral, Vector};
   use crate::solid::Solid;
   use crate::stl::{Facet, StlSolid};
   use crate::transform::Transform;

   #[test]
   fn push_boxing() {
      struct SolidImpl(Facet);
      impl Solid for SolidImpl {
         fn generate_stl_solid(&self) -> StlSolid {
            StlSolid {
               facets: vec![
                  Facet { vertexes: self.0.vertexes }
               ]
            }
         }
      }

      let mut vec: Vec<Box<dyn Solid>> = vec![];

      let solid_impl = SolidImpl(
         Facet { vertexes: [Point::ORIGIN, Point::ORIGIN, Point::ORIGIN] }
      );
      let r = vec.push_borrowing(solid_impl);
      r.0.vertexes[0].translate(&Vector::X_UNIT_VECTOR);
      r.0.vertexes[1].translate(&Vector::Y_UNIT_VECTOR);

      let stl_solid = vec[0].generate_stl_solid();
      let expected =  [
         Point::new(1.mm(), 0.mm(), 0.mm()),
         Point::new(0.mm(), 1.mm(), 0.mm()),
         Point::ORIGIN
      ];
      assert_eq!(stl_solid.facets[0].vertexes, expected);
   }
}
