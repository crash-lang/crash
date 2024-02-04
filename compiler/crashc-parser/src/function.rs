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

use crashc_lexer::position::TokenPosition;
use crashc_lexer::token::TokenType::{IdentifierLiteral, OpenBrace};
use crashc_tree::statements::function::Function;
use crate::modifier::ModifierParser;
use crate::stream::TokenStream;

pub fn try_parse_function_statement(stream: &mut TokenStream) -> Option<Function> {
    // Maybe we have some Access-Modifiers
    let modifier_parser = ModifierParser::parse(stream);

    let mut position = modifier_parser.position();

    let last = stream.peak(modifier_parser.peak());

    // We may not have a function here
    if last.typ() != IdentifierLiteral {
        return None;
    }

    let next = stream.peak(modifier_parser.peak() + 1);

    let function_name;
    let mut return_type = None;

    if next.typ() == IdentifierLiteral {
        if stream.peak(modifier_parser.peak() + 2).typ() != OpenBrace {
            return None;
        }
        function_name = next.value().to_string();
        return_type = Some(last.value().to_string());
        stream.add(modifier_parser.peak() + 2);
    } else if next.typ() == OpenBrace {
        function_name = last.value().to_string();
        stream.add(modifier_parser.peak() + 1);
    } else {
        return None;
    }

    drop(last);
    drop(next);

    //TODO read parameters and parse body

    None
}

pub struct ParameterDeclParser {
    stream: TokenStream,
    position: TokenPosition,
    peak: usize,
}

impl ParameterDeclParser {
    pub fn parse(stream: &TokenStream) -> ParameterDeclParser {
        let mut parser = ParameterDeclParser {
            stream: stream.clone(),
            position: stream.current_pos(),
            peak: 0,
        };
        parser.parse_parameter();
        parser
    }

    fn parse_parameter(&mut self) {

    }
}