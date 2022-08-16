use crate::geometry::Point;
use crate::solid::builder::BuildContext;
use crate::solid::{Solid, SolidParent};
use crate::solid::solid_parent::PushBorrowing;
use crate::stl::StlSolid;

pub struct Scale {
   pub scale: f64,
   pub scale_origin: Point,
   pub children: Vec<Box<dyn Solid>>
}

impl Scale {
   pub fn new(scale: f64, scale_origin: Point) -> Scale {
      Scale {
         scale,
         scale_origin,
         children: vec![]
      }
   }
}

pub fn scale(
   scale: f64,
   scale_origin: Point,
   build_action: impl FnOnce(BuildContext<Scale>)
) -> Scale {
   BuildContext::build(
      Scale::new(scale, scale_origin),
      build_action
   )
}

impl Solid for Scale {
   fn generate_stl_solid(&self) -> StlSolid {
      let mut stl_solid = StlSolid {
         facets: self.children.iter()
            .flat_map(|c| c.generate_stl_solid().facets)
            .collect()
      };

      if self.scale_origin == Point::ORIGIN {
         for f in &mut stl_solid.facets {
            for v in &mut f.vertexes {
               v.matrix *= self.scale;
            }
         }
      } else {
         for f in &mut stl_solid.facets {
            for v in &mut f.vertexes {
               v.matrix -= self.scale_origin.matrix;
               v.matrix *= self.scale;
               v.matrix += self.scale_origin.matrix;
            }
         }
      }

      stl_solid
   }
}

impl SolidParent for Scale {
   fn push<S: Solid + 'static>(&mut self, child: S) -> &mut S {
      self.children.push_borrowing(child)
   }
}

#[cfg(test)]
mod tests {
   use crate::geometry::{Point, SizeLiteral};
   use crate::solid::Solid;
   use crate::stl::{Facet, StlSolid};
   use super::scale;

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

      let s = scale(1.5, Point::ORIGIN, |mut c| {
         c <<= Child;
      });
      let s = s.generate_stl_solid();

      let actual: Vec<_> = s.facets.iter()
         .flat_map(|f| f.vertexes)
         .collect();
      let expected = vec![
         Point::new(0.0.mm(),  1.5.mm(),  3.0.mm()),
         Point::new(4.5.mm(),  6.0.mm(),  7.5.mm()),
         Point::new(9.0.mm(), 10.5.mm(), 12.0.mm())
      ];

      assert_eq!(expected, actual);

      let scale_origin = Point::new(1.mm(), 1.mm(), 1.mm());
      let s = scale(1.5, scale_origin, |mut c| {
         c <<= Child;
      });
      let s = s.generate_stl_solid();

      let actual: Vec<_> = s.facets.iter()
         .flat_map(|f| f.vertexes)
         .collect();
      let expected = vec![
         Point::new(-0.5.mm(),  1.0.mm(),  2.5.mm()),
         Point::new( 4.0.mm(),  5.5.mm(),  7.0.mm()),
         Point::new( 8.5.mm(), 10.0.mm(), 11.5.mm())
      ];

      assert_eq!(expected, actual);
   }
}
