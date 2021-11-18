# RustHornBelt Library & Benchmarks

This crate contains the evaluation libraries and benchmarks for the RustHornBelt PLDI paper.

The crate includes the following benchmarks:

1. Fibonacci Cell
2. Even Cell
3. In Place List reversal
4. Even Mutex
5. Knight's Tour
6. All Zero
7. Inc Vec

Each example is in its own file inside of `src/`

All RustHornBelt specifications are provided in `src/prelude/`.

## Generating and verifying proofs yourself

1. To verify the benchmarks, first install Creusot:

```
git clone https://github.com/xldenis/creusot
cargo install --path creusot
```

2. Install Why3, following Creusot's installation guide:

```
opam pin add why3 https://gitlab.inria.fr/why3/why3.git#stackify
```

3. Install Z3 and CVC4.

4. Configure why3:

```
why3 config detect
```

4. Then compile the crate using Creusot:

```
CREUSOT_CONTINUE=1 cargo creusot --features=contracts > proofs.mlcfg
```

5. Finally, load the proofs in why3

```
why3 ide -Lpath/to/creusot/prelude proofs.mlcfg
```

