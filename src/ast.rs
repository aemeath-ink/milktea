//! The AST. Kept very small on purpose. If you add a constructor here, you
//! must update `typing.rs`, `eval.rs`, `parser.rs`, and `pretty.rs` — that's
//! the whole point of the type, to force you to think four times.

use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expr {
    Var(String),
    Lam { param: String, ty: Type, body: Rc<Expr> },
    App { f: Rc<Expr>, x: Rc<Expr> },
    Let { name: String, value: Rc<Expr>, body: Rc<Expr> },
    If  { cond: Rc<Expr>, then_: Rc<Expr>, else_: Rc<Expr> },
    LitInt(i64),
    LitBool(bool),
    LitStr(String),
    BinOp(BinOp, Rc<Expr>, Rc<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp { Add, Sub, Mul, Eq }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Str,
    Arrow(Box<Type>, Box<Type>),
}

impl Type {
    pub fn arrow(a: Type, b: Type) -> Type { Type::Arrow(Box::new(a), Box::new(b)) }
}
