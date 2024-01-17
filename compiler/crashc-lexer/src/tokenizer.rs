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
 *
 */

use std::sync::Mutex;

use crate::*;
use crate::cursor::Cursor;
use crate::token::Token;
use crate::token::*;
use crate::token::LiteralKind::{Boolean, Character, Float, Integer};
use crate::token::TokenKind::*;
use crate::token::PrimitiveKind::*;
use crate::token::ModifierKind::*;
use crate::token::Base::*;
use crate::utils::*;

pub struct Tokenizer<'a> {
    input: &'a str,
    cursor: Mutex<Cursor<'a>>,
}

impl<'a> Tokenizer<'a> {

    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            input,
            cursor: Mutex::new(Cursor::new(input)),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut cursor = lock_cursor!(self);
        let mut tokens = Vec::new();

        while !cursor.is_eof() {
            drop(cursor);

            if let Some(token) = self.tokenize_next() {
                cursor = lock_cursor!(self);
                tokens.push(token);
                continue;
            }
            cursor = lock_cursor!(self);
            cursor.advance();
        }

        tokens
    }

    fn tokenize_next(&mut self) -> Option<Token> {
        let mut cursor = lock_cursor!(self);

        cursor.skip_whitespace();

        if let Some(c) = cursor.current_char() {
            drop(cursor);

            if self.skip_comment() {
                return self.tokenize_next();
            }

            return match c {
                '"' => self.try_match_string_literal(),
                '\'' => self.try_match_char_literal(),
                '#' => self.try_match_number_literal(),
                'o' => {
                    if let Some(token)  = self.try_match_number_literal() {
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
                    cursor = lock_cursor!(self);

                    let next_char = cursor.next();

                    if next_char == '=' {
                        cursor.bump();
                        cursor.bump();
                        drop(cursor);
                        return Some(Token::new(GreaterOrEqual, 2, ">="));
                    }

                    if next_char == '>' {
                        cursor.bump();

                        let next_next_char = cursor.next();

                        if next_next_char == '>' {
                            cursor.bump();
                            cursor.bump();
                            drop(cursor);
                            return Some(Token::new(UnsignedRightShift, 3, ">>>"));
                        }

                        cursor.bump();
                        drop(cursor);
                        return Some(Token::new(RightShift, 2, ">>"));
                    }

                    cursor.bump();
                    drop(cursor);
                    return Some(Token::new(Greater, 1, ">"));
                }

                '<' => {
                    cursor = lock_cursor!(self);

                    let next_char = cursor.next();

                    if next_char == '=' {
                        cursor.bump();
                        cursor.bump();
                        drop(cursor);
                        return Some(Token::new(LessOrEqual, 2, "<="));
                    }

                    if next_char == '<' {
                        cursor.bump();
                        cursor.bump();
                        drop(cursor);
                        return Some(Token::new(LeftShift, 2, "<<"));
                    }

                    cursor.bump();
                    drop(cursor);

                    return Some(Token::new(Less, 1, "<"));
                }

                '|' => self.double_char_token(BitwiseOr, Or, '|', "|", "||"),
                '&' => self.double_char_token(BitwiseAnd, And, '&', "&", "&&"),

                '^' => self.single_char_token(BitwiseXor, "^"),
                '~' => self.single_char_token(BitwiseComplement, "~"),

                'i' => {
                    keyword!(self, "if", If);
                    keyword!(self, "interface", Interface);
                    keyword!(self, "implements", Implements);
                    keyword!(self, "import", Import);
                    keyword!(self, "intern", AccessModifier { kind: Internal });
                    keyword!(self, "i8", Primitive { kind: I8 });
                    keyword!(self, "i16", Primitive { kind: I16 });
                    keyword!(self, "i32", Primitive { kind: I32 });
                    keyword!(self, "int", Primitive { kind: I32 });
                    keyword!(self, "i64", Primitive { kind: I64 });
                    return self.try_match_identifier_literal();
                }
                's' => {
                    keyword!(self, "switch", Switch);
                    keyword!(self, "short", Primitive { kind: I16 });
                    return self.try_match_identifier_literal();
                }
                'm' => {
                    keyword!(self, "match", Match);
                    keyword!(self, "mut", Mutable);
                    return self.try_match_identifier_literal();
                }
                'l' => {
                    keyword!(self, "loop", Loop);
                    keyword!(self, "long", Primitive { kind: I64 });
                    return self.try_match_identifier_literal();
                }
                'f' => {
                    keyword!(self, "for", For);
                    keyword!(self, "false", Literal { kind: Boolean { val: false } });
                    keyword!(self, "f32", Primitive { kind: F32 });
                    keyword!(self, "float", Primitive { kind: F32 });
                    keyword!(self, "f64", Primitive { kind: F64 });
                    return self.try_match_identifier_literal();
                }
                'd' => {
                    keyword!(self, "double", Primitive { kind: F64 });
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
                    keyword!(self, "byte", Primitive { kind: I8 });
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
                    return self.try_match_identifier_literal();
                }
                't' => {
                    keyword!(self, "true", Literal { kind: Boolean { val: true } });
                    return self.try_match_identifier_literal();
                }
                _ => None
            }
        }

        None
    }

    fn single_char_token(&mut self, kind: TokenKind, content: &str) -> Option<Token> {
        self.cursor.lock().unwrap().advance();
        Some(Token::new(kind, 1, content))
    }

    fn double_char_token(&mut self, kind1: TokenKind, kind2: TokenKind, expected: char, val1: &str, val2: &str) -> Option<Token> {
        let mut cursor = lock_cursor!(self);

        let next_char = cursor.next();
        if next_char == expected {
            cursor.bump();
            cursor.bump();
            Some(Token::new(kind2, 2, val2))
        } else {
            cursor.advance();
            Some(Token::new(kind1, 1, val1))
        }
    }

    pub(crate) fn try_match_identifier_literal(&mut self) -> Option<Token> {
        let mut cursor = lock_cursor!(self);

        let start_pos = cursor.pos_within_token();
        let mut content = String::new();

        loop {
            if let Some(current_char) = cursor.current_char() {
                if current_char.is_alphabetic() || current_char == '.' || current_char == ':' {
                    content.push(current_char);
                    cursor.bump();
                } else {
                    /*
                        Since we don't really now what could come next,
                        we just ignore the fact, that illegal characters
                        could sit in this identifier literal.
                     */
                    break;
                }
            }
            // Probably eof here. We don't care.
        }

        Some(Token::new(Identifier, cursor.pos_within_token() - start_pos, &content))
    }

    /// Returns true if skipped
    pub(crate) fn skip_comment(&mut self) -> bool {
        let cursor = lock_cursor!(self);

        if let Some(c) = cursor.current_char() {
            if c == '/' {
                let next = cursor.next();
                drop(cursor); // Next functions lock cursor
                if next == '/' {
                    self.skip_line_comment();
                    return true;
                }
                if next == '*' {
                    self.skip_multi_line_comment();
                    return true;
                }
            }
        }

        false
    }

    fn skip_line_comment(&mut self) {
        let mut cursor = lock_cursor!(self);
        cursor.bump();
        cursor.bump();

        loop {
            if let Some(c) = cursor.current_char() {
                if c == '\n' {
                    cursor.bump();
                    break;
                }
                // ignore the comment
                cursor.bump();
            } else {
                /*
                    Here could be an eof. Since single lines comment end is a new line,
                    we don't care about it.
                */
                break;
            }
        }
    }

    fn skip_multi_line_comment(&mut self) {
        let mut cursor = lock_cursor!(self);
        cursor.bump();
        cursor.bump();

        loop {
            if let Some(c) = cursor.current_char() {
                if c == '*' && cursor.next() == '/' {
                    cursor.bump();
                    cursor.bump();
                    break;
                }
                // well, we ignore it so skiiiip
                cursor.bump();
            } else {
                syntax_error!(cursor.pos_within_token(), self.input, "Comment never closed", "");
            }
        }
    }

    /// Tokenizes every number literal
    pub(crate) fn try_match_number_literal(&mut self) -> Option<Token> {
        let input = self.input;

        let mut cursor = lock_cursor!(self);
        let start_pos = cursor.pos_within_token();

        let mut has_digit = false;
        let mut content = String::new();

        let base_prefix = match cursor.current_char() {
            Some('o') => {
                let next = cursor.next();
                if !next.is_digit(8) {
                    return None;
                }
                let base_char = cursor.current_char().unwrap();
                cursor.bump();
                Some(base_char)
            }
            Some('b') => {
                let next = cursor.next();
                if !next.is_digit(2) {
                    return None;
                }
                let base_char = cursor.current_char().unwrap();
                cursor.bump();
                Some(base_char)
            }
            Some('#') => {
                let base_char = cursor.current_char().unwrap();
                cursor.bump();
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

        drop(cursor); // Need to drop, because next function call locks cursor

        let ret = self.check_number_literal(has_digit, content, radix);
        content = ret.0;
        has_digit = ret.1;

        // Re-locking cursor after we gave it to last function call
        cursor = lock_cursor!(self);

        if !has_digit {
            syntax_error!(cursor.pos_within_token(), input, "Invalid integer literal", "");
        }

        let is_float = match cursor.current_char() {
            Some('.') => {
                content.push('.');
                cursor.bump();
                true
            }
            _ => false
        };

        if is_float {
            drop(cursor);

            let ret = self.check_number_literal(has_digit, content, radix);

            cursor = lock_cursor!(self);

            content = ret.0;
            has_digit = ret.1;
        }

        if let Some('e') | Some('E') = cursor.current_char() {
            content.push(cursor.current_char().unwrap());
            cursor.bump();

            drop(cursor); // Need to drop, because next function call locks cursor

            let ret = self.check_number_literal(has_digit, content, radix);

            content = ret.0;
            has_digit = ret.1;

            cursor = lock_cursor!(self);
        }

        let prefix = match base_prefix {
            Some('b') => Some(Binary),
            Some('o') => Some(Octal),
            Some('#') => Some(Hexadecimal),
            _ => None
        };

        if !has_digit {
            syntax_error!(cursor.pos_within_token(), input, "Invalid float literal", "");
        }

        let end_pos = cursor.pos_within_token();

        let kind;

        if let Some(base) = prefix {
            kind = Integer { base }
        } else if is_float {
            kind = Float
        } else {
            kind = Integer { base: Decimal }
        };

        Some(Token::new(Literal { kind }, end_pos - start_pos, &content))
    }

    pub(crate) fn check_number_literal(&self, mut has_digit: bool, mut content: String, radix: u32) -> (String, bool) {
        let mut cursor = lock_cursor!(self);

        while let Some(current_char) = cursor.current_char() {
            if current_char == '_' {
                cursor.bump();
                continue;
            }
            if current_char.is_digit(radix) {
                has_digit = true;
                content.push(current_char);
                cursor.bump();
            } else {
                break;
            }
        }

        (content, has_digit)
    }

    pub(crate) fn try_match_string_literal(&mut self) -> Option<Token> {
        let mut cursor = lock_cursor!(self);
        let start_pos = cursor.pos_within_token();

        if cursor.current_char() == Some('"') {
            cursor.bump();

            let mut content = String::new();

            while let Some(current_char) = cursor.current_char() {
                cursor.bump();

                if current_char == '"' {
                    let end_pos = cursor.pos_within_token();

                    return Some(Token::new(Literal {
                        kind: LiteralKind::String
                    }, end_pos - start_pos, content.as_str()));
                }

                content.push(current_char);
            }
        }

        syntax_error!(start_pos, self.input, "Unterminated string literal", "Please terminate the string literal with another quote \"");
    }

    pub(crate) fn try_match_char_literal(&mut self) -> Option<Token> {
        let input = self.input;

        let mut cursor = lock_cursor!(self);
        let start_pos = cursor.pos_within_token();

        if cursor.current_char() == Some('\'') {
            cursor.bump(); // skip quote

            let mut content = String::new();

            if let Some(char_value) = cursor.current_char() {
                content.push(char_value);
                cursor.bump();

                // expecting end of char literal with quote
                if cursor.current_char() == Some('\'') {
                    cursor.bump();

                    let end_pos = cursor.pos_within_token();

                    return Some(Token::new(Literal {
                        kind: Character
                    }, end_pos - start_pos, &content));
                } else if cursor.current_char() != None {
                    // The char literal is not closed by an quote
                    syntax_error!(start_pos, input, "Unterminated char literal", "Please terminate the char literal with another quote '");
                }
            }

            syntax_error!(start_pos, input, "Invalid symbol quote '", "If you meant a char literal, please add a char and terminate the literal with another quote '");
        }

        None
    }

    pub(crate) fn try_match_keyword(&mut self, expected: &str, token_kind: TokenKind) -> Option<Token> {
        let mut cursor = lock_cursor!(self);

        if cursor.chars().as_str().starts_with(expected) {
            let start_pos = cursor.pos_within_token();

            for _ in 0..expected.len() {
                cursor.bump();
            }

            let end_pos = cursor.pos_within_token();

            drop(cursor);

            return Some(Token::new(token_kind, end_pos - start_pos, expected));
        }

        None
    }
}