use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::{instructions::Instructions, state::hello::Message};

pub struct HelloAccounts<'a> {
    payer: &'a AccountInfo,
    msg: &'a AccountInfo,
    system_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for HelloAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, msg, system_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        Ok(Self {
            payer,
            msg,
            system_program,
        })
    }
}

impl<'a> HelloAccounts<'a> {
    pub fn handler(&self, ix: Instructions) -> ProgramResult {
        let Instructions::Hello { msg: msg_data } = ix else {
            return Err(ProgramError::InvalidInstructionData);
        };

        let lamports = Rent::get()?.minimum_balance(msg_data.len() + std::mem::size_of::<u32>());

        pinocchio_system::instructions::CreateAccount {
            from: self.payer,
            to: self.msg,
            space: msg_data.len() as u64 + std::mem::size_of::<u32>() as u64,
            lamports,
            owner: &crate::ID,
        }
        .invoke()?;

        pinocchio::msg!(&format!("{} {}", self.msg.data_len(), msg_data.len()));

        let mut data = self.msg.try_borrow_mut_data()?;
        data.copy_from_slice(&Message { msg: msg_data }.pack());

        Ok(())
    }
    pub fn validations(&self) -> ProgramResult {
        assert!(pinocchio_system::check_id(self.system_program.key()));

        assert!(self.payer.is_signer() && self.payer.is_writable());

        assert!(self.msg.is_writable());

        Ok(())
    }
}
