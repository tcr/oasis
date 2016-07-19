use ast::*;
use alloc::*;
use std::fmt;
use std::cell::{RefCell, Ref, RefMut, BorrowState};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct FuncFnId(pub String);

pub type FuncFn = Fn(&mut Context, Vec<Expr>) -> Expr;
pub type SpecialFn = Fn(&mut Context, Alloc, Vec<Expr>) -> Expr;

pub struct FuncInner {
    pub body: Box<FuncFn>,
    pub statements: Vec<Expr>,
    pub scope: Alloc,
}

pub enum GcMem {
    ListMem(Vec<Expr>),
    FuncMem(FuncInner),
    SpecialMem(Box<SpecialFn>),
    ScopeMem(Scope),
    Deallocated,
}

impl fmt::Debug for GcMem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GcMem::ListMem(..) => write!(f, "ListMem({:p})", self),
            &GcMem::FuncMem(..) => write!(f, "FuncMem({:p})", self),
            &GcMem::SpecialMem(..) => write!(f, "SpecialMem({:p})", self),
            &GcMem::ScopeMem(..) => write!(f, "ScopeMem({:p})", self),
            &GcMem::Deallocated => write!(f, "**DEALLOCATED**({:p})", self),
        }
    }
}

impl GcMem {
    pub fn as_list(&self) -> &Vec<Expr> {
        match self {
            &GcMem::ListMem(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_list_mut(&mut self) -> &mut Vec<Expr> {
        match self {
            &mut GcMem::ListMem(ref mut inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_func(&self) -> &FuncInner {
        match self {
            &GcMem::FuncMem(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_special(&self) -> &Box<SpecialFn> {
        match self {
            &GcMem::SpecialMem(ref inner) => inner,
            _ => unimplemented!(),
        }
    }

    pub fn as_scope(&mut self) -> &mut Scope {
        match self {
            &mut GcMem::ScopeMem(ref mut inner) => inner,
            _ => panic!("Cannot dereference {:?}", self),
        }
    }

    pub fn wrap_fn(target: Box<FuncFn>, scope: Alloc) -> GcMem {
        GcMem::FuncMem(FuncInner {
            body: target,
            scope: scope,
            statements: vec![],
        })
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Expr {
    Int(i32),
    Atom(String),
    Str(String),
    Null,
    TailCall(FuncFnId, Vec<Expr>),
    SExpr(Alloc),
    Func(Alloc),
    Special(Alloc),
}

impl Expr {
    pub fn from_ast(ctx: &mut Context, ast: &Ast) -> Expr {
        match ast {
            &Ast::Int(value) => Expr::Int(value),
            &Ast::Atom(ref value) => Expr::Atom(value.clone()),
            &Ast::SExpr(ref value) => {
                let exprs: Vec<Expr> = value.iter().map(|x| {
                    Expr::from_ast(ctx, x)
                }).collect();
                Expr::SExpr(alloc!(ctx, GcMem::ListMem(exprs)))
            }
            &Ast::Str(ref value) => Expr::Str(value.clone()),
            &Ast::Null => Expr::Null,
        }
    }

    pub fn new_atom(key: &str) -> Expr {
        Expr::Atom(key.to_owned())
    }

    pub fn as_vec<'a>(&'a self) -> Ref<'a, Vec<Expr>> {
        match self {
            &Expr::SExpr(ref inner) => {
                Ref::map(inner.borrow(), |x| {
                    x.as_list()
                })
            }
            _ => unreachable!(),
        }
    }

    pub fn as_vec_mut<'a>(&'a mut self) -> RefMut<'a, Vec<Expr>> {
        match self {
            &mut Expr::SExpr(ref mut inner) => {
                RefMut::map(inner.borrow_mut(), |x| {
                    x.as_list_mut()
                })
            }
            _ => unreachable!(),
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            &Expr::Int(0) | &Expr::Null => false,
            _ => true,
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            &Expr::Int(value) => value,
            _ => 0,
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            &Expr::Str(ref value) => value.clone(),
            &Expr::Int(value) => format!("{}", value),
            rest => format!("{:?}", rest),
        }
    }

    pub fn get_mem(&self) -> Option<&Alloc> {
        match self {
            &Expr::SExpr(ref inner) => Some(inner),
            &Expr::Func(ref inner) => Some(inner),
            &Expr::Special(ref inner) => Some(inner),
            _ => None,
        }
    }
}

pub struct Context {
    pub callstack: Vec<(FuncFnId, bool)>,
    pub alloc: AllocArena,
    pub roots: Vec<Alloc>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            callstack: vec![],
            alloc: AllocArena::new(),
            roots: vec![],
        }
    }

    pub fn pin(&mut self, item: AllocInterior) -> Alloc {
        self.alloc.pin(item)
    }

    pub fn mark_expr(value: &mut Expr) {
        match value {
            &mut Expr::Func(ref mut inner) => {
                if !inner.marked {
                    //println!("fn");
                    Context::mark(inner);
                }
            }
            &mut Expr::Special(ref mut inner) => {
                if !inner.marked {
                    //println!("special");
                    Context::mark(inner);
                }
            }
            &mut Expr::SExpr(ref mut inner) => {
                if !inner.marked {
                    Context::mark(inner);
                }
            }
            _ => {
                //println!("???");
            }
        }
    }

    pub fn mark(value: &mut Alloc) {
        //println!("marking start... {:?}", value);
        value.marked = true;

        if value.borrow_state() != BorrowState::Unused {
            //println!("*** active borrow state on mem, ignoring: {:?}", value.borrow_state())
        } else {
            match *value.borrow_mut() {
                GcMem::ScopeMem(ref mut inner) => {
                    //println!("marking scope: {:?}", value);
                    for (_, value) in &mut inner.scope {
                        Context::mark_expr(&mut *value.borrow_mut());
                    }
                    if let Some(ref mut parent) = inner.parent {
                        //println!("parent");
                        if !parent.marked {
                            Context::mark(parent);
                        }
                        //println!("done parent");
                    }
                }
                GcMem::ListMem(ref mut inner) => {
                    for value in inner.iter_mut() {
                        Context::mark_expr(value);
                    }
                }
                GcMem::FuncMem(ref mut inner) => {
                    Context::mark(&mut inner.scope);
                    for value in inner.statements.iter_mut() {
                        Context::mark_expr(value);
                    }
                }
                _ => { }
            }
        }
    }

    pub fn mark_roots(&mut self) {
        for scope in &mut self.roots {
            if !scope.marked {
                Context::mark(scope);
            }
        }
    }
}

pub fn funcfn_id(closure: &Alloc) -> FuncFnId {
    FuncFnId(closure.id())
}

pub struct Scope {
    parent: Option<Alloc>,
    pub scope: HashMap<Expr, RefCell<Expr>>,
}

impl Scope {
    pub fn new(ctx: &mut Context, parent: Option<Alloc>) -> Alloc {
        alloc!(ctx, GcMem::ScopeMem(Scope {
            parent: parent,
            scope: HashMap::new(),
        }))
    }

    pub fn set(&mut self, key: Expr, value: Expr) {
        self.scope.insert(key, RefCell::new(value));
    }

    pub fn set_atom(&mut self, key: &str, value: Expr) {
        self.scope.insert(Expr::Atom(key.to_owned()), RefCell::new(value));
    }

    pub fn lookup<F, T>(&mut self, key: &Expr, mut inner: F) -> Option<T>
        where F: FnMut(Option<&mut Expr>) -> T
    {
        match self.scope.get(key) {
            Some(ref value) => Some(inner(Some(&mut *value.borrow_mut()))),
            None => {
                match self.parent {
                    Some(ref parent) => {
                        parent.borrow_mut().as_scope().lookup(key, inner)
                    }
                    None => None,
                }
            }
        }
    }
}

pub fn eval_expr(ctx: &mut Context, scope: Alloc, x: Expr, args: Vec<Expr>) -> Expr {
    use self::Expr::*;

    match x {
        Atom(..) => {
            let (func, special): (Option<AllocRef<_>>, Option<AllocRef<_>>) = scope
                .borrow_mut()
                .as_scope()
                .lookup(&x, |value| {
                    match value {
                        Some(&mut Expr::Func(ref mut func)) => {
                            Context::mark(func);
                            (Some(func.clone()), None)
                        }
                        Some(&mut Expr::Special(ref func)) => {
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

            ctx.roots.push(scope.clone());

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
                let call = func.borrow();
                let call = call.as_func();
                let call = &call.body;
                call(ctx, args)
            } else if let Some(special) = special {
                let call = special.borrow();
                let call = call.as_special();
                call(ctx, scope, args)
            } else {
                Expr::Null
            };

            ctx.roots.pop();
            ret
        }
        _ => {
            panic!("Attempted to evaluate non-atom: {:?}", x);
        }
    }
}

pub fn eval(ctx: &mut Context, scope: Alloc, expr: Expr) -> Expr {
    match expr {
        Expr::SExpr(args) => {
            let mut args: Vec<Expr> = {
                args.borrow_mut().as_list().clone()
            };
            let term = args.remove(0);
            eval_expr(ctx, scope, term, args)
        }
        Expr::Atom(..) => {
            scope.borrow_mut()
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
