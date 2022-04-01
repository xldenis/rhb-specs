// SPEC LINES 2
use crate::prelude::*;

enum List<T> {
    Nil,
    Cons(Node<T>),
}
use List::*;
type Node<T> = Box<(T, List<T>)>;

// A pure, logical helper function, reverses n onto the head of o.
// rev_append l nil == rev l
#[logic]
fn rev_append<T>(n: List<T>, o: List<T>) -> List<T> {
    match n {
        Nil => o,
        Cons(box (hd, tl)) => rev_append(tl, Cons(Box::new((hd, o)))),
    }
}

#[ensures(^l === rev_append(*l, Nil))]
fn rev<T>(l: &mut List<T>) {
    // Record the original value of the list as ghost information
    let old_l = Ghost::record(&*l);
    let mut prev = Nil;
    let mut head = replace(l, Nil);
    // rev head ++ prev == rev old_l ++ nil
    #[invariant(x, rev_append(head, prev) === rev_append(@old_l, Nil))]
    while let Cons(mut curr) = head {
        let next = curr.1;
        curr.1 = prev;
        prev = Cons(curr);
        head = next;
    }
    *l = prev;
}
