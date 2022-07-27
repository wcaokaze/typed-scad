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
}
