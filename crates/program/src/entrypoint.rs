use pinocchio::{
    account_info::AccountInfo, default_allocator, default_panic_handler,
    program_entrypoint, pubkey::Pubkey, ProgramResult,
};

use crate::processor;

program_entrypoint!(process_instruction);
default_allocator!();
default_panic_handler!();

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert!(program_id.eq(&crate::ID));

    processor::process(accounts, instruction_data)
}
