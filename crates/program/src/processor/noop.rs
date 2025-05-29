use pinocchio::program_error::ProgramError;
use putils::processor::InstructionProcessor;

use crate::instructions::Instructions;

pub struct NoopAccounts {}

impl<'a> InstructionProcessor<'a, Instructions> for NoopAccounts {
    fn from_accounts(
        accounts: &'a [pinocchio::account_info::AccountInfo],
    ) -> Result<Self, pinocchio::program_error::ProgramError> {
        Ok(Self {})
    }
    fn process(&self, instruction: Instructions) -> pinocchio::ProgramResult {
        let Instructions::Noop { msg } = instruction else {
            return Err(ProgramError::InvalidInstructionData);
        };

        pinocchio::log::sol_log_data(&[&msg]);
        Ok(())
    }
    fn validations(&self, instruction: &Instructions) -> pinocchio::ProgramResult {
        Ok(())
    }
}
