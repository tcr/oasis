# oasis

LISP in Rust. Uses LALRPOP. Implements tail-call optimization.

```
echo "(+ 1 (index (vec 0 0 1) 2))" | cargo run
```

or e.g.

```
cat test/binary-trees.oasis | cargo run --release
```

## License

MIT and Apache-2.0
