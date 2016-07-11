pub mod lisp; // synthesized by LALRPOP
pub mod ast;

fn main() {
    println!("{:?}", lisp::parse_Exprs("(+ (* 22 44) 66)").unwrap());
}
