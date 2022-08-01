use std::ops::{Deref, DerefMut, ShlAssign};
use crate::solid::builder::ChildReceiver;
use crate::solid::{Solid, SolidParent};

pub struct BuildContext<'a, P: SolidParent + ?Sized> {
   child_receiver: ChildReceiver<'a, P>
}

impl<P: SolidParent + 'static> BuildContext<'_, P> {
   pub fn build(
      mut solid_parent: P,
      build_action: impl FnOnce(BuildContext<P>) -> ()
   ) -> P {
      let context = BuildContext {
         child_receiver: ChildReceiver::new(&mut solid_parent)
      };
      build_action(context);
      solid_parent
   }
}

impl<'a, P: SolidParent + ?Sized> Deref for BuildContext<'a, P> {
   type Target = P;
   fn deref(&self) -> &P {
      self.child_receiver.parent
   }
}

impl<'a, P: SolidParent + ?Sized> DerefMut for BuildContext<'a, P> {
   fn deref_mut(&mut self) -> &mut P {
      self.child_receiver.parent
   }
}

impl<'a, P: SolidParent + ?Sized, S: Solid + 'static>
   ShlAssign<S> for BuildContext<'a, P>
{
   fn shl_assign(&mut self, rhs: S) {
      self.child_receiver <<= rhs;
   }
}

#[cfg(test)]
mod tests {
   use super::BuildContext;
   use crate::geometry::{Point, SizeLiteral};
   use crate::solid::{Solid, SolidParent};
   use crate::solid::solid_parent::PushBorrowing;
   use crate::stl::{Facet, StlSolid};

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

   fn solid(facet: Facet) -> SolidImpl {
      SolidImpl(facet)
   }

   struct SolidParentImpl {
      multiply: i32,
      children: Vec<Box<dyn Solid>>
   }

   impl SolidParentImpl {
      fn new(multiply: i32) -> SolidParentImpl {
         SolidParentImpl {
            multiply,
            children: vec![]
         }
      }
   }

   impl Solid for SolidParentImpl {
      fn generate_stl_solid(&self) -> StlSolid {
         StlSolid {
            facets: self.children.iter()
               .flat_map(|c|
                  c.generate_stl_solid().facets.into_iter().map(|f|
                     Facet {
                        vertexes: f.vertexes.map(|v|
                           Point {
                              matrix: v.matrix.map(|s|
                                 s * self.multiply
                              )
                           }
                        )
                     }
                  )
               )
               .collect()
         }
      }
   }

   impl SolidParent for SolidParentImpl {
      fn push<S: Solid + 'static>(&mut self, child: S) -> &mut S {
         self.children.push_borrowing(child)
      }
   }

   fn solid_parent(
      num: i32,
      build_action: impl FnOnce(BuildContext<SolidParentImpl>) -> ()
   ) -> SolidParentImpl {
      BuildContext::build(
         SolidParentImpl::new(num),
         build_action
      )
   }

   #[test]
   fn build_context() {
      let solid_parent = solid_parent(1, |mut p| {
         p <<= solid(
            Facet {
               vertexes: [
                  Point::new(1.mm(), 2.mm(), 3.mm()),
                  Point::new(4.mm(), 5.mm(), 6.mm()),
                  Point::new(7.mm(), 8.mm(), 9.mm())
               ]
            }
         );

         p <<= solid_parent(1, |mut p| {
            p.multiply += 1;

            p <<= solid(
               Facet {
                  vertexes: [
                     Point::new(1.mm(), 2.mm(), 3.mm()),
                     Point::new(4.mm(), 5.mm(), 6.mm()),
                     Point::new(7.mm(), 8.mm(), 9.mm())
                  ]
               }
            );

            p <<= solid(
               Facet {
                  vertexes: [
                     Point::new(10.mm(), 11.mm(), 12.mm()),
                     Point::new(13.mm(), 14.mm(), 15.mm()),
                     Point::new(16.mm(), 17.mm(), 18.mm())
                  ]
               }
            );
         });
      });

      let stl_solid = solid_parent.generate_stl_solid();
      let expected = vec![
         Point::new( 1.mm(),  2.mm(),  3.mm()),
         Point::new( 4.mm(),  5.mm(),  6.mm()),
         Point::new( 7.mm(),  8.mm(),  9.mm()),
         Point::new( 2.mm(),  4.mm(),  6.mm()),
         Point::new( 8.mm(), 10.mm(), 12.mm()),
         Point::new(14.mm(), 16.mm(), 18.mm()),
         Point::new(20.mm(), 22.mm(), 24.mm()),
         Point::new(26.mm(), 28.mm(), 30.mm()),
         Point::new(32.mm(), 34.mm(), 36.mm())
      ];
      assert_eq!(
         stl_solid.facets.iter()
            .flat_map(|f| f.vertexes)
            .collect::<Vec<_>>(),
         expected
      );
   }
}
