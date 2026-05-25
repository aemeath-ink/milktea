//! milktea — a small typed lambda calculus interpreter.
//!
//! Currently the binary is a stub; the language is being moved here piece by
//! piece from a private prototype branch. The REPL works for the basic core
//! (variables, application, lambda, `let`, `if`) and gives sensible type
//! errors. Integer arithmetic and `String` literals are next.

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

mod ast;
mod lexer;
mod parser;
mod typing;
mod eval;
mod pretty;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("milktea 0.1.0 — typed λ-calculus, REPL. Ctrl-D to exit.");
    let mut rl = DefaultEditor::new()?;
    let mut env = eval::Env::default();
    let mut tenv = typing::Env::default();

    loop {
        match rl.readline("λ> ") {
            Ok(line) => {
                if line.trim().is_empty() { continue; }
                rl.add_history_entry(line.as_str())?;
                match run(&line, &mut env, &mut tenv) {
                    Ok(s)  => println!("{s}"),
                    Err(e) => eprintln!("{}", pretty::err(&e)),
                }
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}

fn run(src: &str, env: &mut eval::Env, tenv: &mut typing::Env) -> Result<String, MilkError> {
    let toks  = lexer::lex(src)?;
    let ast   = parser::parse(&toks)?;
    let ty    = typing::infer(&ast, tenv)?;
    let value = eval::eval(&ast, env)?;
    Ok(format!("{} : {}", pretty::value(&value), pretty::ty(&ty)))
}

#[derive(Debug)]
pub enum MilkError {
    Lex(String),
    Parse(String),
    Type(String),
    Runtime(String),
}
