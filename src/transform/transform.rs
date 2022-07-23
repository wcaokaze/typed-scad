use crate::geometry::{Angle, Line, Size, Vector};

pub trait Transform: Sized {
   fn translated(&self, offset: &Vector) -> Self;

   fn translate(&mut self, offset: &Vector) {
      *self = self.translated(offset);
   }

   fn translated_toward(&self, direction: &Vector, distance: Size) -> Self {
      let v = *direction * (distance / direction.norm());
      self.translated(&v)
   }

   fn translate_toward(&mut self, direction: &Vector, distance: Size) {
      *self = self.translated_toward(direction, distance);
   }

   fn rotated(&self, axis: &Line, angle: Angle) -> Self;

   fn rotate(&mut self, axis: &Line, angle: Angle) {
      *self = self.rotated(axis, angle);
   }
}
