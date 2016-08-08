extern crate ctrie;
extern crate rand;
extern crate strfmt;
extern crate uuid;

pub mod ast;
pub mod global;
pub mod lisp;
pub mod rc;
pub mod scope;
pub mod values;

pub use rc as ac;
use global::populate_global;
use scope::*;
use std::env;
use std::fs::File;
use std::io::{self, Read};
use values::*;

/// Program entry point.
fn main() {
    run().expect("Runtime code failed with error.");
}

/// Program execution that is able to return a standard io::Result<..> object.
fn run() -> io::Result<()> {
    // Read in first argument as file.
    let content_path = env::args().nth(1).unwrap();
    let mut f = try!(File::open(content_path));
    let mut content = String::new();
    try!(f.read_to_string(&mut content));

    // Parse AST.
    let ast = lisp::parse_Exprs(&content).unwrap();

    // Create new context and scope, and populate it with default functions.
    let mut ctx = Context::new();
    let s = Scope::new(&mut ctx, None);
    populate_global(&mut ctx, s.clone());

    // Evaluate file as a sequence of expressions.
    let mut res = Expr::Null;
    let exprs: Vec<Expr> = ast.iter().map(|x| Expr::from_ast(&mut ctx, x)).collect();
    for statement in exprs {
        res = ctx.eval(s.clone(), statement);
    }

    // Ignore final return value.
    let _ = res;

    Ok(())
}
