use std::cmp::Ordering;

pub(crate) const FLOAT_POINT_ALLOWABLE_ERROR: f64 = 1e-10;

pub(crate) fn rough_partial_eq(s: f64, o: f64) -> bool {
   s > o - FLOAT_POINT_ALLOWABLE_ERROR && s < o + FLOAT_POINT_ALLOWABLE_ERROR
}

pub(crate) fn rough_partial_cmp(s: f64, o: f64) -> Option<Ordering> {
   match (s < o + FLOAT_POINT_ALLOWABLE_ERROR,
          s > o - FLOAT_POINT_ALLOWABLE_ERROR)
   {
      (false, false) => None,
      (false,  true) => Some(Ordering::Greater),
      ( true, false) => Some(Ordering::Less),
      ( true,  true) => Some(Ordering::Equal)
   }
}
