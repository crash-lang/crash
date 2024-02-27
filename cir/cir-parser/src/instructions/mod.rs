mod mov;

use crash_ast::instructions::Instruction;
use crash_ir_lexer::TokenType;
use crate::parser::Parser;

impl Parser {

    pub fn try_parse_instruction(&mut self) -> Option<Instruction> {
        if !self.next_is_type(TokenType::Move) {
            return None;
        }

        let start_pos = self.expect_type(TokenType::Move);

        //TODO parse rest

        None
    }

}