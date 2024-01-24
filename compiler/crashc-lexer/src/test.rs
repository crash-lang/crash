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
use crate::token::*;
use crate::token::LiteralKind::*;
use crate::tokenizer::Tokenizer;

fn assert_tok(tok: Token, expected_string: &str, expected_len: u32, expected_kind: TokenKind) {
    assert_eq!(tok.content(), expected_string);
    assert_eq!(tok.len(), expected_len);
    assert_eq!(tok.kind(), expected_kind);
}

#[test]
fn test_tokenize_string_some_literals() {
    let tokens = Tokenizer::new("\"Hello world\" \"Hey you\" 'a'")
        .tokenize();

    let mut index = 0;
    for tok in tokens {
        match index {
            0 => assert_tok(tok, "Hello world", 13, Literal { kind: String }),
            1 => assert_tok(tok, "Hey you", 9, Literal { kind: String }),
            2 => assert_tok(tok, "a", 3, Literal { kind: Character }),
            _ => panic!("We don't have enough strings tokenized")
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
        match index {
            0 => assert_tok(tok, "12.56", 6, Literal { kind: Float }),
            1 => assert_tok(tok, "34.635656", 11, Literal { kind: Float }),
            _ => panic!("We don't have enough floats tokenized")
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
        match index {
            0 => assert_tok(tok, "345656", 8, Literal { kind: Integer { base: Decimal }}),
            1 => assert_tok(tok, "34546f", 8, Literal { kind: Integer { base: Hexadecimal } }),
            2 => assert_tok(tok, "456", 4, Literal { kind: Integer { base: Octal } }),
            3 => assert_tok(tok, "00010000", 10, Literal { kind: Integer { base: Binary } }),
            _ => panic!("We don't have enough integers tokenized")
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
        match index {
            0 => assert_tok(tok, "==", 2, Equals),
            1 => assert_tok(tok, "!=", 2, NotEquals),
            2 => assert_tok(tok, "||", 2, Or),
            3 => assert_tok(tok, "&&", 2, And),
            4 => assert_tok(tok, "|", 1, BitwiseOr),
            5 => assert_tok(tok, "&", 1, BitwiseAnd),
            6 => assert_tok(tok, "<<", 2, LeftShift),
            7 => assert_tok(tok, ">>", 2, RightShift),
            8 => assert_tok(tok, "<", 1, Less),
            9 => assert_tok(tok, ">", 1, Greater),
            10 => assert_tok(tok, "|", 1, BitwiseOr),
            11 => assert_tok(tok, ">>>", 3, UnsignedRightShift),
            _ => panic!("We don't have enough things tokenized")
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
        match index {
            0 => assert_tok(tok, ";", 1, Semicolon),
            1 => assert_tok(tok, "{", 1, OpenCurlyBrace),
            2 => assert_tok(tok, "(", 1, OpenBrace),
            3 => assert_tok(tok, "[", 1, OpenSquareBrace),
            4 => assert_tok(tok, ")", 1, CloseBrace),
            5 => assert_tok(tok, "]", 1, CloseSquareBrace),
            6 => assert_tok(tok, "}", 1, CloseCurlyBrace),
            7 => assert_tok(tok, ",", 1, Comma),
            8 => assert_tok(tok, ":", 1, Colon),
            9 => assert_tok(tok, "?", 1, Question),
            _ => panic!("We don't have enough symbols tokenized")
        }
        index+=1;
    }
}
