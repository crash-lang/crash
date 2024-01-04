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

use crate::{Base, LiteralKind, tokenize_string, TokenKind};

#[test]
fn test_tokenize_string_string_literal() {
    let tokens = tokenize_string("\"Hello world\" \"Hey you\"");
    let mut index = 0;
    for tok in tokens {
        let content = tok.content;
        let len = tok.len;
        let kind = tok.kind;
        match index {
            0 => {
                assert_eq!(content, "Hello world");
                assert_eq!(len, 13);
                assert_eq!(kind, TokenKind::Literal { kind: LiteralKind::String { terminated: true } });
            }
            1 => {
                assert_eq!(content, "Hey you");
                assert_eq!(len, 9);
                assert_eq!(kind, TokenKind::Literal { kind: LiteralKind::String { terminated: true } });
            }
            _ => {
                panic!("We don't have enogh strings tokenized");
            }
        }
        index+=1;
    }
}

#[test]
fn test_tokenize_string_float_literals() {
    let tokens = tokenize_string(r"
        12.5_6
        34.6_356_56
    ");

    let mut index = 0;
    for tok in tokens {
        let content = tok.content;
        let len = tok.len;
        let kind = tok.kind;
        match index {
            0 => {
                assert_eq!(content, "12.56");
                assert_eq!(len, 6);
                assert_eq!(kind, TokenKind::Literal { kind: LiteralKind::Float })
            }
            1 => {
                assert_eq!(content, "34.635656");
                assert_eq!(len, 11);
                assert_eq!(kind, TokenKind::Literal { kind: LiteralKind::Float })
            }
            _ => {
                panic!("We don't have enogh floats tokenized");
            }
        }
        index+=1;
    }
}

#[test]
fn test_tokenize_string_int_literals() {
    let tokens = tokenize_string(r"
        345_65_6
        #3_4546f
        o456
        b000_10000
    ");

    let mut index = 0;
    for tok in tokens {
        let content = tok.content;
        let len = tok.len;
        let kind = tok.kind;
        match index {
            0 => {
                assert_eq!(content, "345656");
                assert_eq!(len, 8);
                assert_eq!(kind, TokenKind::Literal { kind: LiteralKind::Integer { base: Base::Decimal } })
            }
            1 => {
                assert_eq!(content, "34546f");
                assert_eq!(len, 8);
                assert_eq!(kind, TokenKind::Literal { kind: LiteralKind::Integer { base: Base::Hexadecimal } })
            }
            2 => {
                assert_eq!(content, "456");
                assert_eq!(len, 4);
                assert_eq!(kind, TokenKind::Literal { kind: LiteralKind::Integer { base: Base::Octal } })
            }
            3 => {
                assert_eq!(content, "00010000");
                assert_eq!(len, 10);
                assert_eq!(kind, TokenKind::Literal { kind: LiteralKind::Integer { base: Base::Binary } })
            }
            _ => {
                panic!("We don't have enogh integers tokenized");
            }
        }
        index+=1;
    }
}

