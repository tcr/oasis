# oasis

LISP in Rust using LALRPOP and some other things.

```
echo "(+ 1 (index (vec 0 0 1) 2))" | cargo run
```

or e.g.

```
cat test/binary-trees.oasis | cargo run --release
```

**TODO:** Tail call recursion. Please help.

## License

MIT and Apache-2.0
