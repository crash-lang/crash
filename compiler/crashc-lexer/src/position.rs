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


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    line: u32,
    column: u32
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TokenPosition {
    start: Position,
    end: Position
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    pub fn line(&self) -> u32 {
        self.line
    }
    pub fn column(&self) -> u32 {
        self.column
    }
}

impl TokenPosition {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> &Position {
        &self.start
    }
    pub fn end(&self) -> &Position {
        &self.end
    }
}