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

use crate::position::{Position, TokenPosition};

#[derive(Debug)]
pub struct Token {
    typ: TokenType,
    value: String,
    position: TokenPosition,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Eof,
    Whitespace,

    // Literals
    IdentifierLiteral,
    BinaryLiteral,
    OctalLiteral,
    DecimalLiteral,
    HexadecimalLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,
    BooleanLiteral,

    // Primitive types
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    F32,
    F64,
    Boolean,
    Char,

    // Symbols
    Semicolon,
    Comma,
    OpenBrace,
    CloseBrace,
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenSquareBrace,
    CloseSquareBrace,
    Question,
    Colon,
    At,
    Dots,

    // Math
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,

    // Logic
    Equals,
    NotEquals,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Exclamation,
    And,
    Or,

    // Bitwise and shifting
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    BitwiseComplement,
    LeftShift,
    RightShift,
    UnsignedRightShift,

    Assign,
    Mutable,

    // Statements
    If,
    Switch,
    Match,
    Loop,
    For,
    Return,
    Break,
    Continue,

    // General keywords
    Class,
    Interface,
    Enum,
    Annotation,
    Import,
    Implements,

    // Modifiers
    Public,
    Protected
}

impl Token {
    pub(crate) fn new(typ: TokenType, value: String, position: Position) -> Token {
        let length = value.len();
        Token {
            typ,
            value,
            position: TokenPosition::new(position,
                                         Position::new(
                                             position.line(),
                                             position.column() + (length as u32))),
        }
    }

    pub fn typ(&self) -> TokenType {
        self.typ.clone()
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn position(&self) -> TokenPosition {
        self.position
    }
}