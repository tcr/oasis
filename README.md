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

* Fix binary-trees to work with GC system
* Make defn freeze its statements so it doesn't need to GC them?
* Move to generic alloc system to switch between RefCell and GC
* then simplify alloc! call
* make it possible for GC to happen at any time—i.e. objects should not have to
be explicitly added to a scope if they could have been GC'd in between creation
and addition to scope
* finally set up to persue an epoch based multi-thread setup!!
* LPRI: Pursure immutability (vecs vs lists)

## License

MIT and Apache-2.0
