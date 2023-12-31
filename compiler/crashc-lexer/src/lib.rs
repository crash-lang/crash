/*
 * Copyright 2024 Julian Siebert
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

mod cursor;
mod token;
#[cfg(test)]
mod test;

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
            return Some(Token::new(token_kind, end_pos - start_pos, expected));
        }

        None
    }

    fn try_match_integer_type(&mut self) -> Option<Token> {
        let integer_types = [
            ("i8", PrimitiveKind::I8),
            ("i16", PrimitiveKind::I16),
            ("i32", PrimitiveKind::I32),
            ("i64", PrimitiveKind::I64),
            ("i128", PrimitiveKind::I128),
            ("u8", PrimitiveKind::U8),
            ("u16", PrimitiveKind::U16),
            ("u32", PrimitiveKind::U32),
            ("u64", PrimitiveKind::U64),
            ("u128", PrimitiveKind::U128)
        ];

        for (type_str, kind) in &integer_types {
            if let Some(token) = self.try_match_keyword(type_str, TokenKind::Primitive { kind: *kind }) {
                return Some(token);
            }
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

    fn try_match_char_literal(&mut self) -> Option<Token> {
        if self.current_char() == Some('\'') {
            let start_pos = self.pos_within_token();
            self.bump();

            let mut content = String::new();

            if let Some(char_value) = self.current_char() {
                content.push(char_value);
                self.bump();
                if self.current_char() == Some('\'') {
                    content.push('\'');
                    self.bump();
                    let end_pos = self.pos_within_token();
                    return Some(Token::new(TokenKind::Literal {
                        kind: LiteralKind::Character {
                            terminated: true,
                        },
                    }, end_pos - start_pos, &content));
                }
            }
        }

        Some(Token::new(TokenKind::Literal {
            kind: LiteralKind::Character {
                terminated: false,
            },
        }, 2, "''"))
    }

    fn try_match_string_literal(&mut self) -> Option<Token> {
        if self.current_char() == Some('"') {
            let start_pos = self.pos_within_token();
            self.bump();
            let mut content = String::new();

            while let Some(current_char) = self.current_char() {
                self.bump();
                content.push(current_char);

                if current_char == '"' {
                    let end_pos = self.pos_within_token();

                    let mut chars = content.chars();
                    chars.next_back();

                    return Some(Token::new(TokenKind::Literal {
                        kind: LiteralKind::String {
                            terminated: true,
                        },
                    }, end_pos - start_pos, chars.as_str()));
                }
            }
        }

        Some(Token::new(TokenKind::Literal {
            kind: LiteralKind::String {
                terminated: false,
            },
        }, 1, "\""))
    }

    fn try_match_number_literal(&mut self) -> Option<Token> {
        let start_pos = self.pos_within_token();
        let mut has_digit = false;
        let mut content = String::new();

        let base_prefix = match self.current_char() {
            Some('o') => {
                let next = self.next();
                if !next.is_digit(8) {
                    return None
                }
                let base_char = self.current_char().unwrap();
                self.bump();
                Some(base_char)
            }
            Some('b') | Some('#') => {
                let base_char = self.current_char().unwrap();
                self.bump();
                Some(base_char)
            }
            _ => None,
        };

        let radix;

        if let Some(prefix) = base_prefix {
            radix = match prefix {
                'o' => 8,
                'b' => 2,
                '#' => 16,
                 _ => 0
            }
        } else {
            radix = 10;
        }

        while let Some(current_char) = self.current_char() {
            if current_char == '_' {
                self.bump();
                continue;
            }
            if current_char.is_digit(radix) {
                has_digit = true;
                content.push(current_char);
                self.bump();
            } else {
                break;
            }
        }

        if !has_digit {
            return None;
        }

        let is_float = match self.current_char() {
            Some('.') => {
                content.push('.');
                self.bump();
                true
            }
            _ => false,
        };

        if is_float {
            while let Some(current_char) = self.current_char() {
                if current_char == '_' {
                    self.bump();
                    continue;
                }
                if current_char.is_digit(radix) {
                    has_digit = true;
                    content.push(current_char);
                    self.bump();
                } else {
                    break;
                }
            }
        }

        if let Some('e') | Some('E') = self.current_char() {
            content.push(self.current_char().unwrap());
            self.bump();
            while let Some(current_char) = self.current_char() {
                if current_char == '_' {
                    self.bump();
                    continue;
                }
                if current_char.is_digit(radix) {
                    has_digit = true;
                    content.push(current_char);
                    self.bump();
                } else {
                    break;
                }
            }
        }

        let prefix = match base_prefix {
            Some('b') => Some(Base::Binary),
            Some('o') => Some(Base::Octal),
            Some('#') => Some(Base::Hexadecimal),
            _ => None,
        };

        if !has_digit {
            return None;
        }

        let end_pos = self.pos_within_token();

        let kind = if let Some(base) = prefix {
            LiteralKind::Integer { base }
        } else if is_float {
            LiteralKind::Float
        } else {
            LiteralKind::Integer { base: Base::Decimal }
        };

        Some(Token::new(TokenKind::Literal { kind }, end_pos - start_pos, &content))
    }

    fn try_match_identifier_literal(&mut self) -> Option<Token> {
        let start_pos = self.pos_within_token();
        let mut content = String::new();

        while let Some(current_char) = self.current_char() {
            if current_char.is_alphanumeric() || current_char == '.' {
                content.push(current_char);
                self.bump();
            } else {
                break;
            }
        }

        let end_pos = self.pos_within_token();

        Some(Token::new(TokenKind::Identifier, end_pos - start_pos, &content))
    }

    fn skip_line_comment(&mut self) {
        self.bump();
        self.bump();
        while let Some(c) = self.current_char() {
            if c == '\n' {
                self.bump();
                break;
            }
            self.bump();
        }
    }

    fn skip_multi_line_comment(&mut self) {
        self.bump();
        self.bump();

        while let Some(c) = self.current_char() {
            if c == '*' && self.next() == '/' {
                self.bump();
                self.bump();
                break;
            }
            self.bump();
        }
    }

    fn single_char_token(&mut self, kind: TokenKind, content: &str) -> Option<Token> {
        self.advance();
        Some(Token::new(kind, 1, content))
    }

    fn double_char_token(&mut self, kind1: TokenKind, kind2: TokenKind, expected: char, val1: &str, val2: &str) -> Option<Token> {
        let next_char = self.next();
        if next_char == expected {
            self.bump();
            self.bump();
            Some(Token::new(kind2, 2, val2))
        } else {
            self.advance();
            Some(Token::new(kind1, 1, val1))
        }
    }

    fn tokenize_next(&mut self) -> Option<Token> {
        self.skip_whitespace();

        if let Some(c) = self.current_char() {
            if c == '/' {
                let next = self.next();
                if next == '/' {
                    self.skip_line_comment();
                    return self.tokenize_next();
                }
                if next == '*' {
                    self.skip_multi_line_comment();
                    return self.tokenize_next();
                }
            }

            return match c {
                '"' => self.try_match_string_literal(),
                '\'' => self.try_match_char_literal(),
                '#' => self.try_match_number_literal(),
                'o' => {
                    if let Some(token) = self.try_match_number_literal() {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                '0'..='9' => self.try_match_number_literal(),
                ';' => self.single_char_token(TokenKind::Semicolon, ";"),
                '{' => self.single_char_token(TokenKind::OpenCurlyBrace, "{"),
                '}' => self.single_char_token(TokenKind::CloseCurlyBrace, "}"),
                '(' => self.single_char_token(TokenKind::OpenBrace, "("),
                ')' => self.single_char_token(TokenKind::CloseBrace, ")"),
                '[' => self.single_char_token(TokenKind::OpenSquareBrace, "["),
                ']' => self.single_char_token(TokenKind::CloseSquareBrace, "]"),
                ',' => self.single_char_token(TokenKind::Comma, ","),
                '?' => self.single_char_token(TokenKind::Question, "?"),
                ':' => self.single_char_token(TokenKind::Colon, ":"),
                '+' => self.single_char_token(TokenKind::Add, "+"),
                '-' => self.single_char_token(TokenKind::Subtract, "-"),
                '*' => self.single_char_token(TokenKind::Multiply, "*"),
                '/' => self.single_char_token(TokenKind::Divide, "/"),
                '%' => self.single_char_token(TokenKind::Modulus, "%"),

                '=' => self.double_char_token(TokenKind::Assign, TokenKind::Equals, '=', "=", "=="),
                '!' => self.double_char_token(TokenKind::Exclamation, TokenKind::NotEquals, '=', "!", "!="),
                '>' => self.double_char_token(TokenKind::Greater, TokenKind::GreaterOrEqual, '=', ">", ">="),
                '<' => self.double_char_token(TokenKind::Less, TokenKind::LessOrEqual, '=', "<", "<="),

                '|' => {
                    let next_char = self.next();
                    if next_char == '|' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(TokenKind::Or, 2, "||"));
                    }
                    return None;
                }

                '&' => {
                    let next_char = self.next();
                    if next_char == '&' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(TokenKind::Add, 2, "&&"));
                    }
                    return None;
                }

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
                    return self.try_match_identifier_literal();
                }
                's' => {
                    if let Some(token) = self.try_match_keyword("switch", TokenKind::Switch) {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                'm' => {
                    if let Some(token) = self.try_match_keyword("match", TokenKind::Match) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("mut", TokenKind::Mutable) {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                'l' => {
                    if let Some(token) = self.try_match_keyword("loop", TokenKind::Loop) {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                'f' => {
                    if let Some(token) = self.try_match_keyword("for", TokenKind::For) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_float_type() {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("false", TokenKind::Literal {
                        kind: LiteralKind::Boolean {
                            val: false
                        }
                    }) {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                'r' => {
                    if let Some(token) = self.try_match_keyword("return", TokenKind::Return) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("ret", TokenKind::Return) {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                'c' => {
                    if let Some(token) = self.try_match_keyword("class", TokenKind::Class) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_char_type() {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_keyword("continue", TokenKind::Continue) {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                'e' => {
                    if let Some(token) = self.try_match_keyword("enum", TokenKind::Enum) {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
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
                    return self.try_match_identifier_literal();
                }
                'b' => {
                    if let Some(token) = self.try_match_keyword("break", TokenKind::Break) {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_boolean_type() {
                        return Some(token);
                    }
                    if let Some(token) = self.try_match_number_literal() {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                'u' => {
                    if let Some(token) = self.try_match_integer_type() {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                't' => {
                    if let Some(token) = self.try_match_keyword("true", TokenKind::Literal {
                        kind: LiteralKind::Boolean {
                            val: true
                        }
                    }) {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                _ => {
                    return self.try_match_identifier_literal();
                }
            }
        }
        None
    }
}