use crate::geometry::Vector;
use crate::solid::{Solid, SolidParent};
use crate::solid::builder::BuildContext;
use crate::solid::solid_parent::PushBorrowing;
use crate::stl::StlSolid;
use crate::transform::Transform;

pub struct Translate {
   pub offset: Vector,
   pub children: Vec<Box<dyn Solid>>
}

impl Translate {
   pub fn new(offset: Vector) -> Translate {
      Translate {
         offset,
         children: vec![]
      }
   }
}

pub fn translate(
   offset: Vector,
   build_action: impl FnOnce(BuildContext<Translate>)
) -> Translate {
   BuildContext::build(
      Translate::new(offset),
      build_action
   )
}

impl Solid for Translate {
   fn generate_stl_solid(&self) -> StlSolid {
      let mut stl_solid = StlSolid {
         facets: self.children.iter()
            .flat_map(|c| c.generate_stl_solid().facets)
            .collect()
      };

      for f in &mut stl_solid.facets {
         for v in &mut f.vertexes {
            v.translate(&self.offset);
         }
      }

      stl_solid
   }
}

impl SolidParent for Translate {
   fn push<S: Solid + 'static>(&mut self, child: S) -> &mut S {
      self.children.push_borrowing(child)
   }
}

#[cfg(test)]
mod tests {
   use super::translate;
   use crate::geometry::{Point, SizeLiteral, Vector};
   use crate::solid::Solid;
   use crate::stl::{Facet, StlSolid};

   #[test]
   fn vertexes() {
      struct Child;
      impl Solid for Child {
         fn generate_stl_solid(&self) -> StlSolid {
            StlSolid {
               facets: vec![
                  Facet {
                     vertexes: [
                        Point::new(0.mm(), 1.mm(), 2.mm()),
                        Point::new(3.mm(), 4.mm(), 5.mm()),
                        Point::new(6.mm(), 7.mm(), 8.mm())
                     ]
                  }
               ]
            }
         }
      }

      let t = translate(Vector::new(9.mm(), 10.mm(), 11.mm()), |mut c| {
         c <<= Child;
      });
      let s = t.generate_stl_solid();

      let actual: Vec<_> = s.facets.iter()
         .flat_map(|f| f.vertexes)
         .collect();
      let expected = vec![
         Point::new( 9.mm(), 11.mm(), 13.mm()),
         Point::new(12.mm(), 14.mm(), 16.mm()),
         Point::new(15.mm(), 17.mm(), 19.mm())
      ];

      assert_eq!(expected, actual);
   }
}
