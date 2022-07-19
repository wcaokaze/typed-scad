use std::cmp::Ordering;

const D: f64 = 1e-10;

pub(crate) fn rough_partial_eq(s: f64, o: f64) -> bool {
   s > o - D && s < o + D
}

pub(crate) fn rough_partial_cmp(s: f64, o: f64) -> Option<Ordering> {
   match (s < o + D, s > o - D) {
      (false, false) => None,
      (false,  true) => Some(Ordering::Greater),
      ( true, false) => Some(Ordering::Less),
      ( true,  true) => Some(Ordering::Equal)
   }
}
