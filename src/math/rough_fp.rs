use std::cmp::Ordering;
use noisy_float::prelude::*;

pub(crate) const FLOAT_POINT_ALLOWABLE_ERROR: N64 = N64::unchecked_new(1e-10);

pub(crate) fn rough_eq(s: N64, o: N64) -> bool {
   s > o - FLOAT_POINT_ALLOWABLE_ERROR && s < o + FLOAT_POINT_ALLOWABLE_ERROR
}

pub(crate) fn rough_cmp(s: N64, o: N64) -> Ordering {
   if s > o + FLOAT_POINT_ALLOWABLE_ERROR {
      Ordering::Greater
   } else if s < o - FLOAT_POINT_ALLOWABLE_ERROR {
      Ordering::Less
   } else {
      Ordering::Equal
   }
}
