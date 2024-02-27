pub use {
    crate::instructions::init::InitInstruction,
    crate::instructions::mov::MoveInstruction
};

mod mov;
mod init;

#[derive(Clone)]
pub enum Instruction {
    Mov(MoveInstruction),
    Init(InitInstruction),

    
}