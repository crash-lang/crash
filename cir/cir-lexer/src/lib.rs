pub use crate::token::*;

use crate::lexer::Lexer;
use crate::rule::LexingRule;
use crate::TokenType::*;

mod token;
mod lexer;
mod position;
mod rule;
mod macros;

pub fn tokenize(content: &str) -> Vec<Token> {
    Lexer::new(build_rules()).tokenize(content, true)
}

fn build_rules() -> Vec<LexingRule> {
    let mut rules: Vec<LexingRule> = Vec::new();

    add_regex_rule!(rules, Whitespace, "[ \t\r\n]+");
    add_regex_rule!(rules, Whitespace, "//[^\\r\\n]*");
    add_multi_line_rule!(rules, Whitespace, "/*", "*/");

    add_rule!(rules, Include, "include");
    add_rule!(rules, Const, "const");
    add_rule!(rules, Let, "let");
    add_rule!(rules, Struct, "struct");
    add_rule!(rules, Impl, "impl");
    add_rule!(rules, Trait, "trait");
    add_rule!(rules, For, "for");
    add_rule!(rules, Enum, "enum");
    add_rule!(rules, Fn, "fn");
    add_rule!(rules, Return, "return");
    add_rule!(rules, Match, "match");
    add_rule!(rules, If, "if");
    add_rule!(rules, Break, "break");
    add_rule!(rules, Continue, "continue");
    add_rule!(rules, Loop, "loop");

    add_rule!(rules, Equals, "==");
    add_rule!(rules, Assign, "=");

    add_rule!(rules, BooleanLiteral, "true");
    add_rule!(rules, BooleanLiteral, "false");

    add_multi_line_rule!(rules, StringLiteral, "\"", "\"");
    add_multi_line_rule!(rules, CharLiteral, "\'", "\'");

    add_regex_rule!(rules, IntegerLiteral, "b[01_]*");
    add_regex_rule!(rules, IntegerLiteral, "o[0-7_]*");
    add_regex_rule!(rules, IntegerLiteral, "[0-9_]*");
    add_regex_rule!(rules, IntegerLiteral, "#[0-9a-fA-F_]*");

    add_regex_rule!(rules, FloatLiteral, "[0-9_]*\\.[0-9_]*");
    
    add_regex_rule!(rules, Identifier, "[a-zA-Z]*");
    
    add_regex_rule!(rules, PathLiteral, "[a-zA-Z.:/\\0-9]*");

    rules
}

