use crate::geometry::{Size, SizeLiteral};
use crate::math::rough_fp::FLOAT_POINT_ALLOWABLE_ERROR;
use std::ops::{Range, RangeFrom, RangeInclusive};
use rayon::iter::plumbing::{
   bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer
};
use rayon::prelude::{IndexedParallelIterator, ParallelIterator};

pub struct SizeIteratorBuilder<R>(pub R);
pub struct SizeParallelIteratorBuilder<R>(pub R);

fn size_count(start: Size, end: Size, step: Size) -> usize {
   if start < end {
      if step < 0.mm() {
         0
      } else {
         ((end.0 - FLOAT_POINT_ALLOWABLE_ERROR - start.0) / step.0)
            .raw() as usize + 1
      }
   } else {
      if step > 0.mm() {
         0
      } else {
         ((end.0 + FLOAT_POINT_ALLOWABLE_ERROR - start.0) / step.0)
            .raw() as usize + 1
      }
   }
}

impl SizeIteratorBuilder<Range<Size>> {
   pub fn step(self, step: Size) -> SizeIterator {
      let start = self.0.start;
      let end = self.0.end;
      let len = size_count(start, end, step);
      SizeIterator::new(start, step, len)
   }
}

impl SizeParallelIteratorBuilder<Range<Size>> {
   pub fn step(self, step: Size) -> SizeParallelIterator {
      let start = self.0.start;
      let end = self.0.end;
      let len = size_count(start, end, step);
      SizeParallelIterator { start, step, len }
   }
}

fn size_count_inclusive(start: Size, end: Size, step: Size) -> usize {
   if start < end {
      if step < 0.mm() {
         0
      } else {
         ((end.0 + FLOAT_POINT_ALLOWABLE_ERROR - start.0) / step.0)
            .raw() as usize + 1
      }
   } else {
      if step > 0.mm() {
         0
      } else {
         ((end.0 - FLOAT_POINT_ALLOWABLE_ERROR - start.0) / step.0)
            .raw() as usize + 1
      }
   }
}

impl SizeIteratorBuilder<RangeInclusive<Size>> {
   pub fn step(self, step: Size) -> SizeIterator {
      let start = *self.0.start();
      let end = *self.0.end();
      let len = size_count_inclusive(start, end, step);
      SizeIterator::new(start, step, len)
   }
}

impl SizeParallelIteratorBuilder<RangeInclusive<Size>> {
   pub fn step(self, step: Size) -> SizeParallelIterator {
      let start = *self.0.start();
      let end = *self.0.end();
      let len = size_count_inclusive(start, end, step);
      SizeParallelIterator { start, step, len }
   }
}

impl SizeIteratorBuilder<RangeFrom<Size>> {
   pub fn step(self, step: Size) -> SizeIteratorInfinite {
      let start = self.0.start;
      SizeIteratorInfinite::new(start, step)
   }
}

/// An [Iterator] for [Size].
#[derive(Clone)]
pub struct SizeIterator {
   next_left: Size,
   next_left_index: isize,
   next_right: Size,
   next_right_index: isize,
   step: Size,
   len: usize
}

#[derive(Clone)]
pub struct SizeParallelIterator {
   start: Size,
   step: Size,
   len: usize
}

/// An [Iterator] for [Size].
#[derive(Clone)]
pub struct SizeIteratorInfinite {
   next: Size,
   step: Size
}

impl SizeIterator {
   fn new(start: Size, step: Size, len: usize) -> SizeIterator {
      SizeIterator {
         next_left: start,
         next_left_index: 0,
         next_right: start + step * (len as isize - 1),
         next_right_index: len as isize - 1,
         step,
         len
      }
   }
}

impl SizeIteratorInfinite {
   fn new(start: Size, step: Size) -> SizeIteratorInfinite {
      SizeIteratorInfinite {
         next: start,
         step
      }
   }
}

impl Iterator for SizeIterator {
   type Item = Size;

   fn next(&mut self) -> Option<Size> {
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

impl Iterator for SizeIteratorInfinite {
   type Item = Size;

   fn next(&mut self) -> Option<Self::Item> {
      let next = self.next;
      self.next += self.step;
      Some(next)
   }

   fn size_hint(&self) -> (usize, Option<usize>) {
      (usize::MAX, None)
   }
}

impl ExactSizeIterator for SizeIterator {}

impl DoubleEndedIterator for SizeIterator {
   fn next_back(&mut self) -> Option<Self::Item> {
      if self.next_right_index < self.next_left_index { return None; }

      let next = self.next_right;
      self.next_right_index -= 1;
      self.next_right -= self.step;
      Some(next)
   }
}

impl ParallelIterator for SizeParallelIterator {
   type Item = Size;

   fn drive_unindexed<C>(self, consumer: C) -> C::Result
      where C: UnindexedConsumer<Self::Item>
   {
      bridge(self, consumer)
   }

   fn opt_len(&self) -> Option<usize> {
      Some(self.len)
   }
}

impl IndexedParallelIterator for SizeParallelIterator {
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
      struct SizeIterProducer {
         start: Size,
         step: Size,
         len: usize
      }

      impl Producer for SizeIterProducer {
         type Item = Size;
         type IntoIter = SizeIterator;

         fn into_iter(self) -> Self::IntoIter {
            SizeIterator::new(self.start, self.step, self.len)
         }

         fn split_at(self, index: usize) -> (Self, Self) {
            let left = SizeIterProducer {
               start: self.start,
               step: self.step,
               len: index
            };
            let right = SizeIterProducer {
               start: self.start + self.step * index,
               step: self.step,
               len: self.len - index
            };
            (left, right)
         }
      }

      callback.callback(SizeIterProducer {
         start: self.start,
         step: self.step,
         len: self.len
      })
   }
}

#[cfg(test)]
mod tests {
   use crate::geometry::{Size, SizeLiteral};
   use rayon::prelude::ParallelIterator;

   #[test]
   fn iterate() {
      let expected = vec![42.mm(), 43.5.mm(), 45.mm()];
      let actual: Vec<_> = Size::iterate(42.mm()..=45.mm()).step(1.5.mm())
         .collect();
      assert_eq!(actual, expected);

      let expected = vec![42.mm(), 43.5.mm()];
      let actual: Vec<_> = Size::iterate(42.mm()..45.mm()).step(1.5.mm())
         .collect();
      assert_eq!(actual, expected);

      let expected = vec![42.mm(), 43.5.mm(), 45.mm()];
      let actual: Vec<_> = Size::iterate(42.mm()..).step(1.5.mm())
         .take(3)
         .collect();
      assert_eq!(actual, expected);

      let actual: Vec<_> = Size::iterate(45.mm()..=42.mm()).step(1.5.mm())
         .collect();
      assert_eq!(actual, vec![]);

      let actual: Vec<_> = Size::iterate(45.mm()..42.mm()).step(1.5.mm())
         .collect();
      assert_eq!(actual, vec![]);
   }

   #[test]
   fn iterate_down() {
      let expected = vec![45.mm(), 43.5.mm(), 42.mm()];
      let actual: Vec<_> = Size::iterate(45.mm()..=42.mm()).step(-1.5.mm())
         .collect();
      assert_eq!(actual, expected);

      let expected = vec![45.mm(), 43.5.mm()];
      let actual: Vec<_> = Size::iterate(45.mm()..42.mm()).step(-1.5.mm())
         .collect();
      assert_eq!(actual, expected);

      let expected = vec![45.mm(), 43.5.mm(), 42.mm()];
      let actual: Vec<_> = Size::iterate(45.mm()..).step(-1.5.mm())
         .take(3)
         .collect();
      assert_eq!(actual, expected);

      let actual: Vec<_> = Size::iterate(42.mm()..=45.mm()).step(-1.5.mm())
         .collect();
      assert_eq!(actual, vec![]);

      let actual: Vec<_> = Size::iterate(42.mm()..45.mm()).step(-1.5.mm())
         .collect();
      assert_eq!(actual, vec![]);
   }

   #[test]
   fn size_hint() {
      let iter = Size::iterate(42.mm()..=45.mm()).step(1.5.mm());
      assert_eq!(iter.size_hint(), (3, Some(3)));

      let iter = Size::iterate(42.mm()..45.mm()).step(1.5.mm());
      assert_eq!(iter.size_hint(), (2, Some(2)));

      let iter = Size::iterate(42.mm()..=45.mm()).step(-1.5.mm());
      assert_eq!(iter.size_hint(), (0, Some(0)));

      let iter = Size::iterate(42.mm()..45.mm()).step(-1.5.mm());
      assert_eq!(iter.size_hint(), (0, Some(0)));

      let iter = Size::iterate(45.mm()..=42.mm()).step(-1.5.mm());
      assert_eq!(iter.size_hint(), (3, Some(3)));

      let iter = Size::iterate(45.mm()..42.mm()).step(-1.5.mm());
      assert_eq!(iter.size_hint(), (2, Some(2)));

      let iter = Size::iterate(45.mm()..=42.mm()).step(1.5.mm());
      assert_eq!(iter.size_hint(), (0, Some(0)));

      let iter = Size::iterate(45.mm()..42.mm()).step(1.5.mm());
      assert_eq!(iter.size_hint(), (0, Some(0)));

      let iter = Size::iterate(42.mm()..).step(1.5.mm());
      assert_eq!(iter.size_hint(), (usize::MAX, None));

      let iter = Size::iterate(45.mm()..).step(-1.5.mm());
      assert_eq!(iter.size_hint(), (usize::MAX, None));
   }

   #[test]
   fn double_ended_iter() {
      let mut iter = Size::iterate(42.mm()..=45.mm()).step(1.5.mm());
      assert_eq!(iter.next_back(), Some(45.0.mm()));
      assert_eq!(iter.next_back(), Some(43.5.mm()));
      assert_eq!(iter.next_back(), Some(42.0.mm()));
      assert_eq!(iter.next_back(), None);

      let mut iter = Size::iterate(42.mm()..45.mm()).step(1.5.mm());
      assert_eq!(iter.next_back(), Some(43.5.mm()));
      assert_eq!(iter.next_back(), Some(42.0.mm()));
      assert_eq!(iter.next_back(), None);

      let mut iter = Size::iterate(45.mm()..=42.mm()).step(-1.5.mm());
      assert_eq!(iter.next_back(), Some(42.0.mm()));
      assert_eq!(iter.next_back(), Some(43.5.mm()));
      assert_eq!(iter.next_back(), Some(45.0.mm()));
      assert_eq!(iter.next_back(), None);

      let mut iter = Size::iterate(45.mm()..42.mm()).step(-1.5.mm());
      assert_eq!(iter.next_back(), Some(43.5.mm()));
      assert_eq!(iter.next_back(), Some(45.0.mm()));
      assert_eq!(iter.next_back(), None);

      let mut iter = Size::iterate(42.mm()..=45.mm()).step(-1.5.mm());
      assert_eq!(iter.next_back(), None);

      let mut iter = Size::iterate(42.mm()..45.mm()).step(-1.5.mm());
      assert_eq!(iter.next_back(), None);

      let mut iter = Size::iterate(45.mm()..=42.mm()).step(1.5.mm());
      assert_eq!(iter.next_back(), None);

      let mut iter = Size::iterate(45.mm()..42.mm()).step(1.5.mm());
      assert_eq!(iter.next_back(), None);

      let mut iter = Size::iterate(42.mm()..=46.5.mm()).step(1.5.mm());
      assert_eq!(iter.next(),      Some(42.0.mm()));
      assert_eq!(iter.next_back(), Some(46.5.mm()));
      assert_eq!(iter.next(),      Some(43.5.mm()));
      assert_eq!(iter.next_back(), Some(45.0.mm()));
      assert_eq!(iter.next(),      None);
      assert_eq!(iter.next_back(), None);
   }

   #[test]
   fn parallel_iter() {
      let actual: Vec<_> = Size::par_iterate(0.mm()..100.mm()).step(1.mm())
         .map(|a| a * 2)
         .collect();

      let expected = (0..100).map(|i| i * 2);

      actual.into_iter().zip(expected)
         .enumerate()
         .for_each(|(i, (actual, expected))| {
            assert_eq!(actual, expected.mm(), "{i}");
         });
   }
}
