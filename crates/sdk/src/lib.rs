use program::instructions::Instructions;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

pub const ID: Pubkey = Pubkey::new_from_array(program::ID);

pub fn hello_instruction(msg: Vec<u8>) -> Instruction {
    Instruction {
        program_id: ID,
        accounts: vec![],
        data: Instructions::Hello { msg }.pack(),
    }
}
