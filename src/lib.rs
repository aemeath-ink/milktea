//! Stub modules — being filled in from the prototype branch.
//! These compile so the crate isn't broken while I migrate.

use crate::{ast::*, MilkError};

pub mod lexer {
    use super::MilkError;
    pub fn lex(_src: &str) -> Result<Vec<()>, MilkError> {
        Err(MilkError::Parse("lexer not yet ported".into()))
    }
}

pub mod parser {
    use super::*;
    pub fn parse(_toks: &[()]) -> Result<Expr, MilkError> {
        Err(MilkError::Parse("parser not yet ported".into()))
    }
}

pub mod typing {
    use super::*;
    use std::collections::HashMap;
    #[derive(Default)]
    pub struct Env(pub HashMap<String, Type>);
    pub fn infer(_e: &Expr, _env: &mut Env) -> Result<Type, MilkError> {
        Err(MilkError::Type("type checker not yet ported".into()))
    }
}

pub mod eval {
    use super::*;
    use std::collections::HashMap;
    #[derive(Default)]
    pub struct Env(pub HashMap<String, Value>);
    #[derive(Debug, Clone)]
    pub enum Value { Int(i64), Bool(bool), Str(String), Closure }
    pub fn eval(_e: &Expr, _env: &mut Env) -> Result<Value, MilkError> {
        Err(MilkError::Runtime("evaluator not yet ported".into()))
    }
}

pub mod pretty {
    use super::*;
    pub fn ty(_t: &Type) -> String { "?".into() }
    pub fn value(_v: &eval::Value) -> String { "?".into() }
    pub fn err(e: &MilkError) -> String { format!("{:?}", e) }
}
