use crate::solid::Solid;
use std::mem;
use std::ops::ShlAssign;

pub trait SolidParent: Solid {
   fn push<S: Solid + 'static>(&mut self, child: S) -> &mut S;

   fn push_children(
      &mut self,
      generator: impl FnOnce(ChildReceiver<Self>) -> ()
   ) -> &mut Self {
      let child_receiver = ChildReceiver { parent: self };
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

pub struct ChildReceiver<'a, P: SolidParent + ?Sized> {
   parent: &'a mut P
}

impl<'a, P: SolidParent, S: Solid + 'static>
   ShlAssign<S> for ChildReceiver<'a, P>
{
   fn shl_assign(&mut self, rhs: S) {
      self.parent.push(rhs);
   }
}

#[cfg(test)]
mod test {
   use crate::geometry::{Point, SizeLiteral, Vector};
   use crate::solid::{Solid, SolidParent};
   use crate::solid::solid_parent::PushBorrowing;
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

   #[test]
   fn push_children() {
      let mut solid_parent = SolidParentImpl::new();
      solid_parent.push_children(|mut p| {
         p <<= SolidImpl::new(
            Facet {
               vertexes: [
                  Point::new(1.mm(), 2.mm(), 3.mm()),
                  Point::new(4.mm(), 5.mm(), 6.mm()),
                  Point::new(7.mm(), 8.mm(), 9.mm())
               ]
            }
         );

         p <<= SolidImpl::new(
            Facet {
               vertexes: [
                  Point::new(10.mm(), 11.mm(), 12.mm()),
                  Point::new(13.mm(), 14.mm(), 15.mm()),
                  Point::new(16.mm(), 17.mm(), 18.mm())
               ]
            }
         );
      });

      let stl_solid = solid_parent.generate_stl_solid();
      let expected = vec![
         Point::new( 1.mm(),  2.mm(),  3.mm()),
         Point::new( 4.mm(),  5.mm(),  6.mm()),
         Point::new( 7.mm(),  8.mm(),  9.mm()),
         Point::new(10.mm(), 11.mm(), 12.mm()),
         Point::new(13.mm(), 14.mm(), 15.mm()),
         Point::new(16.mm(), 17.mm(), 18.mm())
      ];
      assert_eq!(
         stl_solid.facets.iter()
            .flat_map(|f| f.vertexes)
            .collect::<Vec<_>>(),
         expected
      );
   }
}
