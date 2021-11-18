use crate::prelude::*;

#[trusted]
#[requires(f.precondition())]
pub fn spawn<T: Send + 'static, F: Send + 'static + FakeFnOnce<Return = T>>(
    f: F,
) -> JoinHandle<T, SpawnPostCond<F>> {
    JoinHandle(
        JoinHandleInner(::std::thread::spawn(
            #[cfg_attr(feature = "contracts", creusot::no_translate)]
            || f.call(),
        )),
        Ghost::record(&SpawnPostCond { f }),
    )
}

pub struct SpawnPostCond<F> {
    f: F,
}

impl<F: FakeFnOnce> Inv<F::Return> for SpawnPostCond<F> {
    #[predicate]
    fn inv(&self, v: F::Return) -> bool {
        self.f.postcondition(v)
    }
}

