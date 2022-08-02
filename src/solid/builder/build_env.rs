use std::any::Any;
use std::cell::{RefCell, UnsafeCell};
use std::collections::HashMap;
use std::mem;
use std::ops::Deref;

thread_local! {
   static NEXT_ID: RefCell<u32> = RefCell::new(0);

   static ENV_MAP: UnsafeCell<HashMap<u32, RefCell<Box<dyn Any>>>>
      = UnsafeCell::new(HashMap::new());
}

pub fn env<T: 'static>(
   env: &BuildEnv<T>,
   value: T,
   build_action: impl FnOnce() -> ()
) {
   let old_value = mem::replace(env.get_mut(), value);
   build_action();
   *env.get_mut() = old_value;
}

pub struct BuildEnv<T: 'static> {
   id: u32,
   default: Box<dyn Fn() -> T>
}

impl<T: 'static> BuildEnv<T> {
   pub fn new(default: impl Fn() -> T + 'static) -> BuildEnv<T> {
      BuildEnv {
         id: NEXT_ID.with(|cell|
            cell.replace_with(|i| *i + 1)
         ),
         default: Box::new(default)
      }
   }

   fn get_mut(&self) -> &mut T {
      ENV_MAP.with(|m| {
         let map = unsafe { &mut *m.get() };
         let cell = map.entry(self.id).or_insert_with(|| {
            let default = (self.default)();
            RefCell::new(Box::new(default))
         });

         let r = cell.borrow_mut().downcast_mut().unwrap() as *mut _;

         // borrow as longer lifetime.
         // This is safe since any entry in ENV_MAP is never removed.
         unsafe { &mut *r }
      })
   }
}

impl<T: 'static> Deref for BuildEnv<T> {
   type Target = T;
   fn deref(&self) -> &T {
      self.get_mut()
   }
}

#[cfg(test)]
mod tests {
   use super::{BuildEnv, env};

   #[test]
   fn id() {
      let a = BuildEnv::<()>::new(|| ());
      let b = BuildEnv::<()>::new(|| ());
      let c = BuildEnv::<()>::new(|| ());

      assert_eq!(a.id, 0);
      assert_eq!(b.id, 1);
      assert_eq!(c.id, 2);
   }

   #[test]
   fn set_env() {
      let a = BuildEnv::<i32>::new(|| 0);
      let b = BuildEnv::<i32>::new(|| 42);
      let c = BuildEnv::<String>::new(|| "wcaokaze".to_string());

      assert_eq!(*a, 0);
      assert_eq!(*b, 42);
      assert_eq!(&*c, "wcaokaze");

      env(&a, 1, || {
         assert_eq!(*a, 1);
         assert_eq!(*b, 42);
         assert_eq!(&*c, "wcaokaze");

         env(&b, 2, || {
            assert_eq!(*a, 1);
            assert_eq!(*b, 2);
            assert_eq!(&*c, "wcaokaze");
         });

         env(&c, "a".to_string(), || {
            assert_eq!(*a, 1);
            assert_eq!(*b, 42);
            assert_eq!(&*c, "a");
         });
      });

      assert_eq!(*a, 0);
      assert_eq!(*b, 42);
      assert_eq!(&*c, "wcaokaze");
   }
}
