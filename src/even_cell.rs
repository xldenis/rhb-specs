// SPEC LINES 5
use crate::prelude::*;

struct Even;

impl Inv<u32> for Even {
    #[predicate]
    fn inv(&self, x: u32) -> bool {
        x % 2u32 == 0u32
    }
}

fn adds_two(c: &Cell<u32, Even>) {
    let v = c.get();
    // To shut up overflow checking
    if v < 100000 {
        c.set(v + 2);
    } else {
        c.set(0);
    }
}
