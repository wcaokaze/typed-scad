use once_cell::sync::Lazy;
use crate::geometry::{Angle, AngleLiteral};
use crate::solid::builder::BuildEnv;

pub static MINIMUM_ANGLE: Lazy<BuildEnv<Angle>> = Lazy::new(||
   BuildEnv::new(||
      12.deg()
   )
);
