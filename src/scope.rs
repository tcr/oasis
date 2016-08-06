use ac::{Ac, AcArena};
use ac::types::OMap;
use alloc::*;
use std::cell::RefCell;
use values::*;

pub struct Context {
    pub callstack: Vec<(FuncFnId, bool)>,
    pub alloc: AcArena,
}

impl Context {
    pub fn new() -> Context {
        Context {
            callstack: vec![],
            alloc: AcArena::new(),
        }
    }

    pub fn allocate(&mut self, value: Mem) -> Ac {
        self.alloc.pin(value)
    }
}

pub struct Scope {
    pub parent: Option<Ac>,
    pub scope: RefCell<OMap<Expr, Expr>>,
}

impl Scope {
    pub fn new(ctx: &mut Context, parent: Option<Ac>) -> Ac {
        ctx.allocate(Mem::ScopeMem(Scope {
            parent: parent,
            scope: RefCell::new(OMap::new()),
        }))
    }

    pub fn set(&self, key: Expr, value: Expr) {
        self.scope.borrow_mut().insert(key, value.clone());
    }

    pub fn set_atom(&self, key: &str, value: Expr) {
        self.set(Expr::Atom(key.to_owned()), value)
    }

    pub fn lookup<F, T>(&self, key: &Expr, inner: F) -> Option<T>
        where F: Fn(Option<&Expr>) -> T
    {
        if let Some(value) = self.scope.borrow().search(key, |value| {
            inner(Some(value))
        }) {
            Some(value)
        } else {
            match self.parent {
                Some(ref parent) => {
                    parent.get().as_scope().lookup(key, inner)
                }
                None => None,
            }
        }
    }
}

pub fn eval_expr(ctx: &mut Context, scope: Ac, x: Expr, args: Vec<Expr>) -> Expr {
    match x {
        Expr::Atom(..) => {
            let (func, special): (Option<Ac>, Option<Ac>) = scope.get()
                .as_scope()
                .lookup(&x, |value| {
                    match value {
                        Some(&Expr::Func(ref func)) => {
                            (Some(func.clone()), None)
                        }
                        Some(&Expr::Special(ref func)) => {
                            (None, Some(func.clone()))
                        }
                        Some(ref value) => {
                            panic!("Called uncallable value: {:?}", value);
                        }
                        _ => {
                            panic!("Called value that doesn't exist");
                        }
                    }
                })
                .expect(&format!("Could not eval unknown atom {:?}", x));

            // TODO delete attachment to root?
            //ctx.state.roots.push(scope.clone());
            //scope.set_completed(true)

            ctx.callstack.push((FuncFnId("0x0".to_owned()), false));
            let args: Vec<Expr> = args.into_iter()
                .map(|x| if func.is_some() {
                    eval(ctx, scope.clone(), x)
                } else {
                    x
                })
                .collect();
            ctx.callstack.pop();

            let ret = if let Some(func) = func {
                let call = func.get();
                let call = call.as_func();
                let call = &call.body;
                call(ctx, args)
            } else if let Some(special) = special {
                let call = special.get();
                let call = call.as_special();
                call(ctx, scope, args)
            } else {
                Expr::Null
            };

            //ctx.state.roots.pop();
            ret
        }
        _ => {
            panic!("Attempted to evaluate non-atom: {:?}", x);
        }
    }
}

pub fn eval(ctx: &mut Context, scope: Ac, expr: Expr) -> Expr {
    match expr {
        Expr::List(args) => {
            let mut args = args.clone();
            let term = args.remove(0);
            eval_expr(ctx, scope, term, args)
        }
        Expr::Atom(..) => {
            //println!("why is scope scope {:?}", scope);
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
