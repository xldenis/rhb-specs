// SPEC LINES 5
use crate::prelude::*;

struct Even;

impl Inv<u64> for Even {
    #[predicate]
    fn inv(&self, x: u64) -> bool {
        x % 2u64 == 0u64
    }
}

fn adds_two(c: &Cell<u64, Even>) {
    let v = c.get();
    // To shut up overflow checking
    if v < 100000 {
        c.set(v + 2);
    } else {
        c.set(0);
    }
}
