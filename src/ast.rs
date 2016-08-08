/// Oasis AST definition. This contains all of the value forms that are able
/// to be parsed from Oasis source code. To evaluate this AST, these objects
/// are converted into their matching expression (Expr) objects.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Ast {
    Atom(String),
    Int(i32),
    List(Vec<Ast>),
    Null,
    Str(String),
}
