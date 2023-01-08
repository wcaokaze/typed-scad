use noisy_float::prelude::*;

pub trait ToN64 {
   fn to_n64(self) -> N64;
}

macro_rules! to_n64_as {
   ($($t:ty),+) => ($(
      impl ToN64 for $t {
         fn to_n64(self) -> N64 {
            n64(self as f64)
         }
      }
   )+)
}

to_n64_as!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32);

impl ToN64 for f64 {
   fn to_n64(self) -> N64 {
      n64(self)
   }
}

impl ToN64 for N32 {
   fn to_n64(self) -> N64 {
      N64::unchecked_new(self.raw() as f64)
   }
}

impl ToN64 for N64 {
   fn to_n64(self) -> N64 {
      self
   }
}

impl ToN64 for R32 {
   fn to_n64(self) -> N64 {
      N64::unchecked_new(self.raw() as f64)
   }
}

impl ToN64 for R64 {
   fn to_n64(self) -> N64 {
      N64::unchecked_new(self.raw())
   }
}
