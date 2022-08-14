use crate::geometry::{Angle, Line, Point, Vector};
use crate::solid::builder::BuildContext;
use crate::solid::{Solid, SolidParent};
use crate::solid::solid_parent::PushBorrowing;
use crate::stl::StlSolid;
use crate::transform::Transform;
use std::mem;

pub struct Rotate {
   pub axis: Line,
   pub angle: Angle,
   pub children: Vec<Box<dyn Solid>>
}

impl Rotate {
   pub fn new(axis: Line, angle: Angle) -> Self {
      Self {
         axis,
         angle,
         children: vec![]
      }
   }
}

pub fn rotate(
   axis: Line,
   angle: Angle,
   build_action: impl FnOnce(BuildContext<Rotate>)
) -> Rotate {
   BuildContext::build(
      Rotate::new(axis, angle),
      build_action
   )
}

impl Solid for Rotate {
   fn generate_stl_solid(&self) -> StlSolid {
      let mut stl_solid = StlSolid {
         facets: self.children.iter()
            .flat_map(|c| c.generate_stl_solid().facets)
            .collect()
      };

      if self.axis.point() == Point::ORIGIN {
         let axis = self.axis.vector();
         for f in &mut stl_solid.facets {
            for v in &mut f.vertexes {
               unsafe {
                  mem::transmute::<&mut Point, &mut Vector>(v)
                     .rotate(axis, self.angle);
               }
            }
         }
      } else {
         for f in &mut stl_solid.facets {
            for v in &mut f.vertexes {
               v.rotate(&self.axis, self.angle);
            }
         }
      }

      stl_solid
   }
}

impl SolidParent for Rotate {
   fn push<S: Solid + 'static>(&mut self, child: S) -> &mut S {
      self.children.push_borrowing(child)
   }
}

#[cfg(test)]
mod tests {
   use super::rotate;
   use crate::geometry::{AngleLiteral, Line, Point, SizeLiteral, Vector};
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

      let r = rotate(Line::Z_AXIS, 90.deg(), |mut c| {
         c <<= Child;
      });
      let s = r.generate_stl_solid();

      let actual: Vec<_> = s.facets.iter()
         .flat_map(|f| f.vertexes)
         .collect();
      let expected = vec![
         Point::new(-1.mm(), 0.mm(), 2.mm()),
         Point::new(-4.mm(), 3.mm(), 5.mm()),
         Point::new(-7.mm(), 6.mm(), 8.mm())
      ];

      assert_eq!(expected, actual);

      let axis = Line::new(
         &Point::new(1.mm(), 1.mm(), 0.mm()),
         &Vector::Z_UNIT_VECTOR
      );
      let r = rotate(axis, 90.deg(), |mut c| {
         c <<= Child;
      });
      let s = r.generate_stl_solid();

      let actual: Vec<_> = s.facets.iter()
         .flat_map(|f| f.vertexes)
         .collect();
      let expected = vec![
         Point::new( 1.mm(), 0.mm(), 2.mm()),
         Point::new(-2.mm(), 3.mm(), 5.mm()),
         Point::new(-5.mm(), 6.mm(), 8.mm())
      ];

      assert_eq!(expected, actual);
   }
}
