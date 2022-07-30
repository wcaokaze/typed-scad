use crate::stl::{StlSolid, write_stl};
use anyhow::Result;
use std::io::Write;

pub trait Solid {
   fn generate_stl_solid(&self) -> StlSolid;

   fn write_to(&self, output: &mut dyn Write) -> Result<()> {
      let stl_solid = self.generate_stl_solid();
      write_stl(output, &stl_solid)?;
      Ok(())
   }

   fn build(builder: impl FnOnce(&mut Self) -> ()) -> Self
      where Self: Default
   {
      let mut solid = Self::default();
      builder(&mut solid);
      solid
   }
}

#[cfg(test)]
mod test {
   use super::Solid;
   use crate::stl::StlSolid;

   #[test]
   fn build() {
      struct SolidImpl(i32);
      impl Default for SolidImpl {
         fn default() -> SolidImpl {
            SolidImpl(0)
         }
      }
      impl Solid for SolidImpl {
         fn generate_stl_solid(&self) -> StlSolid {
            panic!();
         }
      }

      let solid_impl = SolidImpl::build(|s| {
         s.0 = 42;
      });

      assert_eq!(solid_impl.0, 42);
   }
}
