
pub trait Intersection<Rhs> {
   type Output;
   fn intersection(self, rhs: Rhs) -> Self::Output;
}
