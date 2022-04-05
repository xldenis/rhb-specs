# RustHornBelt Benchmark walkthrough

Here we present a quick introduction on how the specifications we proved using RustHornBelt can be used in an automatic context. 
For this walkthrough, we will be using the semi-automatic verifier, Creusot.

Creusot is based on the same prophetic model as the one proved in RustHornBelt, in particular mutable borrows are represented as pairs of values: a current and *final* value.

# Verifying `all_zero`

We will start by verifying a simple function which stores 0 into each element of a vector. Specifically, we are interested in verifying the correctness of the following function:

```rust
fn all_zero(v: &mut Vec<u32>) {
    let mut i = 0;
    while i < v.len() {
        v[i] = 0;
        i += 1;
    }
}
```

## Specifying the contract to our function.

Our first challenge is to give a top-level specification to `all_zero`. We express this using two post-conditions:

1. The length of our vector is unchanged after the call to our function. The way we express this property is by stating that the *final* value of the borrow `v` has the same length as the *initial* value of `v`. We write this as:
  ```rust
    (@^v).len() === (@*v).len()
  ```

  Here the symbol `^` accesses the final value of a borrow while `*` access the initial value. 
  In Creusot, the *representation value* of the vector is then accessed using the operator `@`. Thus, the expression `@^v` corresponds to the representation of the final value of the borrow `v`. 
  Finally, `(@^v).len() === (@*v).len()` asserts that these two lengths are the same.
2. The second postcondition checks that after the call, the vector only stores `0`. We express this property using the following specification:
```rust
forall<i : Int> 0 <= i && i < (@^v).len() ==> (@^v)[i] === 0u32)
```
This statest that for all indicies `i` in the vector, the final value of the borrow `v` will have `0` at index `i`.

## Verifying our contract

We can take our annotated function (below), and run it through Creusot to attempt verification.

```rust
#[ensures((@^v).len() === (@*v).len())]
#[ensures(forall<i : Int> 0 <= i && i < (@^v).len() ==> (@^v)[i] === 0u32)]
fn all_zero(v: &mut Vec<u32>) {
    let mut i = 0;
    while i < v.len() {
        v[i] = 0;
        i += 1;
    }
}
```

Once we do this, we get a large `*.mlcfg` file that can be loaded in Why3. In the Why3 ide, the left-hand column lists all the proof goals and their state. A green checkmark indicates the goal is proven, while the blue question mark indicates it remains to be shown. 

In this column, a folder icon with the name `RhbSpecs_AllZero_AllZero` can be found, which contains a single entry called `all_zero'vc [VC for all_zero]`. This is the *verification condition* for `all_zero`. We can ask why3 to show us the actual logical condition which must be proved by click on the `Task` tab in the upper-right pane. This will show us a large logical formula:

```
goal all_zero'vc :
  forall v:borrowed (rhbspecs_prelude_vec_vec uint32).
   let o = () in
   let _0'unused'unused = ref'mk o in
   let ref'result'unused'unused = ref'mk o in
   forall o1:borrowed (rhbspecs_prelude_vec_vec uint32).
    let any'result'unused'unused = o1 in
    let v_1'unused'unused = ref'mk o1 in
    ....
```

We can ask Why3 to attempt to prove this goal by selecting the entry in the left-hand column and pressing `0`, which will launch all configured provers on this task for 1 second. 

Sadly, after that second the provers will fail to prove our function! To understand why this is happening, Why3 provides us with *transformations* that can be used to dissect proof obligations. 

Select the `all_zero'vc [VC for all_zero]` task and hit `s` on your keyboard to execute the `split_vc` transformation. 
This will decompose the original vc into a set of more atomic obligations which imply the original one. 
For this example, it will produce the following tree:

```
❔ all_zero'vc [VC for all_zero]
  ❔ split_vc 
    ❔ 0 [postcondition]
    ❔ 1 [postcondition]
    ❔ 2 [precondition]
    ❔ 3 [precondition]
```

By selecting the `split_vc` node, we can execute the provers using `0` once again, which will prove the last two goals, producing the tree:

```
❔ all_zero'vc [VC for all_zero]
  ❔ split_vc 
    ❔ 0 [postcondition]
    ❔ 1 [postcondition]
    ✅ 2 [precondition]
    ✅ 3 [precondition]
```

The two postconditions are still not provable! If we select `1 [postcondition]` and click on the `Task` pane, we can now see that the goal has been changed and simplified:

```
goal all_zero'vc : length3 (model3 (^ v)) = length3 (model3 ( * v))
```

Additionally, we can see a series of hypotheses which tell us information about the variables in scope. 
In our case, the issue we face is that our function body uses a loop but we have not provided any invariants, making it impossible to know the value of the vector `v` after the loop.

## Adding invariants to `all_zero`

We return to the Rust code, and add loop invariants using the `#[invariant(name, expr)]` annotation in Creusot. 

To start, we may want to say that at each iteration of our loop, the vector has not changed its value compared to the entry of the function. To do this, we need some way of referring to the value at the entry of the function, which we achieve in Creusot using *ghost values*. The `Ghost::record` function takes a snapshot of a value which can be used during proofs but has no impact on execution.

Using this, we can record the original value of `v`:

```rust
let old_v = Ghost::record(&v);
```

We can now add an invariant to our while loop:
```rust
#[ensures((@^v).len() === (@*v).len())]
// All values in the final value of `v` (after the call) are 0
#[ensures(forall<i : Int> 0 <= i && i < (@^v).len() ==> (@^v)[i] === 0u32)]
fn all_zero(v: &mut Vec<u32>) {
    let mut i = 0;
    let old_v = Ghost::record(&v);
    #[invariant(in_bounds, (@*v).len() === (@*@old_v).len())]
    while i < v.len() {
        v[i] = 0;
        i += 1;
    }
}
```

We can regenerate our proof obligations, and load them up in why3 again, and note that there are two more entries in the split goal for `all_zero`, but we *still* can't prove our postconditions, not even the one on length. 

Here we are encountering a limitation of Why3 & Creusot: they forget certain information about the final values of borrows.
This is easy to rectify by adding a second invariant to the loop:

```rust
#[invariant(proph_const, ^v === ^@old_v)]

```

This invariant states that the final value of a vector doesn't change, a fact guaranteed in the model of RustHornBelt, but which we must show manually in Creusot. 
If we reload the proofs once more, we can prove our first postcondition.

To finish our proof, we add a final invariant:

```rust
#[invariant(all_zero, forall<j : Int> 0 <= j && j < @i ==> (@*v)[j] === 0u32)]
```

Which expresses the second property: for every index less than `i` the vector stores a `0`. Then after the end of our loop, we can conclude that every index of the vector must have a `0` inside. 
