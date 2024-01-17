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

use crashc_lexer::token::Token;
use crate::decl::module::ModuleDecl;
use crate::parser::class::parse_class;
use crate::parser::constants::parse_constants;
use crate::parser::Cursor;
use crate::parser::enumeration::parse_enum;
use crate::parser::functions::parse_functions;
use crate::parser::import::parse_imports;

mod decl;
mod parser;

pub fn build_module_decl(tokens: Vec<Token>, module_name: String) -> ModuleDecl {
    // No need for making it mutable, because each part gets parsed separately anyway.
    let cursor = Cursor::new(tokens.iter());

    // Maybe make it async later, shouldn't be that hard.
    ModuleDecl::new(
        module_name,
        parse_imports(cursor.clone()),
        parse_constants(cursor.clone()),
        parse_functions(cursor.clone()),
        parse_class(cursor.clone()),
        parse_enum(cursor.clone()),
        None
    )
}