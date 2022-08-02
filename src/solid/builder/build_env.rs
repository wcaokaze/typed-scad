use std::cell::RefCell;
use std::marker::PhantomData;

thread_local! {
   static NEXT_ID: RefCell<u32> = RefCell::new(0);
}

pub struct BuildEnv<T> {
   id: u32,
   _phantom: PhantomData<T>
}

impl<T> BuildEnv<T> {
   pub fn new() -> BuildEnv<T> {
      BuildEnv {
         id: NEXT_ID.with(|cell|
            cell.replace_with(|i| *i + 1)
         ),
         _phantom: PhantomData
      }
   }
}

#[cfg(test)]
mod tests {
   use crate::solid::builder::BuildEnv;

   #[test]
   fn id() {
      let a = BuildEnv::<()>::new();
      let b = BuildEnv::<()>::new();
      let c = BuildEnv::<()>::new();

      assert_eq!(a.id, 0);
      assert_eq!(b.id, 1);
      assert_eq!(c.id, 2);
   }
}
