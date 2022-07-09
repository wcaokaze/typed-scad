use crate::geometry::{Angle, AngleLiteral};
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeInclusive};

/// An [Iterator] for [Angle].
pub struct AngleIterator {
   next: Option<Angle>,
   step: Angle,
   end_bound: Bound<Angle>
}

impl AngleIterator {
   fn new(
      start_value: Angle, step: Angle, end_bound: Bound<Angle>
   ) -> AngleIterator {
      let is_iterable = Self::is_available_value(start_value, end_bound, step);

      AngleIterator {
         next: if is_iterable { Some(start_value) } else { None },
         step,
         end_bound
      }
   }

   fn calc_next(&self, current: Angle) -> Option<Angle> {
      let next = current + self.step;
      let has_next = Self::is_available_value(next, self.end_bound, self.step);
      if has_next { Some(next) } else { None }
   }

   fn is_available_value(
      value: Angle,
      end_bound: Bound<Angle>,
      step: Angle
   ) -> bool {
      match end_bound {
         Bound::Included(end) =>
            if step > 0.rad() { value <= end } else { value >= end },
         Bound::Excluded(end) =>
            if step > 0.rad() { value <  end } else { value >  end },
         Bound::Unbounded => true
      }
   }
}

impl Iterator for AngleIterator {
   type Item = Angle;

   fn next(&mut self) -> Option<Angle> {
      let next = self.next?;
      self.next = self.calc_next(next);
      Some(next)
   }
}

/// Range that can iterate with [Angle::iterate].
///
/// The follow 3 range types are iterable.
/// - [Range] `0.deg()..3.deg()`
/// - [RangeInclusive] `0.deg()..=3.deg()`
/// - [RangeFrom] `0.deg()..`
pub trait IterableAngleRange: RangeBounds<Angle> {
   fn start_value(&self) -> Angle;

   fn step(self, step: Angle) -> AngleIterator where Self: Sized {
      AngleIterator::new(
         self.start_value(),
         step,
         self.end_bound().cloned()
      )
   }
}

impl IterableAngleRange for Range<Angle> {
   fn start_value(&self) -> Angle {
      self.start
   }
}

impl IterableAngleRange for RangeInclusive<Angle> {
   fn start_value(&self) -> Angle {
      *self.start()
   }
}

impl IterableAngleRange for RangeFrom<Angle> {
   fn start_value(&self) -> Angle {
      self.start
   }
}

#[cfg(test)]
mod tests {
   use super::IterableAngleRange;
   use crate::geometry::{Angle, AngleLiteral};

   #[test]
   fn iterate() {
      let expected = vec![42.deg(), 43.5.deg(), 45.deg()];
      let actual: Vec<_> = Angle::iterate(42.deg()..=45.deg()).step(1.5.deg())
         .collect();
      assert_eq!(actual, expected);

      let expected = vec![42.deg(), 43.5.deg()];
      let actual: Vec<_> = Angle::iterate(42.deg()..45.deg()).step(1.5.deg())
         .collect();
      assert_eq!(actual, expected);

      let expected = vec![42.deg(), 43.5.deg(), 45.deg()];
      let actual: Vec<_> = Angle::iterate(42.deg()..).step(1.5.deg())
         .take(3)
         .collect();
      assert_eq!(actual, expected);

      let actual: Vec<_> = Angle::iterate(45.deg()..=42.deg()).step(1.5.deg())
         .collect();
      assert_eq!(actual, vec![]);

      let actual: Vec<_> = Angle::iterate(45.deg()..42.deg()).step(1.5.deg())
         .collect();
      assert_eq!(actual, vec![]);
   }

   #[test]
   fn iterate_down() {
      let expected = vec![45.deg(), 43.5.deg(), 42.deg()];
      let actual: Vec<_> = Angle::iterate(45.deg()..=42.deg()).step(-1.5.deg())
         .collect();
      assert_eq!(actual, expected);

      let expected = vec![45.deg(), 43.5.deg()];
      let actual: Vec<_> = Angle::iterate(45.deg()..42.deg()).step(-1.5.deg())
         .collect();
      assert_eq!(actual, expected);

      let expected = vec![45.deg(), 43.5.deg(), 42.deg()];
      let actual: Vec<_> = Angle::iterate(45.deg()..).step(-1.5.deg())
         .take(3)
         .collect();
      assert_eq!(actual, expected);

      let actual: Vec<_> = Angle::iterate(42.deg()..=45.deg()).step(-1.5.deg())
         .collect();
      assert_eq!(actual, vec![]);

      let actual: Vec<_> = Angle::iterate(42.deg()..45.deg()).step(-1.5.deg())
         .collect();
      assert_eq!(actual, vec![]);
   }
}
