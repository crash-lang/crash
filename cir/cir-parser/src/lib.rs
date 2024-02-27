mod parser;
mod includes;
mod files;

use std::fs::File;
use crash_ast::AST;
use crate::parser::Parser;

pub fn build_ast(file: File) -> AST {
    Parser::parse(file).to_ast()
}