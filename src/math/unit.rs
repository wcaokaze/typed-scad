use crate::math::rough_fp::rough_partial_eq;
use std::iter::Sum;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Type which has a value as some unit.
///
/// e.g. [Size](crate::geometry::Size) for millimeter,
/// [Angle](crate::geometry::Angle) for radian
pub trait Unit {}

impl Unit for ! {}

/// A product of other units.
///
/// # Examples
/// ```
/// # use typed_scad::geometry::{Angle, AngleLiteral, Size, SizeLiteral};
/// # use typed_scad::math::unit::{DerivedUnit, Exp, Unit};
/// let _: DerivedUnit<Size, Angle>; // mm⋅rad
/// let _: Exp<Size, 2>; // mm²
/// let _: DerivedUnit<Size, Exp<Angle, -1>>; // mm/rad
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct DerivedUnit<
   A: Unit = Exp<!, 0>,
   B: Unit = Exp<!, 0>,
   C: Unit = Exp<!, 0>,
   D: Unit = Exp<!, 0>,
   E: Unit = Exp<!, 0>,
   F: Unit = Exp<!, 0>,
   G: Unit = Exp<!, 0>,
   H: Unit = Exp<!, 0>,
   I: Unit = Exp<!, 0>,
   J: Unit = Exp<!, 0>,
   K: Unit = Exp<!, 0>,
   L: Unit = Exp<!, 0>,
   M: Unit = Exp<!, 0>,
   N: Unit = Exp<!, 0>,
   O: Unit = Exp<!, 0>,
   P: Unit = Exp<!, 0>,
   Q: Unit = Exp<!, 0>,
   R: Unit = Exp<!, 0>,
   S: Unit = Exp<!, 0>,
   T: Unit = Exp<!, 0>,
   U: Unit = Exp<!, 0>,
   V: Unit = Exp<!, 0>,
>(
   pub f64,
   PhantomData<A>,
   PhantomData<B>,
   PhantomData<C>,
   PhantomData<D>,
   PhantomData<E>,
   PhantomData<F>,
   PhantomData<G>,
   PhantomData<H>,
   PhantomData<I>,
   PhantomData<J>,
   PhantomData<K>,
   PhantomData<L>,
   PhantomData<M>,
   PhantomData<N>,
   PhantomData<O>,
   PhantomData<P>,
   PhantomData<Q>,
   PhantomData<R>,
   PhantomData<S>,
   PhantomData<T>,
   PhantomData<U>,
   PhantomData<V>,
);

impl<
      A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,
   >
   DerivedUnit<
      A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,
   >
   where A: Unit, B: Unit, C: Unit, D: Unit, E: Unit, F: Unit, G: Unit, H: Unit,
         I: Unit, J: Unit, K: Unit, L: Unit, M: Unit, N: Unit, O: Unit, P: Unit,
         Q: Unit, R: Unit, S: Unit, T: Unit, U: Unit, V: Unit,
{
   /// create a new DerivedUnit.
   ///
   /// Be careful about type arguments.
   /// ```
   /// # use typed_scad::geometry::{Angle, Size, SizeLiteral};
   /// # use typed_scad::math::unit::DerivedUnit;
   /// let size = 42.mm();
   ///
   /// let _: DerivedUnit<Size, Angle> = unsafe {
   ///    //              -----------
   ///
   ///    DerivedUnit::new(size.to_millimeter())
   ///    //               ^^^^^^^^^^^^^^^^^^^^ THIS IS SIZE, NOT SIZE*ANGLE
   /// };
   /// ```
   pub unsafe fn new(value: f64)
      -> DerivedUnit<
         A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,
      >
   {
      DerivedUnit(
         value, PhantomData, PhantomData, PhantomData, PhantomData, PhantomData,
         PhantomData, PhantomData, PhantomData, PhantomData, PhantomData,
         PhantomData, PhantomData, PhantomData, PhantomData, PhantomData,
         PhantomData, PhantomData, PhantomData, PhantomData, PhantomData,
         PhantomData, PhantomData,
      )
   }
}

impl<
      A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,
   >
   Unit
   for DerivedUnit<
      A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V,
   >
   where A: Unit, B: Unit, C: Unit, D: Unit, E: Unit, F: Unit, G: Unit, H: Unit,
         I: Unit, J: Unit, K: Unit, L: Unit, M: Unit, N: Unit, O: Unit, P: Unit,
         Q: Unit, R: Unit, S: Unit, T: Unit, U: Unit, V: Unit,
{}

/// exponentiation of unit. e.g. `Exp<Size, 2>` for mm².
/// See also [DerivedUnit].
#[derive(Clone, Copy, Debug, Default)]
pub struct ExponentialUnit<U: Unit, const N: i32>(pub f64, PhantomData<U>);
pub type Exp<U, const N: i32> = ExponentialUnit<U, N>;

impl<U: Unit, const N: i32> ExponentialUnit<U, N> {
   /// create a new ExponentialUnit.
   ///
   /// Be careful about type arguments.
   /// ```
   /// # use typed_scad::geometry::{Size, SizeLiteral};
   /// # use typed_scad::math::unit::Exp;
   /// let size = 42.mm();
   ///
   /// let _: Exp<Size, 3> = unsafe {
   ///    //      -------
   ///
   ///    Exp::new(size.to_millimeter().powi(2))
   ///    //       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ THIS IS SIZE², NOT SIZE³
   /// };
   /// ```
   pub unsafe fn new(value: f64) -> ExponentialUnit<U, N> {
      ExponentialUnit(value, PhantomData)
   }

   pub unsafe fn operate_as<RU: Unit, const RN: i32>(self)
      -> ExponentialUnit<RU, RN>
   {
      ExponentialUnit(self.0, PhantomData)
   }
}

impl<U: Unit, const N: i32> Unit for Exp<U, N> {}

impl<U: Unit, const N: i32> PartialEq for Exp<U, N> {
   fn eq(&self, other: &Self) -> bool {
      rough_partial_eq(self.0, other.0)
   }
}

impl<U: Unit, const N: i32> Add for Exp<U, N> where U: Add {
   type Output = Exp<U, N>;
   fn add(self, rhs: Exp<U, N>) -> Exp<U, N> {
      unsafe { Exp::new(self.0 + rhs.0) }
   }
}

impl<U: Unit, const N: i32> Sub for Exp<U, N> where U: Sub {
   type Output = Exp<U, N>;
   fn sub(self, rhs: Exp<U, N>) -> Exp<U, N> {
      unsafe { Exp::new(self.0 - rhs.0) }
   }
}

impl<U: Unit, const NA: i32, const NB: i32>
   Mul<Exp<U, NB>> for Exp<U, NA>
   where Exp<U, {NA + NB}>: Sized
{
   type Output = Exp<U, {NA + NB}>;
   fn mul(self, rhs: Exp<U, NB>) -> Self::Output {
      unsafe { Exp::new(self.0 * rhs.0) }
   }
}

impl<U: Unit, const NA: i32, const NB: i32>
   Div<Exp<U, NB>> for Exp<U, NA>
   where Exp<U, {NA - NB}>: Sized
{
   type Output = Exp<U, {NA - NB}>;
   fn div(self, rhs: Exp<U, NB>) -> Self::Output {
      unsafe { Exp::new(self.0 / rhs.0) }
   }
}

impl<U: Unit, const N: i32> Neg for Exp<U, N> {
   type Output = Exp<U, N>;
   fn neg(self) -> Exp<U, N> {
      unsafe { Exp::new(-self.0) }
   }
}

impl<U: Unit, const N: i32> Sum for Exp<U, N>
   where U: Sum
{
   fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
      let sum = iter.map(|e| e.0).sum();
      unsafe { Exp::new(sum) }
   }
}

#[cfg(test)]
mod tests {
   use crate::geometry::Size;
   use super::{DerivedUnit, Exp};

   #[test]
   fn instantiate_and_get() {
      let derived_unit: DerivedUnit<Size> = unsafe { DerivedUnit::new(42.0) };
      assert_eq!(derived_unit.0, 42.0);

      let exp: Exp<Size, 2> = unsafe { Exp::new(42.0) };
      assert_eq!(exp.0, 42.0);
   }

   #[test]
   fn exp_operators() {
      let a: Exp<Size, 3> = unsafe { Exp::new(1.0) };
      let b: Exp<Size, 3> = unsafe { Exp::new(4.0) };
      let c: Exp<Size, 3> = a + b;
      assert_eq!(c.0, 5.0);

      let a: Exp<Size, 3> = unsafe { Exp::new(5.0) };
      let b: Exp<Size, 3> = unsafe { Exp::new(4.0) };
      let c: Exp<Size, 3> = a - b;
      assert_eq!(c.0, 1.0);

      let a: Exp<Size, 3> = unsafe { Exp::new(2.0) };
      let b: Exp<Size, 4> = unsafe { Exp::new(5.0) };
      let c: Exp<Size, 7> = a * b;
      assert_eq!(c.0, 10.0);

      let a: Exp<Size, 7> = unsafe { Exp::new(10.0) };
      let b: Exp<Size, 4> = unsafe { Exp::new(5.0) };
      let c: Exp<Size, 3> = a / b;
      assert_eq!(c.0, 2.0);
   }
}
