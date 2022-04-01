// SPEC LINES 11 + 14 + 4 + 11 + 6 + 7
use crate::prelude::*;

// Naive, mathematical fibonacci
#[logic]
#[variant(i)]
fn fib(i: Int) -> Int {
    if i <= 0 {
        0
    } else if i == 1 {
        1
    } else {
        fib(i - 1) + fib(i - 2)
    }
}

// Show a weak upper bound on the value of a fibonacci number
#[logic]
#[requires(0 <= i)]
#[ensures(fib(i) <= 2.pow(i))]
#[variant(i)]
fn lemma_fib_bound(i: Int) {
    if i == 0 {
        ()
    } else if i == 1 {
        ()
    } else {
        lemma_fib_bound(i - 2);
        lemma_fib_bound(i - 1)
    }
}

#[trusted]
#[logic]
#[ensures(2.pow(63) < @0xffff_ffff_ffff_ffffusize)]
fn lemma_max_int() {}

struct Fib {
    ix: usize,
}
// An invariant which depends on a value `ix` over an option of usize.
// If a value is present, then it is the ix-th fibonacci number.
impl Inv<Option<usize>> for Fib {
    #[predicate]
    fn inv(&self, v: Option<usize>) -> bool {
        pearlite! {
            match v {
                None => true,
                Some(i) => @i === fib(@self.ix)
            }
        }
    }
}

type FibCache = Vec<Cell<Option<usize>, Fib>>;

// The cell invariants have the correct indices
#[predicate]
fn fib_cell(v: FibCache) -> bool {
    pearlite! {
        forall<i : Int> @(@((@v)[i].ghost_inv)).ix === i
    }
}

// Given a cache with the proper invariants
#[requires(fib_cell(*mem))]
#[requires(@i < (@mem).len())]
// Purely for overflow reasons
#[requires(@i <= 63)]
// Return the ith fibonacci number
#[ensures(@result === fib(@i))]
fn fib_memo(mem: &FibCache, i: usize) -> usize {
    match mem[i].get() {
        Some(v) => v,
        None => {
            let fib_i = if i == 0 {
                0
            } else if i == 1 {
                1
            } else {
                // Load the lemmas to prove safety of addition
                proof_assert! { {lemma_max_int(); true} };
                proof_assert! { {lemma_fib_bound(0); true} };
                fib_memo(mem, i - 1) + fib_memo(mem, i - 2)
            };
            // assert we have the correct result.
            proof_assert! { @fib_i === fib(@i)};
            mem[i].set(Some(fib_i));
            fib_i
        }
    }
}
