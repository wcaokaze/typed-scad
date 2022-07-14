use crate::geometry::{Angle, Line, Size, Vector};

pub trait Transform {
   fn translate(&mut self, offset: &Vector);

   fn translate_toward(&mut self, direction: &Vector, distance: Size) {
      let v = *direction * (distance / direction.norm());
      self.translate(&v);
   }

   fn rotate(&mut self, axis: &Line, angle: Angle);
}
