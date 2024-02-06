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
use crashc_lexer::token::TokenType::*;
use crashc_tree::misc::Modifier;
use crashc_tree::statements::modifier_block::ModifierBlock;
use crashc_tree::types::{NONE_TYPE, type_by_token};

use crate::statement::parse_statement;
use crate::stream::TokenStream;
use crate::*;

/// pub mut {
/// }
pub fn try_parse_modifier_block_statement(stream: &mut TokenStream) -> Option<ModifierBlock> {
    if !is_modifier(stream) {
        return None;
    }

    // Parse modifiers
    let parser = ModifierParser::parse(stream);

    // This position includes all modifiers
    let mut position = parser.position;

    // We don't have a block.
    if stream.peak(parser.peak).typ() != OpenCurlyBrace {
        return None;
    }

    // Jump over modifiers and open-curly-brace
    stream.add(parser.peak + 1);

    let mut statements = Vec::new();

    loop {
        if stream.current().typ() == CloseCurlyBrace {
            position = TokenPosition::new(
                *position.start(),
                *stream.current_pos().end(),
            );
            stream.advance();

            return Some(ModifierBlock::new(position, parser.modifiers, statements));
        }
        if !stream.has_more_tokens() {
            break;
        }
        if let Some(statement) = parse_statement(stream) {
            statements.push(statement);
            continue
        }
        break;
    }
    unexpected_eof!(stream);
}

pub struct ModifierParser {
    stream: TokenStream,
    position: TokenPosition,
    modifiers: Vec<Modifier>,
    peak: usize,
}

impl ModifierParser {
    pub fn parse(stream: &TokenStream) -> ModifierParser {
        let mut parser = ModifierParser {
            stream: stream.clone(),
            position: stream.current_pos(),
            modifiers: Vec::new(),
            peak: 0,
        };
        parser.parse_modifier();
        parser
    }

    fn parse_modifier(&mut self) {
        let modifier = Modifier::from_token(self.stream.peak(self.peak));

        if modifier == Modifier::None {
            if self.peak == 0 {
                return;
            }

            self.position = TokenPosition::new(
                *self.position.start(),
                *self.stream.peak(self.peak - 1).position().end(),
            );

            return;
        }

        self.modifiers.push(modifier);
        self.peak += 1;
        self.parse_modifier();
    }

    pub fn position(&self) -> TokenPosition {
        self.position
    }

    pub fn modifiers(&self) -> &Vec<Modifier> {
        &self.modifiers
    }

    pub fn peak(&self) -> usize {
        self.peak
    }
}

pub fn is_type(stream: &TokenStream) -> bool {
    let mut peak = 0;

    // We just skip all modifiers
    loop {
        if !is_modifier(stream) {
            break;
        }
        peak += 1;
    }

    return type_by_token(stream.peak(peak)) == NONE_TYPE;
}

pub fn is_modifier(stream: &TokenStream) -> bool {
    stream.matches(Public) || stream.matches(Protected) || stream.matches(Override) || stream.matches(Mutable) || stream.matches(Construct)
}
