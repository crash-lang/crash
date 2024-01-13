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
use crate::Base::*;
pub use crate::cursor::Cursor;
use crate::LiteralKind::*;
pub use crate::ModifierKind::*;
use crate::PrimitiveKind::*;
pub use crate::token::*;
use crate::TokenKind::*;

#[macro_export]
macro_rules! keyword {
    ( $self:expr, $literal:expr , $kind:expr) => {
        if let Some(token) = $self.try_match_keyword($literal, $kind) {
            return Some(token);
        }
    };
}

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
                    return Some(Token::new(Literal {
                        kind: Character {
                            terminated: true,
                        },
                    }, end_pos - start_pos, &content));
                }
            }
        }

        Some(Token::new(Literal {
            kind: Character {
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

                    return Some(Token::new(Literal {
                        kind: LiteralKind::String {
                            terminated: true,
                        },
                    }, end_pos - start_pos, chars.as_str()));
                }
            }
        }

        Some(Token::new(Literal {
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
                    return None;
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

        let ret = self.check_number_literal(has_digit, content, radix);
        content = ret.0;
        has_digit = ret.1;

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
            let ret = self.check_number_literal(has_digit, content, radix);
            content = ret.0;
            has_digit = ret.1;
        }

        if let Some('e') | Some('E') = self.current_char() {
            content.push(self.current_char().unwrap());
            self.bump();
            let ret = self.check_number_literal(has_digit, content, radix);
            content = ret.0;
            has_digit = ret.1;
        }

        let prefix = match base_prefix {
            Some('b') => Some(Binary),
            Some('o') => Some(Octal),
            Some('#') => Some(Hexadecimal),
            _ => None,
        };

        if !has_digit {
            return None;
        }

        let end_pos = self.pos_within_token();

        let kind = if let Some(base) = prefix {
            Integer { base }
        } else if is_float {
            Float
        } else {
            Integer { base: Decimal }
        };

        Some(Token::new(Literal { kind }, end_pos - start_pos, &content))
    }

    fn check_number_literal(&mut self, mut has_digit: bool, mut content: String, radix: u32) -> (String, bool) {
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

        (content, has_digit)
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

        Some(Token::new(Identifier, end_pos - start_pos, &content))
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
                ';' => self.single_char_token(Semicolon, ";"),
                '{' => self.single_char_token(OpenCurlyBrace, "{"),
                '}' => self.single_char_token(CloseCurlyBrace, "}"),
                '(' => self.single_char_token(OpenBrace, "("),
                ')' => self.single_char_token(CloseBrace, ")"),
                '[' => self.single_char_token(OpenSquareBrace, "["),
                ']' => self.single_char_token(CloseSquareBrace, "]"),
                ',' => self.single_char_token(Comma, ","),
                '?' => self.single_char_token(Question, "?"),
                ':' => self.single_char_token(Colon, ":"),
                '+' => self.single_char_token(Add, "+"),
                '-' => self.single_char_token(Subtract, "-"),
                '*' => self.single_char_token(Multiply, "*"),
                '/' => self.single_char_token(Divide, "/"),
                '%' => self.single_char_token(Modulus, "%"),

                '=' => self.double_char_token(Assign, Equals, '=', "=", "=="),
                '!' => self.double_char_token(Exclamation, NotEquals, '=', "!", "!="),

                '>' => {
                    let next_char = self.next();

                    if next_char == '=' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(GreaterOrEqual, 2, ">="));
                    }

                    if next_char == '>' {
                        self.bump();

                        let next_next_char = self.next();

                        if next_next_char == '>' {
                            self.bump();
                            self.bump();
                            return Some(Token::new(UnsignedRightShift, 3, ">>>"));
                        }

                        self.bump();
                        return Some(Token::new(RightShift, 2, ">>"));
                    }

                    self.bump();

                    return Some(Token::new(Greater, 1, ">"));
                }

                '<' => {
                    let next_char = self.next();

                    if next_char == '=' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(LessOrEqual, 2, "<="));
                    }

                    if next_char == '<' {
                        self.bump();
                        self.bump();
                        return Some(Token::new(LeftShift, 2, "<<"));
                    }

                    self.bump();

                    return Some(Token::new(Less, 1, "<"));
                }

                '|' => self.double_char_token(BitwiseOr, Or, '|', "|", "||"),
                '&' => self.double_char_token(BitwiseAnd, And, '&', "&", "&&"),

                '^' => self.single_char_token(BitwiseXor, "^"),
                '~' => self.single_char_token(BitwiseComplement, "~"),

                'i' => {
                    keyword!(self, "if", If);
                    keyword!(self, "interface", Interface);
                    keyword!(self, "import", Import);
                    keyword!(self, "intern", AccessModifier { kind: Internal });
                    keyword!(self, "i8", Primitive { kind: I8 });
                    keyword!(self, "i16", Primitive { kind: I16 });
                    keyword!(self, "i32", Primitive { kind: I32 });
                    keyword!(self, "i64", Primitive { kind: I64 });
                    keyword!(self, "i128", Primitive { kind: I128 });
                    return self.try_match_identifier_literal();
                }
                's' => {
                    keyword!(self, "switch", Switch);
                    return self.try_match_identifier_literal();
                }
                'm' => {
                    keyword!(self, "match", Match);
                    keyword!(self, "mut", Mutable);
                    return self.try_match_identifier_literal();
                }
                'l' => {
                    keyword!(self, "loop", Loop);
                    return self.try_match_identifier_literal();
                }
                'f' => {
                    keyword!(self, "for", For);
                    keyword!(self, "false", Literal { kind: Boolean { val: false } });
                    keyword!(self, "f32", Primitive { kind: F32 });
                    keyword!(self, "f64", Primitive { kind: F64 });
                    return self.try_match_identifier_literal();
                }
                'r' => {
                    keyword!(self, "return", Return);
                    keyword!(self, "ret", Return);
                    return self.try_match_identifier_literal();
                }
                'c' => {
                    keyword!(self, "class", Class);
                    keyword!(self, "char", Primitive { kind: Char });
                    keyword!(self, "continue", Continue);
                    return self.try_match_identifier_literal();
                }
                'e' => {
                    keyword!(self, "enum", Enum);
                    return self.try_match_identifier_literal();
                }
                'p' => {
                    keyword!(self, "public", AccessModifier { kind: Public });
                    keyword!(self, "pub", AccessModifier { kind: Public });
                    keyword!(self, "protected", AccessModifier { kind: Protected });
                    keyword!(self, "prot", AccessModifier { kind: Protected });
                    return self.try_match_identifier_literal();
                }
                'b' => {
                    keyword!(self, "break", Break);
                    keyword!(self, "boolean", Primitive { kind: Bool });
                    keyword!(self, "bool", Primitive { kind: Bool });
                    if let Some(token) = self.try_match_number_literal() {
                        return Some(token);
                    }
                    return self.try_match_identifier_literal();
                }
                'u' => {
                    keyword!(self, "u8", Primitive { kind: U8 });
                    keyword!(self, "u16", Primitive { kind: U16 });
                    keyword!(self, "u32", Primitive { kind: U32 });
                    keyword!(self, "u64", Primitive { kind: U64 });
                    keyword!(self, "u128", Primitive { kind: U128 });
                    return self.try_match_identifier_literal();
                }
                't' => {
                    keyword!(self, "true", Literal { kind: Boolean { val: true } });
                    return self.try_match_identifier_literal();
                }
                _ => {
                    return self.try_match_identifier_literal();
                }
            };
        }
        None
    }
}