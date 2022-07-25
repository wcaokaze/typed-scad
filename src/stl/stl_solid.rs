use crate::geometry::{Angle, Line, Point, Vector};
use crate::transform::Transform;

/// STL Solid. This can be written as STL. (See [crate::stl::write_stl])
pub struct StlSolid {
   pub(crate) facets: Vec<Facet>
}

pub(crate) struct Facet {
   pub(crate) vertexes: [Point; 3]
}

impl Facet {
   pub(crate) fn normal_vector(&self) -> Vector {
      let v1 = Vector::between(&self.vertexes[0], &self.vertexes[1]);
      let v2 = Vector::between(&self.vertexes[1], &self.vertexes[2]);
      v1.vector_product(&v2).to_unit_vector()
   }
}

impl Transform for StlSolid {
   fn translated(&self, offset: &Vector) -> StlSolid {
      let facets = self.facets.iter()
         .map(|f| {
            let vertexes = f.vertexes.map(|v| v.translated(offset));
            Facet { vertexes }
         })
         .collect();

      StlSolid { facets }
   }

   fn rotated(&self, axis: &Line, angle: Angle) -> StlSolid {
      let facets = self.facets.iter()
         .map(|f| {
            let vertexes = f.vertexes.map(|v| v.rotated(axis, angle));
            Facet { vertexes }
         })
         .collect();

      StlSolid { facets }
   }
}

#[cfg(test)]
mod tests {
   use crate::geometry::{Point, SizeLiteral, Vector};
   use super::Facet;

   #[test]
   fn facet_normal_vector() {
      let facet = Facet {
         vertexes: [
            Point::ORIGIN,
            Point::new(2.mm(), 4.mm(), 0.mm()),
            Point::new(-2.mm(), 6.mm(), 0.mm())
         ]
      };

      assert_eq!(
         facet.normal_vector(),
         Vector::Z_UNIT_VECTOR
      );

      let facet = Facet {
         vertexes: [
            Point::ORIGIN,
            Point::new(0.mm(), 0.mm(), 3.mm()),
            Point::new(2.mm(), 2.mm(), 0.mm())
         ]
      };

      assert_eq!(
         facet.normal_vector(),
         Vector::new(-1.mm(), 1.mm(), 0.mm()).to_unit_vector()
      );
   }
}
