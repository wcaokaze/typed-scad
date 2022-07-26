use crate::geometry::{Point, Vector};
use crate::solid::Location;

/// See [Location].
pub struct LocationBuilder<const X: bool, const Y: bool, const Z: bool>
   where [Vector; X as usize]: Sized,
         [Vector; Y as usize]: Sized,
         [Vector; Z as usize]: Sized
{
   point: Point,
   right_vector: [Vector; X as usize],
   back_vector: [Vector; Y as usize],
   top_vector: [Vector; Z as usize]
}

impl LocationBuilder<false, false, false> {
   pub fn new(point: Point) -> LocationBuilder<false, false, false> {
      LocationBuilder {
         point,
         right_vector: [],
         back_vector: [],
         top_vector: []
      }
   }

   pub fn right_vector(self, right_vector: Vector)
      -> LocationBuilder<true, false, false>
   {
      LocationBuilder {
         point: self.point,
         right_vector: [right_vector],
         back_vector: self.back_vector,
         top_vector: self.top_vector
      }
   }

   pub fn left_vector(self, left_vector: Vector)
      -> LocationBuilder<true, false, false>
   {
      self.right_vector(-left_vector)
   }

   pub fn back_vector(self, back_vector: Vector)
      -> LocationBuilder<false, true, false>
   {
      LocationBuilder {
         point: self.point,
         right_vector: self.right_vector,
         back_vector: [back_vector],
         top_vector: self.top_vector
      }
   }

   pub fn front_vector(self, front_vector: Vector)
      -> LocationBuilder<false, true, false>
   {
      self.back_vector(-front_vector)
   }

   pub fn top_vector(self, top_vector: Vector)
      -> LocationBuilder<false, false, true>
   {
      LocationBuilder {
         point: self.point,
         right_vector: self.right_vector,
         back_vector: self.back_vector,
         top_vector: [top_vector]
      }
   }

   pub fn bottom_vector(self, bottom_vector: Vector)
      -> LocationBuilder<false, false, true>
   {
      self.top_vector(-bottom_vector)
   }
}

impl LocationBuilder<true, false, false> {
   pub fn back_vector(self, back_vector: Vector) -> Location {
      let right_vector = self.right_vector[0];
      Location::new(self.point, right_vector, back_vector)
   }

   pub fn front_vector(self, front_vector: Vector) -> Location {
      self.back_vector(-front_vector)
   }

   pub fn top_vector(self, top_vector: Vector) -> Location {
      let right_vector = self.right_vector[0];
      let back_vector = top_vector.vector_product(&right_vector);
      Location::new(self.point, right_vector, back_vector)
   }

   pub fn bottom_vector(self, bottom_vector: Vector) -> Location {
      self.top_vector(-bottom_vector)
   }
}

impl LocationBuilder<false, true, false> {
   pub fn right_vector(self, right_vector: Vector) -> Location {
      let back_vector = self.back_vector[0];
      Location::new(self.point, right_vector, back_vector)
   }

   pub fn left_vector(self, left_vector: Vector) -> Location {
      self.right_vector(-left_vector)
   }

   pub fn top_vector(self, top_vector: Vector) -> Location {
      let back_vector = self.back_vector[0];
      let right_vector = back_vector.vector_product(&top_vector);
      Location::new(self.point, right_vector, back_vector)
   }

   pub fn bottom_vector(self, bottom_vector: Vector) -> Location {
      self.top_vector(-bottom_vector)
   }
}

impl LocationBuilder<false, false, true> {
   pub fn right_vector(self, right_vector: Vector) -> Location {
      let top_vector = self.top_vector[0];
      let back_vector = top_vector.vector_product(&right_vector);
      Location::new(self.point, right_vector, back_vector)
   }

   pub fn left_vector(self, left_vector: Vector) -> Location {
      self.right_vector(-left_vector)
   }

   pub fn back_vector(self, back_vector: Vector) -> Location {
      let top_vector = self.top_vector[0];
      let right_vector = back_vector.vector_product(&top_vector);
      Location::new(self.point, right_vector, back_vector)
   }

   pub fn front_vector(self, front_vector: Vector) -> Location {
      self.back_vector(-front_vector)
   }
}

#[cfg(test)]
mod tests {
   use crate::geometry::{Point, SizeLiteral, Vector};
   use crate::solid::Location;

   #[test]
   fn build() {
      let point = Point::new(1.mm(), 2.mm(), 3.mm());
      let left_vector = Vector::new(-1.mm(), -1.mm(), 0.mm());
      let right_vector = Vector::new(1.mm(), 1.mm(), 0.mm());
      let front_vector = Vector::new(1.mm(), -1.mm(), 0.mm());
      let back_vector = Vector::new(-1.mm(), 1.mm(), 0.mm());
      let bottom_vector = Vector::new(0.mm(), 0.mm(), -1.mm());
      let top_vector = Vector::new(0.mm(), 0.mm(), 1.mm());

      let expected = Location::new(point, right_vector, back_vector);

      assert_eq!(Location::build(point).left_vector (left_vector) .front_vector (front_vector),  expected);
      assert_eq!(Location::build(point).right_vector(right_vector).front_vector (front_vector),  expected);
      assert_eq!(Location::build(point).left_vector (left_vector) .back_vector  (back_vector),   expected);
      assert_eq!(Location::build(point).right_vector(right_vector).back_vector  (back_vector),   expected);
      assert_eq!(Location::build(point).left_vector (left_vector) .bottom_vector(bottom_vector), expected);
      assert_eq!(Location::build(point).right_vector(right_vector).bottom_vector(bottom_vector), expected);
      assert_eq!(Location::build(point).left_vector (left_vector) .top_vector   (top_vector),    expected);
      assert_eq!(Location::build(point).right_vector(right_vector).top_vector   (top_vector),    expected);

      assert_eq!(Location::build(point).front_vector(front_vector).left_vector  (left_vector),   expected);
      assert_eq!(Location::build(point).back_vector (back_vector) .left_vector  (left_vector),   expected);
      assert_eq!(Location::build(point).front_vector(front_vector).right_vector (right_vector),  expected);
      assert_eq!(Location::build(point).back_vector (back_vector) .right_vector (right_vector),  expected);
      assert_eq!(Location::build(point).front_vector(front_vector).bottom_vector(bottom_vector), expected);
      assert_eq!(Location::build(point).back_vector (back_vector) .bottom_vector(bottom_vector), expected);
      assert_eq!(Location::build(point).front_vector(front_vector).top_vector   (top_vector),    expected);
      assert_eq!(Location::build(point).back_vector (back_vector) .top_vector   (top_vector),    expected);

      assert_eq!(Location::build(point).bottom_vector(bottom_vector).left_vector (left_vector),  expected);
      assert_eq!(Location::build(point).top_vector   (top_vector)   .left_vector (left_vector),  expected);
      assert_eq!(Location::build(point).bottom_vector(bottom_vector).right_vector(right_vector), expected);
      assert_eq!(Location::build(point).top_vector   (top_vector)   .right_vector(right_vector), expected);
      assert_eq!(Location::build(point).bottom_vector(bottom_vector).front_vector(front_vector), expected);
      assert_eq!(Location::build(point).top_vector   (top_vector)   .front_vector(front_vector), expected);
      assert_eq!(Location::build(point).bottom_vector(bottom_vector).back_vector (back_vector),  expected);
      assert_eq!(Location::build(point).top_vector   (top_vector)   .back_vector (back_vector),  expected);
   }
}
