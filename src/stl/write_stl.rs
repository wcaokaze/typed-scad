use crate::geometry::{Point, Size, Vector};
use crate::stl::stl_solid::{Facet, StlSolid};
use anyhow::Result;
use std::io::Write;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StlWriteError {
   #[error("TooManyFacets")]
   TooManyFacets,
}

/// Write the specified Solid as binary STL.
pub fn write_stl(output: &mut dyn Write, solid: &StlSolid) -> Result<()> {
   write_header(output)?;
   write_facets_count(output, solid.facets.len())?;
   for f in &solid.facets {
      write_facet(output, f)?;
   }

   Ok(())
}

fn write_header(output: &mut dyn Write) -> Result<()> {
   output.write_all(&[0; 80])?;
   Ok(())
}

fn write_facets_count(output: &mut dyn Write, facets_count: usize) -> Result<()> {
   if facets_count > u32::MAX as usize {
      return Err(StlWriteError::TooManyFacets.into());
   }

   let facets_count = facets_count as u32;
   output.write(&facets_count.to_le_bytes())?;

   Ok(())
}

fn write_facet(output: &mut dyn Write, facet: &Facet) -> Result<()> {
   write_vector(output, &facet.normal_vector())?;
   for v in &facet.vertexes {
      write_point(output, v)?;
   }
   output.write(&[0; 2])?;

   Ok(())
}

fn write_vector(output: &mut dyn Write, vector: &Vector) -> Result<()> {
   write_size(output, vector.x())?;
   write_size(output, vector.y())?;
   write_size(output, vector.z())?;
   Ok(())
}

fn write_point(output: &mut dyn Write, point: &Point) -> Result<()> {
   write_size(output, point.x())?;
   write_size(output, point.y())?;
   write_size(output, point.z())?;
   Ok(())
}

fn write_size(output: &mut dyn Write, size: Size) -> Result<()> {
   let f = size.0 as f32;
   output.write(&f.to_le_bytes())?;
   Ok(())
}

#[cfg(test)]
mod tests {
   use super::write_stl;
   use crate::geometry::{Point, Size};
   use crate::math::rough_fp::rough_partial_eq;
   use crate::stl::stl_solid::{Facet, StlSolid};

   macro_rules! solid {
      ($($f:expr),+) => (
         StlSolid {
            facets: vec![$($f),+]
         }
      );
   }

   fn facet(v1: Point, v2: Point, v3: Point) -> Facet {
      Facet { vertexes: [v1, v2, v3] }
   }

   fn vertex(x: i32, y: i32, z: i32) -> Point {
      Point::new(Size(x as f64), Size(y as f64), Size(z as f64))
   }

   #[test]
   fn write() {
      let solid = solid!(
         facet(
            vertex(0, 0, 0),
            vertex(10, 0, 0),
            vertex(0, 0, 10)
         ),
         facet(
            vertex(10, 0, 0),
            vertex(0, 10, 0),
            vertex(0, 0, 10)
         ),
         facet(
            vertex(0, 0, 0),
            vertex(0, 0, 10),
            vertex(0, 10, 0)
         ),
         facet(
            vertex(0, 0, 0),
            vertex(0, 10, 0),
            vertex(10, 0, 0)
         )
      );

      let mut output = vec![];
      write_stl(&mut output, &solid).unwrap();

      // -------- assert stl length

      let header_bytes = 80;
      let facet_count_bytes = 4;
      let facet_bytes = {
         let vector_bytes = 4 * 3;
         let vertex_bytes = 4 * 3;
         let unused_bytes = 2;
         vector_bytes + vertex_bytes * 3 + unused_bytes
      };
      assert_eq!(
         output.len(),
         header_bytes + facet_count_bytes + facet_bytes * solid.facets.len()
      );

      // -------- assert facet count data

      assert_eq!(
         u32_at(&output, header_bytes),
         solid.facets.len() as u32
      );

      // -------- assert facet data

      for i in 0..solid.facets.len() {
         let facet_start = header_bytes + facet_count_bytes + facet_bytes * i;

         let normal_vector = solid.facets[i].normal_vector();
         assert_rough_eq(f32_at(&output, facet_start +  0), normal_vector.x().0 as f32);
         assert_rough_eq(f32_at(&output, facet_start +  4), normal_vector.y().0 as f32);
         assert_rough_eq(f32_at(&output, facet_start +  8), normal_vector.z().0 as f32);

         let vertex1 = solid.facets[i].vertexes[0];
         assert_rough_eq(f32_at(&output, facet_start + 12), vertex1.x().0 as f32);
         assert_rough_eq(f32_at(&output, facet_start + 16), vertex1.y().0 as f32);
         assert_rough_eq(f32_at(&output, facet_start + 20), vertex1.z().0 as f32);

         let vertex2 = solid.facets[i].vertexes[1];
         assert_rough_eq(f32_at(&output, facet_start + 24), vertex2.x().0 as f32);
         assert_rough_eq(f32_at(&output, facet_start + 28), vertex2.y().0 as f32);
         assert_rough_eq(f32_at(&output, facet_start + 32), vertex2.z().0 as f32);

         let vertex3 = solid.facets[i].vertexes[2];
         assert_rough_eq(f32_at(&output, facet_start + 36), vertex3.x().0 as f32);
         assert_rough_eq(f32_at(&output, facet_start + 40), vertex3.y().0 as f32);
         assert_rough_eq(f32_at(&output, facet_start + 44), vertex3.z().0 as f32);
      }
   }

   fn u32_at(vec: &Vec<u8>, index: usize) -> u32 {
      u32::from_le_bytes(vec[index..(index + 4)].try_into().unwrap())
   }

   fn f32_at(vec: &Vec<u8>, index: usize) -> f32 {
      f32::from_le_bytes(vec[index..(index + 4)].try_into().unwrap())
   }

   fn assert_rough_eq(a: f32, b: f32) {
      assert!(
         rough_partial_eq(a as f64, b as f64),
         "left: {a}, right: {b}"
      );
   }
}
