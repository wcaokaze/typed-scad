use crate::geometry::{Size, SizeLiteral};
use std::ops::{Bound, Range, RangeBounds, RangeFrom, RangeInclusive};

/// An [Iterator] for [Size].
pub struct SizeIterator {
   next: Option<Size>,
   step: Size,
   end_bound: Bound<Size>
}

impl SizeIterator {
   fn new(
      start_value: Size, step: Size, end_bound: Bound<Size>
   ) -> SizeIterator {
      let is_iterable = Self::is_available_value(start_value, end_bound, step);

      SizeIterator {
         next: if is_iterable { Some(start_value) } else { None },
         step,
         end_bound
      }
   }

   fn calc_next(&self, current: Size) -> Option<Size> {
      let next = current + self.step;
      let has_next = Self::is_available_value(next, self.end_bound, self.step);
      if has_next { Some(next) } else { None }
   }

   fn is_available_value(
      value: Size,
      end_bound: Bound<Size>,
      step: Size
   ) -> bool {
      match end_bound {
         Bound::Included(end) =>
            if step > 0.mm() { value <= end } else { value >= end },
         Bound::Excluded(end) =>
            if step > 0.mm() { value <  end } else { value >  end },
         Bound::Unbounded => true
      }
   }
}

impl Iterator for SizeIterator {
   type Item = Size;

   fn next(&mut self) -> Option<Size> {
      let next = self.next?;
      self.next = self.calc_next(next);
      Some(next)
   }
}

/// Range that can iterate with [Size::iterate].
///
/// The follow 3 range types are iterable.
/// - [Range] `0.mm()..3.mm()`
/// - [RangeInclusive] `0.mm()..=3.mm()`
/// - [RangeFrom] `0.mm()..`
pub trait IterableSizeRange: RangeBounds<Size> {
   fn start_value(&self) -> Size;

   fn step(self, step: Size) -> SizeIterator where Self: Sized {
      SizeIterator::new(
         self.start_value(),
         step,
         self.end_bound().cloned()
      )
   }
}

impl IterableSizeRange for Range<Size> {
   fn start_value(&self) -> Size {
      self.start
   }
}

impl IterableSizeRange for RangeInclusive<Size> {
   fn start_value(&self) -> Size {
      *self.start()
   }
}

impl IterableSizeRange for RangeFrom<Size> {
   fn start_value(&self) -> Size {
      self.start
   }
}

#[cfg(test)]
mod tests {
   use super::IterableSizeRange;
   use crate::geometry::{Size, SizeLiteral};

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
}
