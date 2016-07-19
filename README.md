# oasis

LISP in Rust. Uses LALRPOP. Implements tail-call optimization.

```
echo "(+ 1 (index (vec 0 0 1) 2))" | cargo run
```

or e.g.

```
cat test/binary-trees.oasis | cargo run --release
```

## TODO

* Separate new allocs from GC-able allocs
* Make external thread able to iterate entire GC tree
* make it possible for GC to happen at any time—i.e. objects should not have to
be explicitly added to a scope if they could have been GC'd in between creation
and addition to scope

More:

* Make defn freeze its statements so it doesn't need to GC them?
* then simplify alloc! call
* LPRI: Pursure immutability (vecs vs lists)
* Move to generic alloc system to switch between RefCell and GC?
* Determine if scope benefits from append-only lists
* Determine if HAMT is too much for a read-only list (no atomics?)
* Determine if vectors can be simplified by ctries

## License

MIT and Apache-2.0
