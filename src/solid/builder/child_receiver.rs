use std::ops::ShlAssign;
use crate::solid::{Solid, SolidParent};

pub struct ChildReceiver<'a, P: SolidParent + ?Sized> {
   parent: &'a mut P
}

impl<'a, P: SolidParent + ?Sized> ChildReceiver<'a, P> {
   pub(crate) fn new(parent: &mut P) -> ChildReceiver<P> {
      ChildReceiver { parent }
   }
}

impl<'a, P: SolidParent, S: Solid + 'static>
   ShlAssign<S> for ChildReceiver<'a, P>
{
   fn shl_assign(&mut self, rhs: S) {
      self.parent.push(rhs);
   }
}

#[cfg(test)]
mod tests {
   use crate::geometry::{Point, SizeLiteral};
   use crate::solid::{Solid, SolidParent};
   use crate::solid::solid_parent::PushBorrowing;
   use crate::stl::{Facet, StlSolid};

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
