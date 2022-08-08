use crate::geometry::{Angle, AngleLiteral};
use std::ops::{Range, RangeFrom, RangeInclusive};
use crate::math::rough_fp::FLOAT_POINT_ALLOWABLE_ERROR;

pub struct AngleIteratorBuilder<R>(pub R);

impl AngleIteratorBuilder<Range<Angle>> {
   pub fn step(self, step: Angle) -> AngleIterator {
      let start = self.0.start;
      let end = self.0.end;

      let len = if start < end {
         if step < 0.rad() {
            0
         } else {
            ((end.0 - FLOAT_POINT_ALLOWABLE_ERROR - start.0) / step.0)
               as usize + 1
         }
      } else {
         if step > 0.rad() {
            0
         } else {
            ((end.0 + FLOAT_POINT_ALLOWABLE_ERROR - start.0) / step.0)
               as usize + 1
         }
      };

      AngleIterator::new(start, step, len as usize)
   }
}

impl AngleIteratorBuilder<RangeInclusive<Angle>> {
   pub fn step(self, step: Angle) -> AngleIterator {
      let start = *self.0.start();
      let end = *self.0.end();

      let len = if start < end {
         if step < 0.rad() {
            0
         } else {
            ((end.0 + FLOAT_POINT_ALLOWABLE_ERROR - start.0) / step.0)
               as usize + 1
         }
      } else {
         if step > 0.rad() {
            0
         } else {
            ((end.0 - FLOAT_POINT_ALLOWABLE_ERROR - start.0) / step.0)
               as usize + 1
         }
      };

      AngleIterator::new(start, step, len)
   }
}

impl AngleIteratorBuilder<RangeFrom<Angle>> {
   pub fn step(self, step: Angle) -> AngleIteratorInfinite {
      let start = self.0.start;
      AngleIteratorInfinite::new(start, step)
   }
}

/// An [Iterator] for [Angle].
pub struct AngleIterator {
   next: Angle,
   next_index: usize,
   step: Angle,
   len: usize
}

/// An [Iterator] for [Angle].
pub struct AngleIteratorInfinite {
   next: Angle,
   step: Angle
}

impl AngleIterator {
   fn new(start: Angle, step: Angle, len: usize) -> AngleIterator {
      AngleIterator {
         next: start,
         next_index: 1,
         step,
         len
      }
   }
}

impl AngleIteratorInfinite {
   fn new(start: Angle, step: Angle) -> AngleIteratorInfinite {
      AngleIteratorInfinite {
         next: start,
         step,
      }
   }
}

impl Iterator for AngleIterator {
   type Item = Angle;

   fn next(&mut self) -> Option<Angle> {
      let next = self.next;
      if self.next_index > self.len { return None; }

      self.next_index += 1;
      self.next += self.step;
      Some(next)
   }
}

impl Iterator for AngleIteratorInfinite {
   type Item = Angle;

   fn next(&mut self) -> Option<Angle> {
      let next = self.next;
      self.next += self.step;
      Some(next)
   }
}

#[cfg(test)]
mod tests {
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
