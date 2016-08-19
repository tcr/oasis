# oasis

LISP in Rust. Uses LALRPOP. Implements tail-call optimization.

```
echo "(+ 1 (index (vec 0 0 1) 2))" | cargo run
```

or e.g.

```
cat test/binary-trees.oasis | cargo run --release
```

## References

* GC Handbook http://gchandbook.org/

## TODO

* make it possible for GC to happen at any time—i.e. objects should not have to
be explicitly added to a scope if they could have been GC'd in between creation
and addition to scope

* Figure out how to prevent GC for objects which are temporarily on the stack /
being passed between fns. Right now we set rooted too early for things that maybe
are not actually rooted. Perhaps we need to set "floating" instead?

* could set integer that says how many times copied out as expression—could be
added and removed from expressions set (or iterated over in general)

* Need to figure out how split trees work—if we iterate a vector that was an
expression object but switched to an attached object, we can be caught in the
middle of an iteration that sees it as not being attached (since scoped info
  comes from the parent, not from the child)

* see the (vec 1 2 3 ...) example where the above can happen

- vec on stack is rooted
- vec attached is checked, if parent is marked it is queued for marking
- if vec is unchecked it is dequeued for marking but back on stack soooo
- need a queue for stack items? need a queue for the exprs? dunno

* is reachability analysis not enough here? do we need to keep track of # of
objects and # of expressions an object is used as and atomically modify them??

* can the roots array be converted into a set with uniquee UUIDs (not a vector)

TL;DR need to read actual articles about GC iteration in multithreaded setups

More:

* Make defn freeze its statements so it doesn't need to GC them?
* then simplify alloc! call
* LPRI: Pursure immutability (vecs vs lists)
* Move to generic alloc system to switch between RefCell and GC?
* Determine if scope benefits from append-only lists
* Determine if OMap is too much for a read-only list (no atomics?)
* Determine if vectors can be simplified by ctries

## License

MIT and Apache-2.0
