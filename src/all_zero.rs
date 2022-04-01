use crate::prelude::*;

// All values in the final value of `v` (after the call) are 0
#[ensures(forall<i : Int> 0 <= i && i < (@^v).len() ==> (@^v)[i] === 0u32)]
// The length is unchanged
#[ensures((@*v).len() === (@^v).len())]
fn all_zero(v: &mut Vec<u32>) {
    let mut i = 0;
    let old_v = Ghost::record(&v);
    // The prophecy never changes, this is required due to a limitation in why3
    #[invariant(proph_const, ^v === ^@old_v)]
    // The length doesn't change
    #[invariant(in_bounds, (@*v).len() === (@*@old_v).len())]
    // The values of all cells at index less than i is 0
    #[invariant(all_zero, forall<j : Int> 0 <= j && j < i.into() ==> (@*v)[j] === 0u32)]
    while i < v.len() {
        v[i] = 0;
        i += 1;
    }
}
