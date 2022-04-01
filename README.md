# RustHornBelt Library & Benchmarks

This crate contains the evaluation libraries and benchmarks for the RustHornBelt PLDI paper.

The crate includes the following benchmarks:

1. Fibonacci Cell, demonstrates how the RustHornBelt representation of Cells as invariants is expressive enough to express memoization by memoizing a fibonacci calculation.
2. Even Cell, demonstrates a simple invariant cell which always holds an even value.
3. In Place List reversal, reverses a linked list inplace using a memswap like function, proved correct in RustHornBelt.
4. Even Mutex, demonstrates how to use an invariant representation for a mutex by storing and incrementing an even value within one.
5. Knight's Tour, proves the safety of a Knight's Tour calculation, using specifications proved for vectors.
6. All Zero, zeroes the cells of a vector using indexing.
7. Inc Vec, increments the cells of a vector using a mutable iterator, proved correct in RustHornBelt.

Each example is in its own file inside of `src/`

All RustHornBelt specifications are provided in `src/prelude/`.

## Brief overview of Creusot specification syntax.

Creusot specifications are written in a first-order program logic, which includes two novel operators.
The `^` operator (pronounced *final*) accesses the prophecy of a mutable borrow, this corresponds to taking the second projection of the representation of a mutable borrow in RustHornBelt.
The `@` operator (pronounced *model*) accesses the *representation value* of a type, like `⌊_⌋` in RustHornBelt.
To access the *current* value of a mutable borrow, we just use normal dereferencing, as it is just the value being pointed to at any moment by the borrow.

So the resolution of a mutable borrow can be expressed as the equality: `^ x == * x`, stating that the final value of `x` equals the current value of `x`.

## Generating and verifying proofs yourself

1. To verify the benchmarks, first install Creusot:

```
git clone https://github.com/xldenis/creusot
cargo install --path creusot
```

2. Install Why3, following Creusot's installation guide:

```
opam pin add why3 https://gitlab.inria.fr/why3/why3.git#master
```

3. Install Z3 and CVC4.

4. Configure why3:

```
why3 config detect
```

4. Then compile the crate using Creusot:

```
make proofs
```

5. Finally, load the proofs in why3

```
why3 ide -Lpath/to/creusot/prelude proofs.mlcfg
```

