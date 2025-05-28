use pinocchio::{
    account_info::AccountInfo, default_allocator, default_panic_handler, entrypoint, msg,
    program_entrypoint, pubkey::Pubkey, ProgramResult,
};

use crate::instructions::Instructions;

program_entrypoint!(process_instruction);
default_allocator!();
default_panic_handler!();

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = Instructions::unpack(instruction_data)?;

    match ix {
        Instructions::Hello { msg } => {
            msg!(&String::from_utf8(msg).unwrap());
        }
    }
    Ok(())
}
