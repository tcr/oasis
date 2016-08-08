use ac::{Ac, AcArena};
use ac::types::OMap;
use std::cell::RefCell;
use values::*;

/// Object describing the scope of a single Oasis function. This contains a map
/// associating keys in this scope with values.
///
/// Scopes contain a reference to their parents. This hierarchy formed by
/// following the chain of parent objects allows us to implement lexical scope,
/// i.e. inner functions being able to reference variables that were declared
/// inside of an outer scope.

pub struct Scope {
    pub parent: Option<Ac>,
    pub scope: RefCell<OMap<Expr, Expr>>,
}

impl Scope {
    /// Create a new scope object and allocate it within the context's
    /// allocator.
    pub fn new(ctx: &mut Context, parent: Option<Ac>) -> Ac {
        ctx.allocate(Mem::Scope(Scope {
            parent: parent,
            scope: RefCell::new(OMap::new()),
        }))
    }

    /// Set a key in this scope to the corresponding vale.
    pub fn set(&self, key: Expr, value: Expr) {
        self.scope.borrow_mut().insert(key, value.clone());
    }

    /// Shorthand to set an atom in this scope to the corresponding vale.
    pub fn set_atom(&self, key: &str, value: Expr) {
        self.set(Expr::Atom(key.to_owned()), value)
    }

    /// Look through this scope and all of its ancestor scopes to find a value
    /// matching the given key. If a value by that key is found, we call the
    /// callback function with the value; we either return a Some(value)
    /// returned by this callback or None if the lookup failed.
    pub fn lookup<F, T>(&self, key: &Expr, inner: F) -> Option<T>
        where F: Fn(Option<&Expr>) -> T
    {
        if let Some(value) = self.scope.borrow().search(key, |value| inner(Some(value))) {
            Some(value)
        } else {
            match self.parent {
                Some(ref parent) => parent.get().as_scope().lookup(key, inner),
                None => None,
            }
        }
    }
}

/// Execution context for Oasis that carries all state that should be shared
/// between functions.
///
/// We do not need to track state that is not related to the invocation itself,
/// like the scope we are currently executing in. This is handled by eval().
///
/// We use Context to track global state values like the memory allocator. In
/// addition, we use it as a useful reference to the callstack we are evaluating
/// in, which is required for tail-call optimization.
pub struct Context {
    pub callstack: Vec<(AcId, bool)>,
    alloc: AcArena,
}

impl Context {
    /// Create a default context object.
    pub fn new() -> Context {
        Context {
            callstack: vec![],
            alloc: AcArena::new(),
        }
    }

    /// Shortcut for allocating values using the global allocator.
    pub fn allocate(&mut self, value: Mem) -> Ac {
        self.alloc.pin(value)
    }

    /// Evaluate an expression object. If we are passed in an s-expression, we
    /// defer to eval_expr. If we are passed an atom, we look up its value and
    /// return it.
    pub fn eval(&mut self, scope: Ac, expr: Expr) -> Expr {
        match expr {
            Expr::List(args) => {
                let mut args = args.clone();
                let term = args.remove(0);
                self.eval_expr(scope, term, args)
            }
            Expr::Atom(..) => {
                scope.get()
                    .as_scope()
                    .lookup(&expr, |x| {
                        if let Some(inner) = x {
                            inner.clone()
                        } else {
                            unreachable!("Cannot evaluate value {:?}", expr);
                        }
                    })
                    .expect(&format!("Eval failed to find named value: {:?}", expr))
            }
            _ => expr,
        }
    }

    /// Directly evaluate an s-expression. First look up the atom in
    /// the current scope, then evaluate our arguments in the current context,
    /// or pass them along uninterpreted if we are evaluating a special form.
    /// Returns the result of the expression.
    pub fn eval_expr(&mut self, scope: Ac, x: Expr, args: Vec<Expr>) -> Expr {
        match x {
            Expr::Atom(..) => {
                let (func, special): (Option<Ac>, Option<Ac>) = scope.get()
                    .as_scope()
                    .lookup(&x, |value| {
                        match value {
                            Some(&Expr::Func(ref func)) => (Some(func.clone()), None),
                            Some(&Expr::Special(ref func)) => (None, Some(func.clone())),
                            Some(ref value) => {
                                panic!("Called uncallable value: {:?}", value);
                            }
                            _ => {
                                panic!("Called value that doesn't exist");
                            }
                        }
                    })
                    .expect(&format!("Could not eval unknown atom {:?}", x));

                self.callstack.push((AcId("0x0".to_owned()), false));
                let args: Vec<Expr> = args.into_iter()
                    .map(|x| if func.is_some() {
                        self.eval(scope.clone(), x)
                    } else {
                        x
                    })
                    .collect();
                self.callstack.pop();

                if let Some(func) = func {
                    let call = func.get();
                    let call = call.as_func();
                    let call = call.0;
                    call(self, args)
                } else if let Some(special) = special {
                    let call = special.get();
                    let call = call.as_special();
                    call(self, scope, args)
                } else {
                    Expr::Null
                }
            }
            _ => {
                panic!("Attempted to evaluate non-atom: {:?}", x);
            }
        }
    }
}
