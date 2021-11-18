// SPEC LINES 5 + 8
use crate::prelude::*;

struct Even;

impl Inv<u32> for Even {
    #[predicate]
    fn inv(&self, x: u32) -> bool {
        x % 2u32 == 0u32
    }
}

struct AddsTwo<'a> {
    mutex: &'a Mutex<u32, Even>,
}

impl<'a> FakeFnOnce for AddsTwo<'a> {
    type Return = ();
    #[predicate]
    fn precondition(self) -> bool {
        true
    }

    #[predicate]
    fn postcondition(self, _: ()) -> bool {
        true
    }

    fn call(self) -> () {
        let mut v = self.mutex.lock();
        let val = *v.deref();
        if val < 100000 {
            v.set(val + 2);
        } else {
            v.set(0);
        }
    }
}

fn concurrent() {
    let m: &'static _ = leak(Box::new(Mutex::new(0, Even)));
    let t1 = AddsTwo { mutex: &m };
    let j1 = spawn(t1);
    let t2 = AddsTwo { mutex: &m };
    let j2 = spawn(t2);

    j1.join();
    j2.join();

    // assert!(m.into_inner() % 2 == 0);
}
