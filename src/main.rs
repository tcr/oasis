pub mod lisp; // synthesized by LALRPOP
pub mod ast;

use ast::*;

fn eval_add(a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a + b,
        _ => 0
    })
}

fn eval_sub(a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a - b,
        _ => 0
    })
}

fn eval_mul(a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a * b,
        _ => 0
    })
}

fn eval_div(a: Expr, b: Expr) -> Expr {
    Expr::Int(match (a, b) {
        (Expr::Int(a), Expr::Int(b)) => a / b,
        _ => 0
    })
}

fn eval_expr(x: Expr, args: Vec<Box<Expr>>) -> Expr {
    use ast::Expr::*;
    let mut args: Vec<Expr> = args.into_iter().map(|x| x.eval(eval_expr)).collect();
    match x {
        Atom(kind) => match kind.as_ref() {
            "+" => eval_add(args.remove(0), args.remove(0)),
            "-" => eval_sub(args.remove(0), args.remove(0)),
            "*" => eval_mul(args.remove(0), args.remove(0)),
            "/" => eval_div(args.remove(0), args.remove(0)),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn main() {
    let mut parse = lisp::parse_Exprs("(+ (* 22 44) 66)").unwrap();

    let res = parse.remove(0).eval(eval_expr);

    println!("{:?}", res);
}
