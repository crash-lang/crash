use crate::instructions::init::InitInstruction;
use crate::instructions::mov::MoveInstruction;

mod mov;
mod init;

#[derive(Clone)]
pub enum Instruction {
    Mov(MoveInstruction),
    Init(InitInstruction),

    
}