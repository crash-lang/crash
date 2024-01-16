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


use crate::token::Base::*;
use crate::token::TokenKind::*;
use crate::token::LiteralKind;
use crate::token::LiteralKind::*;
use crate::tokenizer::Tokenizer;

#[test]
fn test_tokenize_string_some_literals() {
    let tokens = Tokenizer::new("\"Hello world\" \"Hey you\" 'a'")
        .tokenize();

    let mut index = 0;
    for tok in tokens {
        let content = tok.content;
        let len = tok.len;
        let kind = tok.kind;
        match index {
            0 => {
                assert_eq!(content, "Hello world");
                assert_eq!(len, 13);
                assert_eq!(kind, Literal { kind: String });
            }
            1 => {
                assert_eq!(content, "Hey you");
                assert_eq!(len, 9);
                assert_eq!(kind, Literal { kind: String });
            }
            2 => {
                assert_eq!(content, "a");
                assert_eq!(len, 3);
                assert_eq!(kind, Literal { kind: Character })
            }
            _ => {
                panic!("We don't have enough strings tokenized");
            }
        }
        index+=1;
    }
}

#[test]
fn test_tokenize_string_float_literals() {
    let tokens = Tokenizer::new(r"
        12.5_6
        34.6_356_56
    ").tokenize();

    let mut index = 0;
    for tok in tokens {
        let content = tok.content;
        let len = tok.len;
        let kind = tok.kind;
        match index {
            0 => {
                assert_eq!(content, "12.56");
                assert_eq!(len, 6);
                assert_eq!(kind, Literal { kind: Float });
            }
            1 => {
                assert_eq!(content, "34.635656");
                assert_eq!(len, 11);
                assert_eq!(kind, Literal { kind: Float });
            }
            _ => {
                panic!("We don't have enough floats tokenized");
            }
        }
        index+=1;
    }
}

#[test]
fn test_tokenize_string_int_literals() {
    let tokens = Tokenizer::new(r"
        345_65_6
        #3_4546f
        o456
        b000_10000
    ").tokenize();

    let mut index = 0;
    for tok in tokens {
        let content = tok.content;
        let len = tok.len;
        let kind = tok.kind;
        match index {
            0 => {
                assert_eq!(content, "345656");
                assert_eq!(len, 8);
                assert_eq!(kind, Literal { kind: Integer { base: Decimal } });
            }
            1 => {
                assert_eq!(content, "34546f");
                assert_eq!(len, 8);
                assert_eq!(kind, Literal { kind: Integer { base: Hexadecimal } });
            }
            2 => {
                assert_eq!(content, "456");
                assert_eq!(len, 4);
                assert_eq!(kind, Literal { kind: Integer { base: Octal } });
            }
            3 => {
                assert_eq!(content, "00010000");
                assert_eq!(len, 10);
                assert_eq!(kind, Literal { kind: Integer { base: Binary } });
            }
            _ => {
                panic!("We don't have enough integers tokenized");
            }
        }
        index+=1;
    }
}

#[test]
fn test_tokenize_operators() {
    let tokens = Tokenizer::new(r"
        ==
        !=
        ||
        &&
        |
        &
        <<
        >>
        <
        >
        |
        >>>
    ").tokenize();

    let mut index = 0;
    for tok in tokens {
        let content = tok.content;
        let len = tok.len;
        let kind = tok.kind;
        match index {
            0 => {
                assert_eq!(content, "==");
                assert_eq!(len, 2);
                assert_eq!(kind, Equals);
            }
            1 => {
                assert_eq!(content, "!=");
                assert_eq!(len, 2);
                assert_eq!(kind, NotEquals);
            }
            2 => {
                assert_eq!(content, "||");
                assert_eq!(len, 2);
                assert_eq!(kind, Or);
            }
            3 => {
                assert_eq!(content, "&&");
                assert_eq!(len, 2);
                assert_eq!(kind, And);
            }
            4 => {
                assert_eq!(content, "|");
                assert_eq!(len, 1);
                assert_eq!(kind, BitwiseOr);
            }
            5 => {
                assert_eq!(content, "&");
                assert_eq!(len, 1);
                assert_eq!(kind, BitwiseAnd);
            }
            6 => {
                assert_eq!(content, "<<");
                assert_eq!(len, 2);
                assert_eq!(kind, LeftShift);
            }
            7 => {
                assert_eq!(content, ">>");
                assert_eq!(len, 2);
                assert_eq!(kind, RightShift);
            }
            8 => {
                assert_eq!(content, "<");
                assert_eq!(len, 1);
                assert_eq!(kind, Less);
            }
            9 => {
                assert_eq!(content, ">");
                assert_eq!(len, 1);
                assert_eq!(kind, Greater);
            }
            10 => {
                assert_eq!(content, "|");
                assert_eq!(len, 1);
                assert_eq!(kind, BitwiseOr);
            }
            11 => {
                assert_eq!(content, ">>>");
                assert_eq!(len, 3);
                assert_eq!(kind, UnsignedRightShift);
            }
            _ => {
                panic!("We don't have enough things tokenized");
            }
        }
        index+=1;
    }
}

#[test]
fn test_tokenize_string_symbols() {
    let tokens = Tokenizer::new(r"
        ;{([)]},:?
    ").tokenize();

    let mut index = 0;
    for tok in tokens {
        let content = tok.content;
        let len = tok.len;
        let kind = tok.kind;
        match index {
            0 => {
                assert_eq!(content, ";");
                assert_eq!(len, 1);
                assert_eq!(kind, Semicolon);
            }
            1 => {
                assert_eq!(content, "{");
                assert_eq!(len, 1);
                assert_eq!(kind, OpenCurlyBrace);
            }
            2 => {
                assert_eq!(content, "(");
                assert_eq!(len, 1);
                assert_eq!(kind, OpenBrace);
            }
            3 => {
                assert_eq!(content, "[");
                assert_eq!(len, 1);
                assert_eq!(kind, OpenSquareBrace);
            }
            4 => {
                assert_eq!(content, ")");
                assert_eq!(len, 1);
                assert_eq!(kind, CloseBrace);
            }
            5 => {
                assert_eq!(content, "]");
                assert_eq!(len, 1);
                assert_eq!(kind, CloseSquareBrace);
            }
            6 => {
                assert_eq!(content, "}");
                assert_eq!(len, 1);
                assert_eq!(kind, CloseCurlyBrace);
            }
            7 => {
                assert_eq!(content, ",");
                assert_eq!(len, 1);
                assert_eq!(kind, Comma);
            }
            8 => {
                assert_eq!(content, ":");
                assert_eq!(len, 1);
                assert_eq!(kind, Colon);
            }
            9 => {
                assert_eq!(content, "?");
                assert_eq!(len, 1);
                assert_eq!(kind, Question);
            }
            _ => {
                panic!("We don't have enough symbols tokenized");
            }
        }
        index+=1;
    }
}
