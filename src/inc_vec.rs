// SPEC LINES 11

use crate::prelude::*;

#[ensures((@^v).len() === (@v).len())]
#[ensures(forall<i : Int> 0 <= i && i < (@^v).len() ==> @(@^v)[i] === @(@v)[i] + 5)]
fn inc_vec(v: &mut Vec<u32>) {
    let old_v = Ghost::record(&v);

    let mut it = v.iter_mut();
    let mut _ghost_seen: usize = 0; // Creusot doesn't yet have ghost code
    #[invariant(incremented, forall<i : Int>
        0 <= i && i < @_ghost_seen ==>
        @(@^@old_v)[i] === @(@@old_v)[i] + 5
    )]
    #[invariant(to_come, forall<i : Int> 0 <= i && i < (@it).len() ==>
        *(@it)[i] === (@@old_v)[i + @_ghost_seen] && ^(@it)[i] === (@^@old_v)[i + @_ghost_seen]
    )]
    #[invariant(_ghost_seen, @_ghost_seen + (@it).len() === (@@old_v).len())]
    while let Some(r) = it.next() {
        *r += 5;
        _ghost_seen += 1;
    }
}
