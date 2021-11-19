// SPEC LINES 4 + 4 (INV)
use creusot_contracts::*;

pub trait Inv<T> {
    #[predicate]
    fn inv(&self, x: T) -> bool;
}

pub struct Cell<T, I> {
    inner: ::std::cell::Cell<T>,
    pub ghost_inv: Ghost<I>,
}

impl<T: Copy, I: Inv<T>> Cell<T, I> {
    #[trusted]
    #[ensures((@(self.ghost_inv)).inv(result))]
    pub fn get(&self) -> T {
        self.inner.get()
    }

    #[trusted]
    #[requires((@(self.ghost_inv)).inv(v))]
    pub fn set(&self, v: T) {
        self.inner.set(v)
    }
}
