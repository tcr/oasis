use ac::Ac;
use ac::types::OVec;
use rand::{self, Rng};
use scope::*;
use std::collections::HashMap;
use std::mem;
use strfmt::strfmt;
use values::*;

/// (def *id* *value*)
/// Implements the special form `def`. Assigns the identifier *id* in the
/// current scope to *value*. This is able to be referenced by any subsequent
/// expressions; see `let` for creating a new scope.
fn special_def(ctx: &mut Context, scope: Ac, mut args: Vec<Expr>) -> Expr {
    let key = args.remove(0);
    let value = ctx.eval(scope.clone(), args.remove(0));
    scope.get().as_scope().set(key, value);
    Expr::Null
}

/// (def *id* (*arg-names...*) *expressions...*)
/// Implements the special form `defn`. Assigns the identifier *id* in the
/// current scope to a function that takes *arg-names* and executes
/// *expressions*. This function enables user code to define its own functions.
///
/// Each function, when called, as an entry to the context's *call stack*. It
/// then evaluates the inner expressions, before popping its entry off the call
/// stack.
///
/// Crucially, `defn` implements logic for *tail-call optimization*. When a
/// function is invoked in *tail position* (i.e. is the last expression to be
/// invoked in a function), we check if the function being called already exists
/// in the callstack and is marked as being the last expression being evaluated.
/// If so, we return a special expression value `TailCall`. The most recent
/// invocation of the function in the callstack checks if a `TailCall` value is
/// returned, and if its function ID matches itself, it loops and repeats its
/// invocation as though it were called with those arguments. This optimization
/// allows us to recursively call a function without infinitely expanding the
/// system's call stack, which has an upper limit.
fn special_defn(ctx: &mut Context, scope: Ac, mut args: Vec<Expr>) -> Expr {
    use std::rc::Rc;
    use std::sync::RwLock;

    let key = args.remove(0);
    let names: Vec<Expr> = if let Expr::List(content) = args.remove(0) {
        content
    } else {
        vec![]
    };

    let parent_scope = scope.clone();
    let inner_ref: Rc<RwLock<Option<AcId>>> = Rc::new(RwLock::new(None));
    let outer_ref = inner_ref.clone();

    let content = args;
    let closure: Ac = ctx.allocate(Mem::Func(
        Box::new(move |ctx: &mut Context, mut args: Vec<Expr>| {
            // Check for TCO.
            let fn_ptr = inner_ref.read().unwrap();
            let fn_id = fn_ptr.clone().expect("No FunFnId for this function.");
            assert!(args.iter()
                .all(|x| {
                    match x {
                        &Expr::TailCall(..) => false,
                        _ => true
                    }
                }), "Found tail call expr in args position");

            if ctx.callstack.iter().rev()
                .take_while(|x| x.1)
                .any(|x| x.0 == fn_id) {
                // Return early with evaluated arguments.
                return Expr::TailCall(fn_id, args);
            }

            // Otherwise, add to call stack and evaluate.
            let pos = ctx.callstack.len();
            ctx.callstack.push((fn_id.clone(), false));

            // Evaluate contents.
            let mut res = Expr::Null;
            loop {
                // We are not in tail-call position.
                ctx.callstack[pos].1 = false;

                // Create inner function bindings.
                let s2 = Scope::new(ctx, Some(parent_scope.clone()));
                for (item, value) in names.iter().zip(args) {
                    s2.get().as_scope().set((*item).clone(), value.clone());
                }

                let len = content.len();
                for (i, statement) in content.iter().enumerate() {
                    // When we are evaluating the last statement, change our Context
                    // to indicate we are in terminal position
                    if i + 1 == len {
                        ctx.callstack[pos].1 = true;
                    }
                    res = ctx.eval(s2.clone(), statement.clone());
                }

                // Evaluate tail call expressions if they match this function.
                if match res {
                    Expr::TailCall(ref inner_fn_id, _) => {
                        *inner_fn_id == fn_id
                    },
                    _ => false,
                } {
                    if let Expr::TailCall(_, inner_args) = mem::replace(&mut res, Expr::Null) {
                        args = inner_args;
                        continue;
                    }
                }

                break;
            }

            ctx.callstack.pop();
            res
        }),
        scope.clone(),
    ));

    // Store unique closure ID.
    *outer_ref.write().unwrap() = Some(closure.id());
    scope.get().as_scope().set(key, Expr::Func(closure.clone()));

    Expr::Null
}

/// (if *cond* *then* *else*)
/// Implements the special form `if`. If the expression *cond* is *truthy*, then
/// the *then* clause is evaluated. Otherwise, *else* is evaluated.
fn special_if(ctx: &mut Context, scope: Ac, mut args: Vec<Expr>) -> Expr {
    let if_val = args.remove(0);
    let then_val = args.remove(0);
    let else_val = args.remove(0);

    if ctx.eval(scope.clone(), if_val).as_bool() {
        ctx.eval(scope.clone(), then_val)
    } else {
        ctx.eval(scope.clone(), else_val)
    }
}

/// (let (*bindings...) *expressions...*)
/// Implements the special form `let`. Creates a scope in which the bindings
/// (a list of subsequent *identifier* *value* pairs) are set and in which the
/// *expressions* are evaluated.
fn special_let(ctx: &mut Context, scope: Ac, mut args: Vec<Expr>) -> Expr {
    let bindings = if let Expr::List(content) = args.remove(0) {
        content
    } else {
        vec![]
    };
    let content = args;

    // Extract alternating *key* *value* pairs and set them in the scope.
    let s2 = Scope::new(ctx, Some(scope.clone()));
    for win in bindings[..].chunks(2) {
        let item = win[0].clone();
        let value = win[1].clone();
        let value = ctx.eval(s2.clone(), value);
        s2.get().as_scope().set(item, value);
    }

    let mut res = Expr::Null;
    for statement in &content {
        res = ctx.eval(s2.clone(), statement.clone());
    }

    res
}

/// (+ *left* *right*)
/// Adds two numbers together. Each value is casted to a number during
/// evaluation.
fn eval_add(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(args.iter()
        .map(|x| x.as_int())
        .fold(0, |sum, val| sum + val))
}

/// (- *left* *right*)
/// Subtracts two numbers. Each value is casted to a number during evaluation.
fn eval_sub(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], args.get(1)) {
        (&Expr::Int(a), Some(&Expr::Int(b))) => a - b,
        (&Expr::Int(a), None) => -a,
        _ => 0,
    })
}

/// (* *left* *right*)
/// Multiplies two numbers. Each value is casted to a number during evaluation.
fn eval_mul(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a * b,
        _ => 0,
    })
}

/// (/ *left* *right*)
/// Divides two numbers. Each value is casted to a number during evaluation.
fn eval_div(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a / b,
        _ => 0,
    })
}

/// (<< *left* *right*)
/// Bit shifts *left* by *right* bits. Each value is casted to a number during
/// evaluation.
fn eval_bitshiftleft(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a << b,
        _ => 0,
    })
}

/// (== *left* *right*)
/// Returns a boolean value representing whether *left* is an equivalent value
/// to *right*.
fn eval_eq(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let a = args.remove(0);
    let b = args.remove(0);

    if a == b {
        Expr::Int(1)
    } else {
        Expr::Int(0)
    }
}

/// (< *left* *right*)
/// Returns a boolean value representing whether *left* is a lesser value than
/// *right*. Each value is casted to a number during evaluation.
fn eval_le(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let a = args.remove(0);
    let b = args.remove(0);

    if a.as_int() < b.as_int() {
        Expr::Int(1)
    } else {
        Expr::Int(0)
    }
}

/// (vec *item...*)
/// Creates a new heap-allocated vector of the *item* values passed to this
/// function.
fn eval_vec(ctx: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Vec(ctx.allocate(Mem::Vec(OVec::new_from(args))))
}

/// (index *list* *index*)
/// Returns the value at the *index* indice in the *list* object. Returns null
/// if the *index* is past the length of the *list*.
fn eval_index(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let value = args.remove(0);
    let key = args.remove(0);

    let value_vec = value.as_vec();
    value_vec.get((key.as_int() as usize), |value| value.clone())
        .unwrap_or(Expr::Null)
}

/// (first *list*)
/// Returns the first (0th) value in the *list*.
fn eval_first(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let value = args.remove(0);

    let value_vec = value.as_vec();
    value_vec.get(0, |value| value.clone())
        .unwrap_or(Expr::Null)
}

/// (rest *list*)
/// Returns a vector of all values in *list* but the first.
fn eval_rest(ctx: &mut Context, mut args: Vec<Expr>) -> Expr {
    args.remove(0);
    Expr::Vec(ctx.allocate(Mem::Vec(OVec::new_from(args))))
}

/// (null? *value*)
/// Returns true is the value is equal to null, false otherwise.
fn eval_nullq(_: &mut Context, args: Vec<Expr>) -> Expr {
    match &args[0] {
        &Expr::Null => Expr::Int(1),
        _ => Expr::Int(0),
    }
}

fn eval_println(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let fmt = args.remove(0).as_string();

    let mut vars = HashMap::new();
    for (i, value) in args.iter().enumerate() {
        vars.insert(format!("{}", i), value.as_string());
    }

    println!("{}", strfmt(&fmt, &vars).unwrap());
    Expr::Null
}

/// (random *max*)
/// Returns a random integer in the range from 0 to *max*.
fn eval_random(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let n = args.remove(0);

    let mut rng = rand::thread_rng();
    Expr::Int(rng.gen_range(0, n.as_int()))
}

/// (len *list*)
/// Returns an integer representing the length of *list*.
fn eval_len(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let list = args.remove(0);

    let vec = list.as_vec();
    Expr::Int(vec.len() as i32)
}

/// Shorthand for allocating a special form function in the context's heap,
/// then returning an expression referencing the function.
fn wrap_special(ctx: &mut Context, item: Box<SpecialFn>) -> Expr {
    Expr::Special(ctx.allocate(Mem::Special(item)))
}

/// Shorthand for allocating a normal function in the context's heap,
/// then returning an expression referencing the function.
fn wrap_fn(ctx: &mut Context, item: Box<FuncFn>, scope: &Ac) -> Expr {
    Expr::Func(ctx.allocate(Mem::Func(item, scope.clone())))
}

/// Populates a scope with the default global values, a list of special forms
/// and functions defining the *standard library* and enabling user code to be
/// run.
pub fn populate_global(ctx: &mut Context, scope: Ac) {
    let s = scope.get().as_scope();

    s.set_atom("def", wrap_special(ctx, Box::new(special_def)));
    s.set_atom("defn", wrap_special(ctx, Box::new(special_defn)));
    s.set_atom("if", wrap_special(ctx, Box::new(special_if)));
    s.set_atom("let", wrap_special(ctx, Box::new(special_let)));

    s.set_atom("+", wrap_fn(ctx, Box::new(eval_add), &scope));
    s.set_atom("-", wrap_fn(ctx, Box::new(eval_sub), &scope));
    s.set_atom("*", wrap_fn(ctx, Box::new(eval_mul), &scope));
    s.set_atom("/", wrap_fn(ctx, Box::new(eval_div), &scope));
    s.set_atom("<<", wrap_fn(ctx, Box::new(eval_bitshiftleft), &scope));
    s.set_atom("=", wrap_fn(ctx, Box::new(eval_eq), &scope));
    s.set_atom("<", wrap_fn(ctx, Box::new(eval_le), &scope));
    s.set_atom("vec", wrap_fn(ctx, Box::new(eval_vec), &scope));
    s.set_atom("index", wrap_fn(ctx, Box::new(eval_index), &scope));
    s.set_atom("first", wrap_fn(ctx, Box::new(eval_first), &scope));
    s.set_atom("rest", wrap_fn(ctx, Box::new(eval_rest), &scope));
    s.set_atom("null?", wrap_fn(ctx, Box::new(eval_nullq), &scope));
    s.set_atom("println", wrap_fn(ctx, Box::new(eval_println), &scope));
    s.set_atom("random", wrap_fn(ctx, Box::new(eval_random), &scope));
    s.set_atom("len", wrap_fn(ctx, Box::new(eval_len), &scope));
}
