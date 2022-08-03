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

pub fn env<T: 'static, D: Fn() -> T>(
   env: &BuildEnv<T, D>,
   value: T,
   build_action: impl FnOnce() -> ()
) {
   let cell_inner_mut = env.cell_inner_mut();
   let old_value = mem::replace(cell_inner_mut, Box::new(value));
   build_action();
   *cell_inner_mut = old_value;
}

pub struct BuildEnv<T: 'static, D: Fn() -> T = fn() -> T> {
   id: u32,
   default: D
}

impl<T: 'static, D: Fn() -> T> BuildEnv<T, D> {
   pub fn new(default: D) -> BuildEnv<T, D> {
      BuildEnv {
         id: NEXT_ID.with(|cell|
            cell.replace_with(|i| *i + 1)
         ),
         default
      }
   }

   fn cell_inner_mut(&self) -> &mut Box<dyn Any> {
      ENV_MAP.with(|m| {
         let map = unsafe { &mut *m.get() };
         let cell = map.entry(self.id).or_insert_with(|| {
            let default = (self.default)();
            RefCell::new(Box::new(default))
         });

         let r: &mut Box<_> = &mut *cell.borrow_mut();

         // borrow as longer lifetime.
         // This is safe since any RefCell in ENV_MAP is never removed.
         let r = r as *mut _;
         unsafe { &mut *r }
      })
   }
}

impl<T: 'static, D: Fn() -> T> Deref for BuildEnv<T, D> {
   type Target = T;
   fn deref(&self) -> &T {
      let r = self.cell_inner_mut();
      r.downcast_ref().unwrap()
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

   #[test]
   fn outer_scope_env() {
      let a = BuildEnv::new(|| 0);
      let outer_a_ref = &*a;

      env(&a, 1, || {
         let a_ref = &*a;

         assert_ne!(outer_a_ref as *const _, a_ref as *const _);
         assert_eq!(*outer_a_ref, 0);
         assert_eq!(*a_ref, 1);
      });
   }
}
