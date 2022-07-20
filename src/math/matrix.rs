use crate::math::unit::Unit;
use std::iter::Sum;
use std::mem::{ManuallyDrop, MaybeUninit};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
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
}

union Transmuter<T, const M: usize, const N: usize> {
   a: ManuallyDrop<[[T; N]; M]>,
   b: ManuallyDrop<[[T; M]; N]>
}

impl<U: Unit, const M: usize, const N: usize> Clone for Matrix<U, M, N>
   where U: Clone
{
   fn clone(&self) -> Matrix<U, M, N> {
      Matrix(self.0.clone())
   }
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

impl<U: Unit, const M: usize, const N: usize> PartialEq for Matrix<U, M, N>
   where U: PartialEq
{
   fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
   }
}

impl<U: Unit, const M: usize, const N: usize> Eq for Matrix<U, M, N>
   where U: Eq
{
}

impl<U: Unit, const M: usize, const N: usize> Add for Matrix<U, M, N>
   where U: Add,
         U::Output: Unit
{
   type Output = Matrix<U::Output, M, N>;
   fn add(self, rhs: Self) -> Self::Output {
      let a = self.0.zip(rhs.0)
         .map(|(ma, mb)| {
            ma.zip(mb)
               .map(|(na, nb)| na + nb)
         });

      Matrix(a)
   }
}

impl<U: Unit, const M: usize, const N: usize> Sub for Matrix<U, M, N>
   where U: Sub,
         U::Output: Unit
{
   type Output = Matrix<U::Output, M, N>;
   fn sub(self, rhs: Self) -> Self::Output {
      let a = self.0.zip(rhs.0)
         .map(|(ma, mb)| {
            ma.zip(mb)
               .map(|(na, nb)| na - nb)
         });

      Matrix(a)
   }
}

impl<U: Unit, const M: usize, const N: usize, Rhs> Mul<Rhs> for Matrix<U, M, N>
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
            [Exp::<Size, 2>::new( 9.0), Exp::<Size, 2>::new(12.0), Exp::<Size, 2>::new(15.0)],
            [Exp::<Size, 2>::new(19.0), Exp::<Size, 2>::new(26.0), Exp::<Size, 2>::new(33.0)],
            [Exp::<Size, 2>::new(29.0), Exp::<Size, 2>::new(40.0), Exp::<Size, 2>::new(51.0)],
            [Exp::<Size, 2>::new(39.0), Exp::<Size, 2>::new(54.0), Exp::<Size, 2>::new(69.0)]
         ])
      };

      assert_eq!(a * b, expected);
   }
}
