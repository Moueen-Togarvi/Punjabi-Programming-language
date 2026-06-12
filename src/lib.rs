pub mod ast;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod value;

use crate::error::PunjabiError;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::token::Token;

pub fn tokenize(source: &str) -> Result<Vec<Token>, PunjabiError> {
    Lexer::new(source).scan_tokens()
}

pub fn parse(source: &str) -> Result<Vec<ast::Stmt>, PunjabiError> {
    let tokens = tokenize(source)?;
    Parser::new(tokens).parse()
}

pub fn run_source(source: &str) -> Result<Vec<String>, PunjabiError> {
    let statements = parse(source)?;
    let mut interpreter = Interpreter::new();
    interpreter.run(&statements)
}
