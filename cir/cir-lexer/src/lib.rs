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

    add_rule!(rules, Struct, "struct");
    add_rule!(rules, Fn, "fn");
    add_rule!(rules, Include, "include");
    add_rule!(rules, Null, "null");

    add_rule!(rules, Semicolon, ";");
    add_rule!(rules, Comma, ",");
    add_rule!(rules, OpenSquareBrace, "[");
    add_rule!(rules, CloseSquareBrace, "]");
    add_rule!(rules, OpenCurlyBrace, "{");
    add_rule!(rules, CloseCurlyBrace, "}");
    add_rule!(rules, OpenBrace, "(");
    add_rule!(rules, CloseBrace, ")");

    add_rule!(rules, Move, "mov");
    add_rule!(rules, Init, "init");
    add_rule!(rules, Push, "push");
    add_rule!(rules, Pop, "pop");
    add_rule!(rules, Call, "call");
    add_rule!(rules, Ret, "ret");

    add_rule!(rules, BooleanLiteral, "true");
    add_rule!(rules, BooleanLiteral, "false");

    add_multi_line_rule!(rules, StringLiteral, "\"", "\"");
    add_multi_line_rule!(rules, CharLiteral, "\'", "\'");

    add_regex_rule!(rules, IntegerLiteral, "b[01_]*");
    add_regex_rule!(rules, IntegerLiteral, "o[0-7_]*");
    add_regex_rule!(rules, IntegerLiteral, "[0-9_]*");
    add_regex_rule!(rules, IntegerLiteral, "#[0-9a-fA-F_]*");

    add_regex_rule!(rules, FloatLiteral, "[0-9_]*\\.[0-9_]*");

    add_regex_rule!(rules, Name, "[a-zA-Z_]*");

    rules
}

