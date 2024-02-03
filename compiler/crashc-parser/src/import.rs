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
use crashc_tree::expressions::ExpressionType;
use crashc_tree::expressions::literal::Literal;
use crashc_tree::statements::import::Import;
use crate::stream::TokenStream;
use crate::*;

pub fn try_parse_import_statement(stream: &mut TokenStream) -> Option<Import> {
    if !stream.matches(Import) {
        return None;
    }

    let mut position = stream.current_pos();

    /*
        We can already skip the import keyword,
        because imports only have one place they belong to
     */
    stream.advance();

    let mut imports = Vec::new();

    // We have a single import
    if stream.matches(IdentifierLiteral) {
        let token = stream.current();

        stream.advance();

        check_eof!(stream);

        // Invalid syntax. There must be a semicolon after statement
        if !stream.matches(Semicolon) {
            syntax_error!("Missing ; after import", stream.module_name(), stream.current_pos());
        }

        imports.push(ExpressionType::Literal(Literal::Identifier {
            position: token.position(),
            name: token.value().to_string()
        }));

        position = TokenPosition::new(
            *position.start(),
            *stream.current_pos().end()
        );

        stream.advance();

        return Some(Import::new(position, imports));
    }

    if !stream.matches(OpenCurlyBrace) {
        syntax_error!("Invalid import syntax", stream.module_name(), stream.current_pos());
    }

    stream.advance();

    loop {
        if stream.matches(CloseCurlyBrace) {
            position = TokenPosition::new(
                *position.start(),
                *stream.current_pos().end()
            );

            stream.advance();

            return Some(Import::new(position, imports));
        }
        if !stream.has_more_tokens() {
            unexpected_eof!(stream);
        }
        if !stream.matches(IdentifierLiteral) {
            syntax_error!("Invalid import syntax", stream.module_name(), stream.current_pos());
        }

        let token = stream.current();

        imports.push(ExpressionType::Literal(Literal::Identifier {
            position: token.position(),
            name: token.value().to_string()
        }));

        stream.advance();

        if stream.matches(Comma) {
            stream.advance();
            continue
        }

        if stream.matches(CloseCurlyBrace) {
            continue
        }

        syntax_error!("Invalid import syntax", stream.module_name(), stream.current_pos());
    }
}