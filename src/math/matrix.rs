use std::ops::{Add, Sub};
use crate::math::unit::Unit;

#[derive(Debug)]
pub struct Matrix<U: Unit, const X: usize, const Y: usize>([[U; X]; Y]);

impl<U: Unit, const X: usize, const Y: usize> Clone for Matrix<U, X, Y>
   where U: Clone
{
   fn clone(&self) -> Matrix<U, X, Y> {
      Matrix(self.0.clone())
   }
}

impl<U: Unit, const X: usize, const Y: usize> Default for Matrix<U, X, Y>
   where U: Default
{
   fn default() -> Matrix<U, X, Y> {
      Matrix(
         [(); Y].map(|_|
            [(); X].map(|_| Default::default())
         )
      )
   }
}

impl<U: Unit, const X: usize, const Y: usize> PartialEq for Matrix<U, X, Y>
   where U: PartialEq
{
   fn eq(&self, other: &Self) -> bool {
      self.0 == other.0
   }
}

impl<U: Unit, const X: usize, const Y: usize> Eq for Matrix<U, X, Y>
   where U: Eq
{
}

impl<U: Unit, const X: usize, const Y: usize> Add for Matrix<U, X, Y>
   where U: Add,
         U::Output: Unit
{
   type Output = Matrix<U::Output, X, Y>;
   fn add(self, rhs: Self) -> Self::Output {
      let a = self.0.zip(rhs.0)
         .map(|(ya, yb)| {
            ya.zip(yb)
               .map(|(xa, xb)| xa + xb)
         });

      Matrix(a)
   }
}

impl<U: Unit, const X: usize, const Y: usize> Sub for Matrix<U, X, Y>
   where U: Sub,
         U::Output: Unit
{
   type Output = Matrix<U::Output, X, Y>;
   fn sub(self, rhs: Self) -> Self::Output {
      let a = self.0.zip(rhs.0)
         .map(|(ya, yb)| {
            ya.zip(yb)
               .map(|(xa, xb)| xa - xb)
         });

      Matrix(a)
   }
}

#[cfg(test)]
mod tests {
   use super::Matrix;
   use crate::geometry::{Size, SizeLiteral};

   #[test]
   fn default() {
      let a: Matrix<Size, 4, 2> = Default::default();
      let expected = Matrix([
         [0.mm(), 0.mm(), 0.mm(), 0.mm()],
         [0.mm(), 0.mm(), 0.mm(), 0.mm()],
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

      let a: Matrix<Size, 0, 1> = Matrix([[]]);
      let b: Matrix<Size, 0, 1> = Matrix([[]]);
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

      let a: Matrix<Size, 0, 1> = Matrix([[]]);
      let b: Matrix<Size, 0, 1> = Matrix([[]]);
      assert_eq!(a - b, Matrix([[]]));

      let a: Matrix<Size, 0, 0> = Matrix([]);
      let b: Matrix<Size, 0, 0> = Matrix([]);
      assert_eq!(a - b, Matrix([]));
   }
}
