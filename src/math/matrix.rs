use std::ops::{Add, Div, Mul, Sub};
use std::mem::MaybeUninit;
use crate::math::unit::Unit;

#[derive(Debug)]
pub struct Matrix<U: Unit, const M: usize, const N: usize>([[U; N]; M]);

impl<U: Unit, const M: usize, const N: usize> Matrix<U, M, N> {
   pub fn transpose(self) -> Matrix<U, N, M> {
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

#[cfg(test)]
mod tests {
   use super::Matrix;
   use crate::geometry::{Size, SizeLiteral};

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
}
