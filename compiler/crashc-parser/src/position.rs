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

use crashc_lexer::token::TokenPosition;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct StructurePosition {
    start: TokenPosition,
    end: TokenPosition
}

impl StructurePosition {
    pub fn new(start: TokenPosition, end: TokenPosition) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> TokenPosition {
        self.start
    }
    pub fn end(&self) -> TokenPosition {
        self.end
    }
}