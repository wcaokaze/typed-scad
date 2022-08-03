use crate::geometry::{Angle, AngleLiteral};
use crate::solid::builder::BuildEnv;

pub static FRAGMENT_MINIMUM_ANGLE: BuildEnv<Angle> = BuildEnv::new(|| 12.deg());
