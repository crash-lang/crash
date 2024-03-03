
pub use crate::rule::LexingRule;
use crate::token::{Position, Token};
use crate::token::TokenType::{Eof, Whitespace};

pub struct Lexer {
    rules: Vec<LexingRule>
}

impl Lexer {

    pub fn new(rules: Vec<LexingRule>) -> Lexer {
        Self { rules }
    }

    pub fn tokenize(&mut self, mut input: &str, skip_whitespace: bool) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut current_column = 1;
        let mut current_line = 1;

        while !input.is_empty() {
            let mut matched_length = 0;
            let mut matched_type = None;

            for rule in self.rules.iter() {
                let pattern = rule.patterns();

                for pattern in pattern {
                    if let Some(matcher) = pattern.find(input) {
                        if matcher.start() == 0 {
                            let length = matcher.end();

                            if length > matched_length {
                                matched_length = length;
                                matched_type = Some(rule.typ());
                            }
                        }
                    }
                }
            }

            if matched_length > 0 {
                let substring = &input[..matched_length];

                let token = Token::new(
                    matched_type.unwrap(),
                    String::from(substring),
                    Position::new(current_line, current_column)
                );

                tokens.push(token);

                for c in substring.chars() {
                    if c == '\n' {
                        current_line += 1;
                        current_column = 1;
                        continue;
                    }
                    current_column += 1;
                }

                input = &input[matched_length..];
                continue;
            }

            println!("Unknown token at {:?}:{:?}", current_line, current_column);
            std::process::exit(1);
        }

        if skip_whitespace {
            tokens.retain(|token| token.tok_type() != Whitespace);
        }

        tokens.push(Token::eof());

        tokens
    }
}