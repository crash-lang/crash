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

use std::str::FromStr;
use crashc_lexer::token::TokenType::*;
use crate::stream::TokenStream;

pub enum LiteralExpr {
    String(String),
    Char(char),
    Boolean(bool),
    Number(NumberLiteralExpr)
}

pub enum NumberLiteralExpr {
    Float(f64),
    Integer(u128)
}

impl TokenStream {

    pub(super) fn try_parse_literal_expr(&mut self) -> Option<LiteralExpr> {
        let current_tok = self.current();
        let current_type = current_tok.typ();
        let current_val = current_tok.value();

        if current_type == BooleanLiteral {
            return match current_val {
                "true" => Some(LiteralExpr::Boolean(true)),
                "false" => Some(LiteralExpr::Boolean(false)),
                _ => None // shouldn't happen
            }
        }

        if current_type == StringLiteral {
            let mut content = current_val[1..].to_string();
            content.pop();
            return Some(LiteralExpr::String(content));
        }

        if current_type == CharLiteral {
            let mut content = current_val[1..].to_string();
            content.pop();
            return Some(LiteralExpr::Char(char::from_str(content.as_str()).unwrap_or_else(|err| {
                panic!("Unable to convert string to char: {:?}", err)
            })));
        }

        if current_type == BinaryLiteral {

        }

        None
    }
}