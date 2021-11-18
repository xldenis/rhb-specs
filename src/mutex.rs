use crate::prelude::*;

#[trusted]
struct MutexInner<T>(::std::sync::Mutex<T>);

pub struct Mutex<T, I>(MutexInner<T>, I);
// We ignore poisoning, thus we don't use `LockResult` like in `std`.
impl<T, I: Inv<T>> Mutex<T, I> {
    #[trusted]
    #[requires(i.inv(val))]
    pub fn new(val: T, i: I) -> Self {
        Mutex(MutexInner(::std::sync::Mutex::new(val)), i)
    }

    #[trusted]
    #[ensures(self.1.inv(result))]
    pub fn into_inner(self) -> T {
        self.0 .0.into_inner().unwrap()
    }

    #[trusted]
    #[ensures((*self).1.inv(*result))]
    #[ensures(forall<v: T> (^self).1.inv(v) === true)]
    pub fn get_mut(&mut self) -> &mut T {
        self.0 .0.get_mut().unwrap()
    }

    #[trusted]
    #[ensures(self.1 === @(result.1))]
    pub fn lock(&self) -> MutexGuard<'_, T, I> {
        MutexGuard(GuardInner(self.0 .0.lock().unwrap()), Ghost::record(&self.1))
    }
}

#[trusted]
struct GuardInner<'a, T: ?Sized + 'a>(::std::sync::MutexGuard<'a, T>);
pub struct MutexGuard<'a, T: ?Sized + 'a, I>(GuardInner<'a, T>, Ghost<I>);

impl<'a, T, I: Inv<T>> MutexGuard<'a, T, I> {
    #[trusted]
    #[ensures((@(self.1)).inv(*result))]
    pub fn deref(&self) -> &T {
        &*self.0 .0
    }

    #[trusted]
    #[requires((@(self.1)).inv(v))]
    pub fn set(&mut self, v: T) {
        *self.0 .0 = v;
    }
}

#[trusted]
pub(crate) struct JoinHandleInner<T>(pub(crate) ::std::thread::JoinHandle<T>);
pub struct JoinHandle<T, I>(pub(crate) JoinHandleInner<T>, pub Ghost<I>);

impl<T, I: Inv<T>> JoinHandle<T, I> {
    #[trusted]
    #[ensures(match result {
      Ok(v) => (@(self.1)).inv(v),
      _ => true,
    })]
    pub fn join(self) -> Result<T, ()> {
        match self.0 .0.join() {
            Ok(v) => Ok(v),
            Err(_) => Err(()),
        }
    }
}
