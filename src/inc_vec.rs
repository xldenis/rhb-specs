use crate::prelude::*;

// Require that each index in the vector holds a value less than or equal to 10.
#[requires(forall<i : Int> 0 <= i && i < (@^v).len() ==> @(@v)[i] <= 10)]
// The length of the vector is unchanged by the function call
#[ensures((@^v).len() === (@v).len())]
// Every index in the vector is incremented by 5 after the call
#[ensures(forall<i : Int> 0 <= i && i < (@^v).len() ==> @(@^v)[i] === @(@v)[i] + 5)]
fn inc_vec(v: &mut Vec<u32>) {
    // Record the original value of the vector as ghost information
    let old_v = Ghost::record(&v);

    let mut it = v.iter_mut();

    let mut _ghost_seen: usize = 0; // Creusot doesn't yet have mutable ghost code

    // for every index we've already iterated, the value is incremented by 5 relative to the original value of old_v
    #[invariant(incremented, forall<i : Int>
        0 <= i && i < @_ghost_seen ==> @(@^@old_v)[i] === @(@@old_v)[i] + 5
    )]
    // The values we haven't seen are unchanged.
    #[invariant(to_come, forall<i : Int> 0 <= i && i < (@it).len() ==>
        *(@it)[i] === (@@old_v)[i + @_ghost_seen] && ^(@it)[i] === (@^@old_v)[i + @_ghost_seen]
    )]
    // the length of the remaining iterator + the values we have seen is the same as the original lenght
    #[invariant(_ghost_seen, @_ghost_seen + (@it).len() === (@@old_v).len())]
    while let Some(r) = it.next() {
        *r += 5;
        _ghost_seen += 1;
    }
}
