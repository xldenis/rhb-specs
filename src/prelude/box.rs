// SPEC LINES 2
use crate::prelude::*;

// A version of Box::leak

#[trusted]
#[ensures(*result === *b)]
pub fn leak<'a, T: 'a>(b: Box<T>) -> &'a mut T {
    Box::leak(b)
}
