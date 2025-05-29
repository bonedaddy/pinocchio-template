use hello::HelloAccounts;
use noop::NoopAccounts;
use pinocchio::{account_info::AccountInfo, ProgramResult};
use putils::{instruction_packer::InstructionPacker, processor::InstructionProcessor};

use crate::instructions::Instructions;

pub mod hello;
pub mod noop;

pub fn process(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let ix = Instructions::unpack(instruction_data)?;

    match ix {
        Instructions::Hello { .. } => {
            let accounts = HelloAccounts::from_accounts(accounts)?;
            accounts.try_process(ix)
        }
        Instructions::Noop { .. } => {
            let accounts = NoopAccounts::from_accounts(accounts)?;
            accounts.try_process(ix)
        }
    }
}
