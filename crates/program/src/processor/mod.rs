use hello::HelloAccounts;
use pinocchio::{account_info::AccountInfo, ProgramResult};

use crate::instructions::Instructions;

pub mod hello;

pub fn process(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let ix = Instructions::unpack(instruction_data)?;

    match ix {
        Instructions::Hello { .. } => {
            let accounts = HelloAccounts::try_from(accounts)?;
            accounts.handler(ix)
        }
    }
}
