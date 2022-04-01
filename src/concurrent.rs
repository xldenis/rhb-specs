// SPEC LINES 5 + 8
use crate::prelude::*;

struct Even;

// Create an invariant which checks with a number is even
impl Inv<u32> for Even {
    #[predicate]
    fn inv(&self, x: u32) -> bool {
        x % 2u32 == 0u32
    }
}

// Closures in Rust are represented using closure-conversion, which we do manually here.
struct AddsTwo<'a> {
    mutex: &'a Mutex<u32, Even>,
}

impl<'a> FakeFnOnce for AddsTwo<'a> {
    type Return = ();
    // Empty precondition
    #[predicate]
    fn precondition(self) -> bool {
        true
    }

    // Empty postcondition
    #[predicate]
    fn postcondition(self, _: ()) -> bool {
        true
    }

    // Increment or reset to 0 if the value is too big
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

// Create a mutex, spawn two threads which run the `AddsTwo` closure and then join them.
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
