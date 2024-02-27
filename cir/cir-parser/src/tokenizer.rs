use crash_lexer::*;
use crate::token::TokenTypes::*;

pub fn tokenize(content: String) -> Result<Vec<Token<'static>>, String> {
    Lexer::new(build_rules()).tokenize(content)
}

fn build_rules() -> Vec<LexingRule> {
    let mut rules: Vec<LexingRule> = Vec::new();

    // Whitespace
    let whitespace_type = Whitespace.as_type();

    add_regex_rule!(rules, &whitespace_type, "[ \t\r\n]+");
    add_regex_rule!(rules, &whitespace_type, "//[^\r\n]*");
    add_multi_line_rule!(rules, &whitespace_type, "/*", "*/");

    // Keywords
    add_rule!(rules, &Struct.as_type(), "struct");
    add_rule!(rules, &Fn.as_type(), "fn");

    // Symbols
    add_rule!(rules, &Semicolon.as_type(), ";");
    add_rule!(rules, &Comma.as_type(), ",");
    add_rule!(rules, &OpenCurlyBrace.as_type(), "{");
    add_rule!(rules, &CloseCurlyBrace.as_type(), "}");
    add_rule!(rules, &OpenBrace.as_type(), "(");
    add_rule!(rules, &CloseBrace.as_type(), ")");
    add_rule!(rules, &OpenSquareBrace.as_type(), "[");
    add_rule!(rules, &CloseSquareBrace.as_type(), "]");

    // Instructions
    add_rule!(rules, &Move.as_type(), "mov");
    add_rule!(rules, &Init.as_type(), "init");
    add_rule!(rules, &Push.as_type(), "push");
    add_rule!(rules, &Pop.as_type(), "pop");
    add_rule!(rules, &Call.as_type(), "call");
    add_rule!(rules, &Ret.as_type(), "ret");

    // Literals
    let boolean_literal = BooleanLiteral.as_type();
    add_rule!(rules, &boolean_literal, "true");
    add_rule!(rules, &boolean_literal, "false");

    add_multi_line_rule!(rules, &StringLiteral.as_type(), "\"", "\"");
    add_multi_line_rule!(rules, &CharLiteral.as_type(), "\'", "\'");

    let integer_literal = IntegerLiteral.as_type();

    add_regex_rule!(rules, &integer_literal, "b[01_]*");
    add_regex_rule!(rules, &integer_literal, "o[0-7_]*");
    add_regex_rule!(rules, &integer_literal, "[0-9_]*");
    add_regex_rule!(rules, &integer_literal, "#[0-9a-fA-F_]*");

    add_regex_rule!(rules, &FloatLiteral.as_type(), "[0-9_]*\\.[0-9_]*");


    rules
}