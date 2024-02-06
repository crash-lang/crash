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

use crashc_lexer::token::TokenType;
use crashc_tree::statements::if_statement::If;
use crate::stream::TokenStream;

/// if something {
///     doSomething();
/// }
pub fn try_parse_if_statement(stream: &mut TokenStream) -> Option<If> {
    if !stream.matches(TokenType::If) {
        return None;
    }

    let mut position = stream.current_pos();

    stream.advance();



    None
}