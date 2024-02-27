pub use {
    crate::position::Position,
    crate::position::TokenPosition,
    crate::token::*,
    crate::rule::LexingRule
};

mod position;
mod token;
mod rule;
mod macros;

pub struct Lexer {
    rules: Vec<LexingRule>
}

impl Lexer {
    pub fn new(rules: Vec<LexingRule>) -> Self {
        Self { rules }
    }

    pub fn tokenize<'a>(&self, mut input: String) -> Result<Vec<Token<'a>>, String> {
        let mut tokens = Vec::new();

        let mut current_column = 1;
        let mut current_line = 1;

        while !input.is_empty() {
            let mut matched_length = 0;
            let mut matched_type = None;

            for rule in self.rules.iter() {
                let pattern = rule.patterns();

                for pattern in pattern {
                    if let Some(matcher) = pattern.find(&input) {
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
                    *matched_type.unwrap(),
                    String::from(substring),
                    TokenPosition::new(
                        Position::new(current_line, current_column),
                        Position::new(current_line, current_column + (substring.len() as i32))
                    )
                );

                if !token.tok_type().skipped() {
                    tokens.push(token);
                }

                for c in substring.chars() {
                    if c == '\n' {
                        current_line += 1;
                        current_column = 1;
                        continue;
                    }
                    current_column += 1;
                }

                input = input[matched_length..].to_string();
                continue;
            }

            return Err(format!("Unknown token at {:?}:{:?}", current_line, current_column));
        }

        tokens.push(Token::eof());

        Ok(tokens)
    }
}