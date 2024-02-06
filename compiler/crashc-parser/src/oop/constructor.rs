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

use crashc_lexer::token::TokenType::OpenBrace;
use crashc_tree::statements::function::Function;
use crate::module::modifier::ModifierParser;
use crate::stream::TokenStream;

pub fn try_parse_constructor_statement(stream: &mut TokenStream) -> Option<Function> {
    // Maybe we have some Access-Modifiers
    let modifier_parser = ModifierParser::parse(stream);

    let mut position = modifier_parser.position();

    // We don't have a constructor
    if stream.peak(modifier_parser.peak()).typ() != OpenBrace {
        return None;
    }

    //TODO read parameters and parse body

    None
}