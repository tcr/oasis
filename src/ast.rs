use std::mem;
use std::fmt;
use std::hash::{Hash, Hasher};

pub struct FnHolder {
    f: &'static Fn(Expr, Expr) -> Expr,
}

impl FnHolder {
    pub fn new(f: &'static Fn(Expr, Expr) -> Expr) -> FnHolder {
        FnHolder {
            f: f,
        }
    }

    fn id(&self) -> [usize; 2] {
        unsafe {
            mem::transmute(self.f)
        }
    }
}

impl Hash for FnHolder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in self.id().iter() {
            state.write_usize(*item)
        }
    }
}

impl PartialEq for FnHolder {
    fn eq(&self, other: &FnHolder) -> bool {
        self.id() == other.id()
    }
}

impl fmt::Debug for FnHolder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FnHolder({:X} {:X})", self.id()[0], self.id()[1])
    }
}

impl Eq for FnHolder {}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Expr {
    Int(i32),
    Atom(String),
    SExpr(Vec<Box<Expr>>),
    Func(FnHolder),
}

impl Expr {
    pub fn call(&self, a: Expr, b: Expr) -> Expr {
        match self {
            &Expr::Func(ref holder) => {
                let func = &holder.f;
                func(a, b)
            }
            _ => unreachable!(),
        }
    }
}
