use program::instructions::Instructions;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

/// Helper type that converts a pinochcio pubkey into a [`solana_sdk::pubkey::Pubkey`]
pub const ID: Pubkey = Pubkey::new_from_array(program::ID);

/// Builds a [`Instructions::Hello`] [`Instruction`]
pub fn hello_instruction(payer: Pubkey, msg_account: Pubkey, msg: Vec<u8>) -> Instruction {
    Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(msg_account, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Instructions::Hello { msg }.pack(),
    }
}
