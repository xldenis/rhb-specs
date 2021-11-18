use crate::prelude::*;

#[trusted]
#[ensures(^dest === src)]
#[ensures(result === *dest)]
pub fn replace<T>(dest: &mut T, src: T) -> T {
    std::mem::replace(dest, src)
}
