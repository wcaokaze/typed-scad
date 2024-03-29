use crate::math::unit::Unit;
use std::iter::Sum;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use noisy_float::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Matrix<U: Unit, const M: usize, const N: usize>(pub [[U; N]; M]);

impl<U: Unit, const M: usize, const N: usize> Matrix<U, M, N> {
   pub fn transpose(self) -> Matrix<U, N, M> {
      if M == 1 {
         let transmuter = Transmuter {
            a: ManuallyDrop::new(self.0)
         };
         return Matrix(
            unsafe { ManuallyDrop::into_inner(transmuter.b) }
         );
      }

      if N == 1 {
         let transmuter = Transmuter {
            b: ManuallyDrop::new(self.0)
         };
         return Matrix(
            unsafe { ManuallyDrop::into_inner(transmuter.a) }
         );
      }

      let mut transposed: [[MaybeUninit<U>; M]; N] = unsafe {
         MaybeUninit::uninit().assume_init()
      };

      for (m, ns) in self.0.into_iter().enumerate() {
         for (n, u) in ns.into_iter().enumerate() {
            transposed[n][m].write(u);
         }
      }

      Matrix(
         transposed.map(|m|
            m.map(|n| unsafe { n.assume_init() })
         )
      )
   }

   pub fn map<R: Unit>(self, mut f: impl FnMut(U) -> R) -> Matrix<R, M, N> {
      let f = &mut f;

      Matrix(
         self.0.map(|column| column.map(|u| f(u)))
      )
   }
}

union Transmuter<T, const M: usize, const N: usize> {
   a: ManuallyDrop<[[T; N]; M]>,
   b: ManuallyDrop<[[T; M]; N]>
}

impl<U: Unit, const M: usize, const N: usize> Default for Matrix<U, M, N>
   where U: Default
{
   fn default() -> Matrix<U, M, N> {
      Matrix(
         [(); M].map(|_|
            [(); N].map(|_| Default::default())
         )
      )
   }
}

impl<U: Unit, Rhs: Unit, const M: usize, const N: usize>
   Add<Matrix<Rhs, M, N>> for Matrix<U, M, N>
   where U: Add<Rhs>,
         U::Output: Unit
{
   type Output = Matrix<U::Output, M, N>;
   fn add(self, rhs: Matrix<Rhs, M, N>) -> Self::Output {
      let a = self.0.zip(rhs.0)
         .map(|(ma, mb)| {
            ma.zip(mb)
               .map(|(na, nb)| na + nb)
         });

      Matrix(a)
   }
}

impl<U: Unit, Rhs: Unit, const M: usize, const N: usize>
   AddAssign<Matrix<Rhs, M, N>> for Matrix<U, M, N>
   where U: AddAssign<Rhs>
{
   fn add_assign(&mut self, rhs: Matrix<Rhs, M, N>) {
      let mut x = 0;
      for r_column in rhs.0 {
         let mut y = 0;
         for r_value in r_column {
            self.0[x][y] += r_value;
            y += 1;
         }
         x += 1;
      }
   }
}

impl<U: Unit, Rhs: Unit, const M: usize, const N: usize>
   Sub<Matrix<Rhs, M, N>> for Matrix<U, M, N>
   where U: Sub<Rhs>,
         U::Output: Unit
{
   type Output = Matrix<U::Output, M, N>;
   fn sub(self, rhs: Matrix<Rhs, M, N>) -> Self::Output {
      let a = self.0.zip(rhs.0)
         .map(|(ma, mb)| {
            ma.zip(mb)
               .map(|(na, nb)| na - nb)
         });

      Matrix(a)
   }
}

impl<U: Unit, Rhs: Unit, const M: usize, const N: usize>
   SubAssign<Matrix<Rhs, M, N>> for Matrix<U, M, N>
   where U: SubAssign<Rhs>
{
   fn sub_assign(&mut self, rhs: Matrix<Rhs, M, N>) {
      let mut x = 0;
      for r_column in rhs.0 {
         let mut y = 0;
         for r_value in r_column {
            self.0[x][y] -= r_value;
            y += 1;
         }
         x += 1;
      }
   }
}

impl<U: Unit, Rhs: Unit, const M: usize, const N: usize>
   Mul<Rhs> for Matrix<U, M, N>
   where U: Mul<Rhs>,
         U::Output: Unit,
         Rhs: Copy
{
   type Output = Matrix<U::Output, M, N>;
   fn mul(self, rhs: Rhs) -> Self::Output {
      let a = self.0
         .map(|m|
            m.map(|n| n * rhs)
         );

      Matrix(a)
   }
}

macro_rules! mul_num {
   ($($t:ty),+) => ($(
      impl<U: Unit, const M: usize, const N: usize> Mul<$t> for Matrix<U, M, N>
         where U: Mul<$t>,
               U::Output: Unit
      {
         type Output = Matrix<U::Output, M, N>;
         fn mul(self, rhs: $t) -> Self::Output {
            let a = self.0
               .map(|m|
                  m.map(|n| n * rhs)
               );

            Matrix(a)
         }
      }

      impl<U: Unit, const M: usize, const N: usize> Mul<Matrix<U, M, N>> for $t
         where U: Mul<$t>,
               U::Output: Unit
      {
         type Output = Matrix<U::Output, M, N>;
         fn mul(self, rhs: Matrix<U, M, N>) -> Self::Output {
            rhs * self
         }
      }

      impl<U: Unit, const M: usize, const N: usize> MulAssign<$t> for Matrix<U, M, N>
         where U: MulAssign<$t>
      {
         fn mul_assign(&mut self, rhs: $t) {
            for column in &mut self.0 {
               for value in column {
                  *value *= rhs;
               }
            }
         }
      }
   )+)
}

mul_num!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64,
   N32, N64, R32, R64);

impl<U: Unit, const M: usize, const N: usize, Rhs> Div<Rhs> for Matrix<U, M, N>
   where U: Div<Rhs>,
         U::Output: Unit,
         Rhs: Copy
{
   type Output = Matrix<U::Output, M, N>;
   fn div(self, rhs: Rhs) -> Self::Output {
      let a = self.0
         .map(|m|
            m.map(|n| n / rhs)
         );

      Matrix(a)
   }
}

impl<U: Unit, const M: usize, const N: usize, Rhs>
   DivAssign<Rhs> for Matrix<U, M, N>
   where U: DivAssign<Rhs>,
         Rhs: Copy
{
   fn div_assign(&mut self, rhs: Rhs) {
      for column in &mut self.0 {
         for value in column {
            *value /= rhs;
         }
      }
   }
}

impl<U: Unit, const L: usize, const M: usize, const N: usize>
   Mul<Matrix<U, N, L>> for Matrix<U, L, M>
   where U: Mul<U>,
         U: Copy,
         U::Output: Unit,
         U::Output: Sum
{
   type Output = Matrix<U::Output, N, M>;
   fn mul(self, rhs: Matrix<U, N, L>) -> Self::Output {
      let a = self.transpose().0.map(|self_row|
         rhs.0.map(|rhs_column|
            self_row.iter()
               .zip(rhs_column)
               .map(|(&sm, rn)| sm * rn)
               .sum()
         )
      );

      Matrix(a).transpose()
   }
}

#[cfg(test)]
mod tests {
   use super::Matrix;
   use crate::geometry::{Size, SizeLiteral};
   use crate::math::unit::Exp;
   use noisy_float::prelude::*;

   #[test]
   fn default() {
      let a: Matrix<Size, 2, 4> = Default::default();
      let expected = Matrix([
         [0.mm(), 0.mm(), 0.mm(), 0.mm()],
         [0.mm(), 0.mm(), 0.mm(), 0.mm()]
      ]);
      assert_eq!(a, expected);
   }

   #[test]
   fn add() {
      let a = Matrix([
         [1.mm(), 2.mm(), 3.mm()],
         [4.mm(), 5.mm(), 6.mm()]
      ]);

      let b = Matrix([
         [2.mm(), 4.mm(), 6.mm()],
         [5.mm(), 6.mm(), 7.mm()]
      ]);

      let expected = Matrix([
         [3.mm(), 6.mm(), 9.mm()],
         [9.mm(), 11.mm(), 13.mm()]
      ]);

      assert_eq!(a + b, expected);

      let a = Matrix([[1.mm()]]);
      let b = Matrix([[2.mm()]]);
      assert_eq!(a + b, Matrix([[3.mm()]]));

      let a: Matrix<Size, 1, 0> = Matrix([[]]);
      let b: Matrix<Size, 1, 0> = Matrix([[]]);
      assert_eq!(a + b, Matrix([[]]));

      let a: Matrix<Size, 0, 0> = Matrix([]);
      let b: Matrix<Size, 0, 0> = Matrix([]);
      assert_eq!(a + b, Matrix([]));
   }

   #[test]
   fn sub() {
      let a = Matrix([
         [2.mm(), 4.mm(), 6.mm()],
         [5.mm(), 6.mm(), 7.mm()]
      ]);

      let b = Matrix([
         [1.mm(), 2.mm(), 3.mm()],
         [4.mm(), 5.mm(), 6.mm()]
      ]);

      let expected = Matrix([
         [1.mm(), 2.mm(), 3.mm()],
         [1.mm(), 1.mm(), 1.mm()]
      ]);

      assert_eq!(a - b, expected);

      let a = Matrix([[2.mm()]]);
      let b = Matrix([[1.mm()]]);
      assert_eq!(a - b, Matrix([[1.mm()]]));

      let a: Matrix<Size, 1, 0> = Matrix([[]]);
      let b: Matrix<Size, 1, 0> = Matrix([[]]);
      assert_eq!(a - b, Matrix([[]]));

      let a: Matrix<Size, 0, 0> = Matrix([]);
      let b: Matrix<Size, 0, 0> = Matrix([]);
      assert_eq!(a - b, Matrix([]));
   }

   #[test]
   fn mul() {
      let a = Matrix([
         [1.mm(), 2.mm(), 3.mm()],
         [4.mm(), 5.mm(), 6.mm()]
      ]);

      let expected = Matrix([
         [1.5.mm(), 3.mm(), 4.5.mm()],
         [6.mm(), 7.5.mm(), 9.mm()]
      ]);

      assert_eq!(a * 1.5, expected);

      let a = Matrix([[2.mm()]]);
      assert_eq!(a * 2, Matrix([[4.mm()]]));

      let a: Matrix<Size, 1, 0> = Matrix([[]]);
      assert_eq!(a * 2, Matrix([[]]));

      let a: Matrix<Size, 0, 0> = Matrix([]);
      assert_eq!(a * 2, Matrix([]));
   }

   #[test]
   fn div() {
      let a = Matrix([
         [3.mm(), 6.mm(), 9.mm()],
         [1.5.mm(), 4.5.mm(), 7.5.mm()]
      ]);

      let expected = Matrix([
         [2.mm(), 4.mm(), 6.mm()],
         [1.mm(), 3.mm(), 5.mm()]
      ]);

      assert_eq!(a / 1.5, expected);

      let a = Matrix([[4.mm()]]);
      assert_eq!(a / 2, Matrix([[2.mm()]]));

      let a: Matrix<Size, 1, 0> = Matrix([[]]);
      assert_eq!(a / 2, Matrix([[]]));

      let a: Matrix<Size, 0, 0> = Matrix([]);
      assert_eq!(a / 2, Matrix([]));
   }

   #[test]
   fn transpose() {
      let a = Matrix([[1.mm(), 2.mm(), 3.mm()]]);
      let b = Matrix([[1.mm()], [2.mm()], [3.mm()]]);
      assert_eq!(a.transpose(), b);

      let a = Matrix([[1.mm()], [2.mm()], [3.mm()]]);
      let b = Matrix([[1.mm(), 2.mm(), 3.mm()]]);
      assert_eq!(a.transpose(), b);

      let a = Matrix([
         [1.mm(), 2.mm(), 3.mm()],
         [4.mm(), 5.mm(), 6.mm()]
      ]);

      let expected = Matrix([
         [1.mm(), 4.mm()],
         [2.mm(), 5.mm()],
         [3.mm(), 6.mm()]
      ]);

      assert_eq!(a.transpose(), expected);

      let a = Matrix([
         [1.mm(), 2.mm(), 3.mm()],
         [4.mm(), 5.mm(), 6.mm()],
         [7.mm(), 8.mm(), 9.mm()]
      ]);

      let expected = Matrix([
         [1.mm(), 4.mm(), 7.mm()],
         [2.mm(), 5.mm(), 8.mm()],
         [3.mm(), 6.mm(), 9.mm()]
      ]);

      assert_eq!(a.transpose(), expected);
   }

   #[test]
   fn mul_matrix() {
      // NOTE: It looks like being transposed in code.
      // The type of the follow matrix is Matrix<Size, 2, 3>,
      // not Matrix<Size, 3, 2>. `[1, 2, 3]` looks like a row, but actually it's
      // a column. Since they are accessed with `matrix.0[x][y]`.
      let a = Matrix([
         [1.mm(), 2.mm(), 3.mm()],
         [4.mm(), 5.mm(), 6.mm()]
      ]);

      let b = Matrix([
         [1.mm(), 2.mm()],
         [3.mm(), 4.mm()],
         [5.mm(), 6.mm()],
         [7.mm(), 8.mm()]
      ]);

      let expected = unsafe {
         Matrix([
            [Exp::<Size, 2>::new(n64( 9.0)), Exp::<Size, 2>::new(n64(12.0)), Exp::<Size, 2>::new(n64(15.0))],
            [Exp::<Size, 2>::new(n64(19.0)), Exp::<Size, 2>::new(n64(26.0)), Exp::<Size, 2>::new(n64(33.0))],
            [Exp::<Size, 2>::new(n64(29.0)), Exp::<Size, 2>::new(n64(40.0)), Exp::<Size, 2>::new(n64(51.0))],
            [Exp::<Size, 2>::new(n64(39.0)), Exp::<Size, 2>::new(n64(54.0)), Exp::<Size, 2>::new(n64(69.0))]
         ])
      };

      assert_eq!(a * b, expected);
   }

   #[test]
   fn map() {
      let actual = Matrix([[1.mm(), 2.mm(), 3.mm()]]).map(|s| s * 2);
      let expected = Matrix([[2.mm(), 4.mm(), 6.mm()]]);
      assert_eq!(actual, expected);
   }
}
