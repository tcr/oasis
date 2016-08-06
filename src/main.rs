// TODO https://github.com/ivanjovanovic/sicp/blob/master/2.3/2.3-binary-trees.scm
// TODO http://www.stefankrause.net/wp/?p=14

extern crate ctrie;
extern crate rand;
extern crate strfmt;

pub mod alloc;
pub mod ast;
pub mod cvec;
//pub mod gc;
//pub mod gc_collector;
pub mod lisp;
pub mod scope;
pub mod types;
pub mod rc_arena;

//use gc::*;
use rand::Rng;
use scope::*;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::mem;
use strfmt::strfmt;
use types::OVec;
use rc_arena::{AllocOut};

fn special_def(ctx: &mut Context, scope: AllocOut, mut args: Vec<Expr>) -> Expr {
    let key = args.remove(0);
    let value = eval(ctx, scope.clone(), args.remove(0));
    scope.get().as_scope().set(key, value);
    Expr::Null
}

fn special_defn(ctx: &mut Context, scope: AllocOut, mut args: Vec<Expr>) -> Expr {
    use std::rc::Rc;
    use std::sync::RwLock;

    let key = args.remove(0);
    let names: Vec<Expr> = if let Expr::List(content) = args.remove(0) {
        content
    } else {
        vec![]
    };

    let parent_scope = scope.clone();
    let inner_ref: Rc<RwLock<Option<FuncFnId>>> = Rc::new(RwLock::new(None));
    let outer_ref = inner_ref.clone();

    //let debug_key = key.clone();

    let content = args; // TODO ensure purity
    let closure: AllocOut = ctx.allocate(Mem::FuncMem(FuncInner {
        scope: scope.clone(),
        body: Box::new(move |ctx: &mut Context, mut args: Vec<Expr>| {
            //println!("called fn (key {:?})", debug_key);

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
                .position(|x| x.0 == fn_id).is_some() {
                // Return early with evaluated arguments.
                return Expr::TailCall(fn_id, args);
            }

            // Temporarily pin all arguments
            // TODO make this temporary
            //for item in &args {
            //    if let Some(alloc) = item.get_mem() {
            //        ctx.roots.push(alloc.clone());
            //    }
            //}

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

                // Hold on for dear life. GC
                // TODO better to attach to current scope or something?
                //s2.borrow_mut().as_scope().set_atom("__scope", Expr::Vec(alloc!(ctx, Mem::VecMem(OVec::new_from(content.clone())))));

                let len = content.len();
                for (i, statement) in content.iter().enumerate() {
                    // When we are evaluating the last statement, change our Context
                    // to indicate we are in terminal position
                    if i + 1 == len {
                        ctx.callstack[pos].1 = true;
                    }

                    //s2.borrow_mut().as_scope().lookup(&Expr::Atom("inner".to_owned()), |x| {
                    //    println!("inner: {:?}", x);
                    //});
                    //println!("scope: {:?}", s2.clone());
                    res = eval(ctx, s2.clone(), statement.clone());
                    //s2.borrow_mut().as_scope().lookup(&Expr::Atom("inner".to_owned()), |x| {
                    //    println!("inner2: {:?}", x);
                    //});
                }

                //GC_DETACH
                ctx.state.roots.pop();

                // Evaluate tail call expressions if they match this function.
                if match res {
                    Expr::TailCall(ref inner_fn_id, _) => {
                        *inner_fn_id == fn_id
                    },
                    _ => false,
                } {
                    if let Expr::TailCall(_, inner_args) = mem::replace(&mut res, Expr::Null) {
                        // Temporarily pin all arguments
                        // TODO make this temporary
                        //for item in &inner_args {
                        //    if let Some(alloc) = item.get_mem() {
                        //        ctx.roots.push(alloc.clone());
                        //    }
                        //}

                        args = inner_args;
                        continue;
                    }
                }

                break;
            }

            ctx.callstack.pop();
            res
        }),
    }));

    // Store unique closure ID.
    *outer_ref.write().unwrap() = Some(funcfn_id(&closure));
    scope.get().as_scope().set(key, Expr::Func(closure.clone()));

    Expr::Null
}

fn special_if(ctx: &mut Context, scope: AllocOut, mut args: Vec<Expr>) -> Expr {
    let if_val = args.remove(0);
    let then_val = args.remove(0);
    let else_val = args.remove(0);

    if eval(ctx, scope.clone(), if_val).as_bool() {
        eval(ctx, scope.clone(), then_val)
    } else {
        eval(ctx, scope.clone(), else_val)
    }
}

fn special_let(ctx: &mut Context, scope: AllocOut, mut args: Vec<Expr>) -> Expr {
    let bindings = if let Expr::List(content) = args.remove(0) {
        content
    } else {
        vec![]
    };
    let content = args;

    let s2 = Scope::new(ctx, Some(scope.clone()));
    for win in bindings[..].chunks(2) {
        let item = win[0].clone();
        let value = win[1].clone();
        let value = eval(ctx, s2.clone(), value);
        s2.get().as_scope().set(item, value);
    }

    let mut res = Expr::Null;
    for statement in content.iter() {
        res = eval(ctx, s2.clone(), statement.clone());
    }

    res
}

fn eval_add(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(args.iter()
        .map(|x| x.as_int())
        .fold(0, |sum, val| sum + val))
}

fn eval_sub(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], args.iter().nth(1)) {
        (&Expr::Int(a), Some(&Expr::Int(b))) => a - b,
        (&Expr::Int(a), None) => -a,
        _ => 0,
    })
}

fn eval_mul(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a * b,
        _ => 0,
    })
}

fn eval_div(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a / b,
        _ => 0,
    })
}

fn eval_bitshiftleft(_: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Int(match (&args[0], &args[1]) {
        (&Expr::Int(a), &Expr::Int(b)) => a << b,
        _ => 0,
    })
}

fn eval_eq(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let a = args.remove(0);
    let b = args.remove(0);

    if a == b {
        Expr::Int(1)
    } else {
        Expr::Int(0)
    }
}

fn eval_le(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let a = args.remove(0);
    let b = args.remove(0);

    if a.as_int() < b.as_int() {
        Expr::Int(1)
    } else {
        Expr::Int(0)
    }
}

fn eval_vec(ctx: &mut Context, args: Vec<Expr>) -> Expr {
    Expr::Vec(ctx.allocate(Mem::VecMem(OVec::new_from(args))))
}

fn eval_index(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let value = args.remove(0);
    let key = args.remove(0);

    let value_vec = value.as_vec();
    value_vec.get((key.as_int() as usize), |value| {
        value.clone()
    }).unwrap_or(Expr::Null)
}

fn eval_first(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let value = args.remove(0);

    let value_vec = value.as_vec();
    value_vec.get(0, |value| {
        value.clone()
    }).unwrap_or(Expr::Null)
}

fn eval_rest(ctx: &mut Context, mut args: Vec<Expr>) -> Expr {
    args.remove(0);
    Expr::Vec(ctx.allocate(Mem::VecMem(OVec::new_from(args))))
}

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

//fn eval_concat(_: &mut Context, mut args: Vec<Expr>) -> Expr {
//    let list = args.remove(0);
//    let add = args.remove(0);
//
//    list.as_vec().push(add);
//    list
//}

fn eval_random(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let n = args.remove(0);

    let mut rng = rand::thread_rng();
    Expr::Int(rng.gen_range(0, n.as_int()))
}

fn eval_len(_: &mut Context, mut args: Vec<Expr>) -> Expr {
    let list = args.remove(0);

    let vec = list.as_vec();
    Expr::Int(vec.len() as i32)
}

fn main() {
    run().expect("Runtime code failed with error.");
}

fn run() -> io::Result<()> {
    let content_path = env::args().nth(1).unwrap();
    let mut f = try!(File::open(content_path));
    let mut content = String::new();
    try!(f.read_to_string(&mut content));

    let ast = lisp::parse_Exprs(&content).unwrap();

    let mut ctx = Context::new();
    let s = Scope::new(&mut ctx, None);

    {
        let s2 = s.clone();
        let s = s.get().as_scope();

        s.set_atom("def", Expr::Special(ctx.allocate(Mem::SpecialMem(Box::new(special_def)))));
        s.set_atom("defn", Expr::Special(ctx.allocate(Mem::SpecialMem(Box::new(special_defn)))));
        s.set_atom("if", Expr::Special(ctx.allocate(Mem::SpecialMem(Box::new(special_if)))));
        s.set_atom("let", Expr::Special(ctx.allocate(Mem::SpecialMem(Box::new(special_let)))));

        s.set_atom("+", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_add), s2.clone()))));
        s.set_atom("-", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_sub), s2.clone()))));
        s.set_atom("*", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_mul), s2.clone()))));
        s.set_atom("/", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_div), s2.clone()))));
        s.set_atom("<<", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_bitshiftleft), s2.clone()))));
        s.set_atom("=", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_eq), s2.clone()))));
        s.set_atom("<", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_le), s2.clone()))));
        s.set_atom("vec", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_vec), s2.clone()))));
        s.set_atom("index", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_index), s2.clone()))));
        s.set_atom("first", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_first), s2.clone()))));
        s.set_atom("rest", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_rest), s2.clone()))));
        s.set_atom("null?", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_nullq), s2.clone()))));
        s.set_atom("println", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_println), s2.clone()))));
        //s.set_atom("concat", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_concat), s2.clone()))));
        s.set_atom("random", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_random), s2.clone()))));
        s.set_atom("len", Expr::Func(ctx.allocate(Mem::wrap_fn(Box::new(eval_len), s2.clone()))));
    }

    let mut res = Expr::Null;
    let exprs: Vec<Expr> = ast.iter().map(|x| Expr::from_ast(&mut ctx, x)).collect();
    for statement in exprs {
        res = eval(&mut ctx, s.clone(), statement);
    }

    // Ignore final return value.
    let _ = res;

    Ok(())
}
