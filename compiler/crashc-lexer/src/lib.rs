mod cursor;
mod token;

use std::string::String;
pub use crate::cursor::Cursor;
pub use crate::ModifierKind::*;
pub use crate::token::*;


pub fn tokenize_string(input: &str) -> Vec<Token> {
    let mut cursor = Cursor::new(input);
    let mut tokens = Vec::new();

    while !cursor.is_eof() {
        if let Some(token) = cursor.tokenize_next() {
            tokens.push(token);
            continue;
        }
        cursor.advance();
    }

    tokens
}

impl Cursor<'_> {
    fn try_match_keyword(&mut self, expected: &str, token_kind: TokenKind) -> Option<Token> {
        if self.chars().as_str().starts_with(expected) {
            let start_pos = self.pos_within_token();

            for _ in 0..expected.len() {
                self.bump();
            }

            let end_pos = self.pos_within_token();
            return Some(Token::new(token_kind, end_pos - start_pos));
        }

        None
    }

    fn try_match_integer_type(&mut self) -> Option<Token> {
        if let Some(token) = self.try_match_keyword("i8",
                                                    TokenKind::Primitive { kind: PrimitiveKind::I8 }) {
            return Some(token);
        }
        if let Some(token) = self.try_match_keyword("i16",
                                                    TokenKind::Primitive { kind: PrimitiveKind::I16 }) {
            return Some(token);
        }
        if let Some(token) = self.try_match_keyword("i32",
                                                    TokenKind::Primitive { kind: PrimitiveKind::I32 }) {
            return Some(token);
        }
        if let Some(token) = self.try_match_keyword("i64",
                                                    TokenKind::Primitive { kind: PrimitiveKind::I64 }) {
            return Some(token);
        }
        if let Some(token) = self.try_match_keyword("i128",
                                                    TokenKind::Primitive { kind: PrimitiveKind::I128 }) {
            return Some(token);
        }

        None
    }

    fn try_match_unsigned_integer_type(&mut self) -> Option<Token> {
        if let Some(token) = self.try_match_keyword("u8",
                                                    TokenKind::Primitive { kind: PrimitiveKind::U8 }) {
            return Some(token);
        }
        if let Some(token) = self.try_match_keyword("u16",
                                                    TokenKind::Primitive { kind: PrimitiveKind::U16 }) {
            return Some(token);
        }
        if let Some(token) = self.try_match_keyword("u32",
                                                    TokenKind::Primitive { kind: PrimitiveKind::U32 }) {
            return Some(token);
        }
        if let Some(token) = self.try_match_keyword("u64",
                                                    TokenKind::Primitive { kind: PrimitiveKind::U64 }) {
            return Some(token);
        }
        if let Some(token) = self.try_match_keyword("u128",
                                                    TokenKind::Primitive { kind: PrimitiveKind::U128 }) {
            return Some(token);
        }

        None
    }

    fn try_match_float_type(&mut self) -> Option<Token> {
        if let Some(token) = self.try_match_keyword("f32", TokenKind::Primitive { kind: PrimitiveKind::F32 }) {
            return Some(token);
        }
        if let Some(token) = self.try_match_keyword("f64", TokenKind::Primitive { kind: PrimitiveKind::F64 }) {
            return Some(token);
        }

        None
    }

    fn try_match_boolean_type(&mut self) -> Option<Token> {
        if let Some(token) = self.try_match_keyword("bool", TokenKind::Primitive { kind: PrimitiveKind::Bool }) {
            return Some(token);
        }

        None
    }

    fn try_match_char_type(&mut self) -> Option<Token> {
        if let Some(token) = self.try_match_keyword("char", TokenKind::Primitive { kind: PrimitiveKind::Char }) {
            return Some(token);
        }

        None
    }

    fn try_match_string_literal(&mut self) -> bool {
        let mut escaped = false;

        while let Some(current_char) = self.current_char() {
            if current_char == '\\' && !escaped {
                escaped = true;
            } else if current_char == '"' && !escaped {
                self.advance();
                return true;
            } else {
                escaped = false;
            }

            self.advance();
        }

        false
    }

    fn try_match_char_literal(&mut self) -> bool {
        return false;
    }

    fn try_match_integer_literal(&mut self) -> Option<Token> {
        let mut base = None;

        if let Some(current_char) = self.current_char() {
            if current_char == 'b' {
                base = Some(Base::Binary);
            } else if current_char == 'o' {
                base = Some(Base::Octal);
            } else if current_char == '#' {
                base = Some(Base::Hexadecimal);
            }
        }

        let mut empty = false;

        while let Some(current_char) = self.current_char() {
            if current_char == '_' && !empty {
                // skip _
            } else if !current_char.is_digit(base.map_or(10, |b| b as u32)) {
                return Some(Token::new(TokenKind::Literal {
                    kind: LiteralKind::Integer {
                        base: base.unwrap_or(Base::Decimal),
                        empty
                    }
                }, self.pos_within_token()));
            } else {
                empty = false;
            }

            self.advance();
        }

        None
    }

    fn try_match_float_literal(&mut self) -> Option<Token> {
        let mut empty_exponent = false;

        while let Some(current_char) = self.current_char() {
            if current_char == '_' && !empty_exponent {
                // skip _
            } else if current_char == '.' {
                self.advance();
                return self.try_match_decimal_part();
            } else if current_char == 'e' || current_char == 'E' {
                self.advance();
                return self.try_match_exponent_part();
            } else if !current_char.is_digit(10) {
                return Some(Token::new(TokenKind::Literal {
                    kind: LiteralKind::Float {
                        empty_exponent,
                    },
                }, self.pos_within_token()));
            } else {
                empty_exponent = false;
            }

            self.advance();
        }

        None
    }

    fn try_match_decimal_part(&mut self) -> Option<Token> {
        let mut empty = true;

        while let Some(current_char) = self.current_char() {
            if current_char == '_' && !empty {
                // skip _
            } else if !current_char.is_digit(10) {
                return Some(Token::new(TokenKind::Literal {
                    kind: LiteralKind::Float {
                        empty_exponent: true,
                    },
                }, self.pos_within_token()));
            } else {
                empty = false;
            }

            self.advance();
        }

        None
    }

    fn try_match_exponent_part(&mut self) -> Option<Token> {
        let mut empty_exponent = true;

        if let Some(current_char) = self.current_char() {
            if current_char == '+' || current_char == '-' {
                empty_exponent = false;
                self.advance();
            }
        }

        while let Some(current_char) = self.current_char() {
            if current_char == '_' && !empty_exponent {
                // skip _
            } else if !current_char.is_digit(10) {
                return Some(Token::new(TokenKind::Literal {
                    kind: LiteralKind::Float {
                        empty_exponent,
                    },
                }, self.pos_within_token()));
            } else {
                empty_exponent = false;
            }

            self.advance();
        }

        None
    }

    fn tokenize_next(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if let Some(c) = self.current_char() {
            match c {
                '"' => {
                    self.advance();

                    let terminated = self.try_match_string_literal();
                    return Some(Token::new(TokenKind::Literal {
                        kind: LiteralKind::String { terminated }
                    }, self.pos_within_token() + 2));
                }
                '\'' => {
                    self.advance();

                    let terminated = self.try_match_char_literal();
                    return Some(Token::new(TokenKind::Literal {
                        kind: LiteralKind::Character { terminated }
                    }, 3));
                }
                '0'..='9' => {
                    if let Some(token) = self.try_match_float_literal() {
                        return Some(token);
                    }
                    return self.try_match_integer_literal();
                }
                ';' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Semicolon, 1));
                }
                '{' => {
                    self.advance();
                    return Some(Token::new(TokenKind::OpenCurlyBrace, 1));
                }
                '}' => {
                    self.advance();
                    return Some(Token::new(TokenKind::CloseCurlyBrace, 1));
                }
                '(' => {
                    self.advance();
                    return Some(Token::new(TokenKind::OpenBrace, 1));
                }
                ')' => {
                    self.advance();
                    return Some(Token::new(TokenKind::CloseBrace, 1));
                }
                '[' => {
                    self.advance();
                    return Some(Token::new(TokenKind::OpenSquaredBrace, 1));
                }
                ']' => {
                    self.advance();
                    return Some(Token::new(TokenKind::CloseSquaredBrace, 1));
                }
                ',' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Comma, 1));
                }
                '?' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Question, 1));
                }
                ':' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Colon, 1));
                }

                '+' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Add, 1));
                }
                '-' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Subtract, 1));
                }
                '*' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Multiply, 1));
                }
                '/' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Divide, 1));
                }
                '%' => {
                    self.advance();
                    return Some(Token::new(TokenKind::Modulus, 1));
                }

                '=' => {
                    let next_char = self.second();
                    if next_char == '=' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(TokenKind::Equals, 2));
                    }
                    self.advance();
                    return Some(Token::new(TokenKind::Assign, 1));
                }
                '!' => {
                    let next_char = self.second();
                    if next_char == '=' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(TokenKind::NotEquals, 2));
                    }
                    self.advance();
                    return Some(Token::new(TokenKind::Exclamation, 1));
                }
                '>' => {
                    let next_char = self.second();
                    if next_char == '=' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(TokenKind::GreaterOrEqual, 2));
                    }
                    self.advance();
                    return Some(Token::new(TokenKind::Greater, 1));
                }
                '<' => {
                    let next_char = self.second();
                    if next_char == '=' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(TokenKind::LessOrEqual, 2));
                    }
                    self.advance();
                    return Some(Token::new(TokenKind::Less, 1));
                }

                '&' => {
                    let next_char = self.second();
                    if next_char == '&' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(TokenKind::And, 2));
                    }
                    self.advance();
                    return Some(Token::new(TokenKind::Copy, 1));
                }
                '|' => {
                    let next_char = self.second();
                    if next_char == '|' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(TokenKind::Or, 2));
                    }
                }

                // Keywords
                'i' => {
                    if let Some(token) = self.try_match_keyword("if", TokenKind::If) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("interface", TokenKind::Interface) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("import", TokenKind::Import) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("intern",
                                                                TokenKind::AccessModifier {
                                                                    kind: Internal
                                                                }) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_integer_type() {
                        return Some(token);
                    }
                }
                's' => {
                    return self.try_match_keyword("switch", TokenKind::Switch);
                }
                'm' => {
                    if let Some(token) = self.try_match_keyword("match", TokenKind::Match) {
                        return Some(token);
                    }
                    return self.try_match_keyword("mut", TokenKind::Mutable);
                }
                'l' => {
                    return self.try_match_keyword("loop", TokenKind::Loop);
                }
                'f' => {
                    if let Some(token) = self.try_match_keyword("for", TokenKind::For) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_float_type() {
                        return Some(token);
                    }
                    return self.try_match_keyword("false", TokenKind::Literal {
                        kind: LiteralKind::Boolean {
                            val: false
                        }
                    })
                }
                'r' => {
                    if let Some(token) = self.try_match_keyword("return", TokenKind::Return) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("ret", TokenKind::Return) {
                        return Some(token);
                    }
                }
                'c' => {
                    if let Some(token) = self.try_match_keyword("class", TokenKind::Class) {
                        return Some(token);
                    }
                    return self.try_match_char_type();
                }
                'e' => {
                    return self.try_match_keyword("enum", TokenKind::Enum);
                }
                'p' => {
                    if let Some(token) = self.try_match_keyword("public",
                                                                TokenKind::AccessModifier {
                                                                    kind: Public
                                                                }) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("protected",
                                                                TokenKind::AccessModifier {
                                                                    kind: Protected
                                                                }) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("pub",
                                                                TokenKind::AccessModifier {
                                                                    kind: Public
                                                                }) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("prot",
                                                                TokenKind::AccessModifier {
                                                                    kind: Protected
                                                                }) {
                        return Some(token);
                    }
                }
                'b' => {
                    return self.try_match_boolean_type();
                }
                'u' => {
                    return self.try_match_unsigned_integer_type();
                }
                't' => {
                    return self.try_match_keyword("true", TokenKind::Literal {
                        kind: LiteralKind::Boolean {
                            val: true
                        }
                    })
                }
                _ => {
                    return None;
                }
            }
        }
        None
    }
}