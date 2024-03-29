use crate::solid::builder::ChildReceiver;
use crate::solid::Solid;
use std::mem;

pub trait SolidParent: Solid {
   fn push<S: Solid + 'static>(&mut self, child: S) -> &mut S;

   fn push_children(
      &mut self,
      generator: impl FnOnce(ChildReceiver<Self>) -> ()
   ) -> &mut Self {
      let child_receiver = ChildReceiver::new(self);
      generator(child_receiver);
      self
   }
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
   use crate::solid::{Solid, SolidParent};
   use crate::stl::{Facet, StlSolid};
   use crate::transform::Transform;

   struct SolidImpl(Facet);

   impl SolidImpl {
      fn new(facet: Facet) -> SolidImpl {
         SolidImpl(facet)
      }
   }

   impl Solid for SolidImpl {
      fn generate_stl_solid(&self) -> StlSolid {
         StlSolid {
            facets: vec![
               Facet { vertexes: self.0.vertexes }
            ]
         }
      }
   }

   struct SolidParentImpl(Vec<Box<dyn Solid>>);

   impl SolidParentImpl {
      fn new() -> SolidParentImpl {
         SolidParentImpl(vec![])
      }
   }

   impl Solid for SolidParentImpl {
      fn generate_stl_solid(&self) -> StlSolid {
         StlSolid {
            facets: self.0.iter()
               .flat_map(|c| c.generate_stl_solid().facets)
               .collect()
         }
      }
   }

   impl SolidParent for SolidParentImpl {
      fn push<S: Solid + 'static>(&mut self, child: S) -> &mut S {
         self.0.push_borrowing(child)
      }
   }

   #[test]
   fn push() {
      let mut solid_parent = SolidParentImpl::new();
      let solid = SolidImpl::new(
         Facet { vertexes: [Point::ORIGIN, Point::ORIGIN, Point::ORIGIN] }
      );

      let r = solid_parent.push(solid);
      r.0.vertexes[0].translate(&Vector::X_UNIT_VECTOR);
      r.0.vertexes[1].translate(&Vector::Y_UNIT_VECTOR);

      let stl_solid = solid_parent.generate_stl_solid();
      let expected = vec![
         Point::new(1.mm(), 0.mm(), 0.mm()),
         Point::new(0.mm(), 1.mm(), 0.mm()),
         Point::ORIGIN
      ];
      assert_eq!(
         stl_solid.facets.iter()
            .flat_map(|f| f.vertexes)
            .collect::<Vec<_>>(),
         expected
      );
   }
}
