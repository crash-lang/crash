pub const INVALID_POS: Position = Position {
    line: -1,
    column: -1
};

pub const INVALID_TOKEN_POS: TokenPosition = TokenPosition {
    start: INVALID_POS,
    end: INVALID_POS
};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    line: i32,
    column: i32
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct TokenPosition {
    start: Position,
    end: Position
}

impl Position {
    pub fn new(line: i32, column: i32) -> Self {
        Self { line, column }
    }

    pub fn line(&self) -> i32 {
        self.line
    }
    pub fn column(&self) -> i32 {
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