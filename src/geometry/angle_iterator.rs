use crate::geometry::{Angle, AngleLiteral};
use crate::math::rough_fp::FLOAT_POINT_ALLOWABLE_ERROR;
use std::ops::{Range, RangeFrom, RangeInclusive};
use rayon::iter::plumbing::{
   bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer
};
use rayon::prelude::{IndexedParallelIterator, ParallelIterator};

pub struct AngleIteratorBuilder<R>(pub R);
pub struct AngleParallelIteratorBuilder<R>(pub R);

fn angle_count(start: Angle, end: Angle, step: Angle) -> usize {
   if start < end {
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
   }
}

impl AngleIteratorBuilder<Range<Angle>> {
   pub fn step(self, step: Angle) -> AngleIterator {
      let start = self.0.start;
      let end = self.0.end;
      let len = angle_count(start, end, step);
      AngleIterator::new(start, step, len)
   }
}

impl AngleParallelIteratorBuilder<Range<Angle>> {
   pub fn step(self, step: Angle) -> AngleParallelIterator {
      let start = self.0.start;
      let end = self.0.end;
      let len = angle_count(start, end, step);
      AngleParallelIterator { start, step, len }
   }
}

fn angle_count_inclusive(start: Angle, end: Angle, step: Angle) -> usize {
   if start < end {
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
   }
}

impl AngleIteratorBuilder<RangeInclusive<Angle>> {
   pub fn step(self, step: Angle) -> AngleIterator {
      let start = *self.0.start();
      let end = *self.0.end();
      let len = angle_count_inclusive(start, end, step);
      AngleIterator::new(start, step, len)
   }
}

impl AngleParallelIteratorBuilder<RangeInclusive<Angle>> {
   pub fn step(self, step: Angle) -> AngleParallelIterator {
      let start = *self.0.start();
      let end = *self.0.end();
      let len = angle_count_inclusive(start, end, step);
      AngleParallelIterator { start, step, len }
   }
}

impl AngleIteratorBuilder<RangeFrom<Angle>> {
   pub fn step(self, step: Angle) -> AngleIteratorInfinite {
      let start = self.0.start;
      AngleIteratorInfinite::new(start, step)
   }
}

/// An [Iterator] for [Angle].
#[derive(Clone)]
pub struct AngleIterator {
   next_left: Angle,
   next_left_index: isize,
   next_right: Angle,
   next_right_index: isize,
   step: Angle,
   len: usize
}

#[derive(Clone)]
pub struct AngleParallelIterator {
   start: Angle,
   step: Angle,
   len: usize
}

/// An [Iterator] for [Angle].
#[derive(Clone)]
pub struct AngleIteratorInfinite {
   next: Angle,
   step: Angle
}

impl AngleIterator {
   fn new(start: Angle, step: Angle, len: usize) -> AngleIterator {
      AngleIterator {
         next_left: start,
         next_left_index: 0,
         next_right: start + step * (len as isize - 1),
         next_right_index: len as isize - 1,
         step,
         len
      }
   }
}

impl AngleIteratorInfinite {
   fn new(start: Angle, step: Angle) -> AngleIteratorInfinite {
      AngleIteratorInfinite {
         next: start,
         step
      }
   }
}

impl Iterator for AngleIterator {
   type Item = Angle;

   fn next(&mut self) -> Option<Angle> {
      if self.next_left_index > self.next_right_index { return None; }

      let next = self.next_left;
      self.next_left_index += 1;
      self.next_left += self.step;
      Some(next)
   }

   fn size_hint(&self) -> (usize, Option<usize>) {
      let remain_size = self.len - self.next_left_index as usize;
      (remain_size, Some(remain_size))
   }
}

impl Iterator for AngleIteratorInfinite {
   type Item = Angle;

   fn next(&mut self) -> Option<Angle> {
      let next = self.next;
      self.next += self.step;
      Some(next)
   }

   fn size_hint(&self) -> (usize, Option<usize>) {
      (usize::MAX, None)
   }
}

impl ExactSizeIterator for AngleIterator {}

impl DoubleEndedIterator for AngleIterator {
   fn next_back(&mut self) -> Option<Self::Item> {
      if self.next_right_index < self.next_left_index { return None; }

      let next = self.next_right;
      self.next_right_index -= 1;
      self.next_right -= self.step;
      Some(next)
   }
}

impl ParallelIterator for AngleParallelIterator {
   type Item = Angle;

   fn drive_unindexed<C>(self, consumer: C) -> C::Result
      where C: UnindexedConsumer<Self::Item>
   {
      bridge(self, consumer)
   }

   fn opt_len(&self) -> Option<usize> {
      Some(self.len)
   }
}

impl IndexedParallelIterator for AngleParallelIterator {
   fn len(&self) -> usize {
      self.len
   }

   fn drive<C>(self, consumer: C) -> C::Result
      where C: Consumer<Self::Item>
   {
      bridge(self, consumer)
   }

   fn with_producer<CB>(self, callback: CB) -> CB::Output
      where CB: ProducerCallback<Self::Item>
   {
      struct AngleIterProducer {
         start: Angle,
         step: Angle,
         len: usize
      }
      
      impl Producer for AngleIterProducer {
         type Item = Angle;
         type IntoIter = AngleIterator;

         fn into_iter(self) -> Self::IntoIter {
            AngleIterator::new(self.start, self.step, self.len)
         }

         fn split_at(self, index: usize) -> (Self, Self) {
            let left = AngleIterProducer {
               start: self.start,
               step: self.step,
               len: index
            };
            let right = AngleIterProducer {
               start: self.start + self.step * index,
               step: self.step,
               len: self.len - index
            };
            (left, right)
         }
      }

      callback.callback(AngleIterProducer {
         start: self.start,
         step: self.step,
         len: self.len
      })
   }
}

#[cfg(test)]
mod tests {
   use crate::geometry::{Angle, AngleLiteral};
   use rayon::prelude::ParallelIterator;

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

   #[test]
   fn size_hint() {
      let iter = Angle::iterate(42.deg()..=45.deg()).step(1.5.deg());
      assert_eq!(iter.size_hint(), (3, Some(3)));

      let iter = Angle::iterate(42.deg()..45.deg()).step(1.5.deg());
      assert_eq!(iter.size_hint(), (2, Some(2)));

      let iter = Angle::iterate(42.deg()..=45.deg()).step(-1.5.deg());
      assert_eq!(iter.size_hint(), (0, Some(0)));

      let iter = Angle::iterate(42.deg()..45.deg()).step(-1.5.deg());
      assert_eq!(iter.size_hint(), (0, Some(0)));

      let iter = Angle::iterate(45.deg()..=42.deg()).step(-1.5.deg());
      assert_eq!(iter.size_hint(), (3, Some(3)));

      let iter = Angle::iterate(45.deg()..42.deg()).step(-1.5.deg());
      assert_eq!(iter.size_hint(), (2, Some(2)));

      let iter = Angle::iterate(45.deg()..=42.deg()).step(1.5.deg());
      assert_eq!(iter.size_hint(), (0, Some(0)));

      let iter = Angle::iterate(45.deg()..42.deg()).step(1.5.deg());
      assert_eq!(iter.size_hint(), (0, Some(0)));

      let iter = Angle::iterate(42.deg()..).step(1.5.deg());
      assert_eq!(iter.size_hint(), (usize::MAX, None));

      let iter = Angle::iterate(45.deg()..).step(-1.5.deg());
      assert_eq!(iter.size_hint(), (usize::MAX, None));
   }

   #[test]
   fn double_ended_iter() {
      let mut iter = Angle::iterate(42.deg()..=45.deg()).step(1.5.deg());
      assert_eq!(iter.next_back(), Some(45.0.deg()));
      assert_eq!(iter.next_back(), Some(43.5.deg()));
      assert_eq!(iter.next_back(), Some(42.0.deg()));
      assert_eq!(iter.next_back(), None);

      let mut iter = Angle::iterate(42.deg()..45.deg()).step(1.5.deg());
      assert_eq!(iter.next_back(), Some(43.5.deg()));
      assert_eq!(iter.next_back(), Some(42.0.deg()));
      assert_eq!(iter.next_back(), None);

      let mut iter = Angle::iterate(45.deg()..=42.deg()).step(-1.5.deg());
      assert_eq!(iter.next_back(), Some(42.0.deg()));
      assert_eq!(iter.next_back(), Some(43.5.deg()));
      assert_eq!(iter.next_back(), Some(45.0.deg()));
      assert_eq!(iter.next_back(), None);

      let mut iter = Angle::iterate(45.deg()..42.deg()).step(-1.5.deg());
      assert_eq!(iter.next_back(), Some(43.5.deg()));
      assert_eq!(iter.next_back(), Some(45.0.deg()));
      assert_eq!(iter.next_back(), None);

      let mut iter = Angle::iterate(42.deg()..=45.deg()).step(-1.5.deg());
      assert_eq!(iter.next_back(), None);

      let mut iter = Angle::iterate(42.deg()..45.deg()).step(-1.5.deg());
      assert_eq!(iter.next_back(), None);

      let mut iter = Angle::iterate(45.deg()..=42.deg()).step(1.5.deg());
      assert_eq!(iter.next_back(), None);

      let mut iter = Angle::iterate(45.deg()..42.deg()).step(1.5.deg());
      assert_eq!(iter.next_back(), None);

      let mut iter = Angle::iterate(42.deg()..=46.5.deg()).step(1.5.deg());
      assert_eq!(iter.next(),      Some(42.0.deg()));
      assert_eq!(iter.next_back(), Some(46.5.deg()));
      assert_eq!(iter.next(),      Some(43.5.deg()));
      assert_eq!(iter.next_back(), Some(45.0.deg()));
      assert_eq!(iter.next(),      None);
      assert_eq!(iter.next_back(), None);
   }

   #[test]
   fn parallel_iter() {
      let actual: Vec<_> = Angle::par_iterate(0.deg()..100.deg()).step(1.deg())
         .map(|a| a * 2)
         .collect();

      let expected = (0..100).map(|i| i * 2);

      actual.into_iter().zip(expected)
         .enumerate()
         .for_each(|(i, (actual, expected))| {
            assert_eq!(actual, expected.deg(), "{i}");
         });
   }
}
