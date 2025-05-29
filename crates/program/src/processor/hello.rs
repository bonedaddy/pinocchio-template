use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use putils::{
    account::{AccountDeserialize, AccountSerialize, AccountWrite},
    processor::InstructionProcessor,
};

use crate::{instructions::Instructions, prelude::*, state::hello::Message};

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

impl<'a> InstructionProcessor<'a, Instructions> for HelloAccounts<'a> {
    fn from_accounts(accounts: &'a [AccountInfo]) -> Result<Self, ProgramError> {
        HelloAccounts::try_from(accounts)
    }
    fn process(&self, instruction: Instructions) -> ProgramResult {
        let Instructions::Hello { msg: msg_data } = instruction else {
            return Err(ProgramError::InvalidInstructionData);
        };

        let lamports = Rent::get()?.minimum_balance(Message::SERIALIZED_SIZE);

        pinocchio_system::instructions::CreateAccount {
            from: self.payer,
            to: self.msg,
            space: Message::SERIALIZED_SIZE as u64,
            lamports,
            owner: &crate::ID,
        }
        .invoke()?;

        pinocchio::msg!(&format!("{} {}", self.msg.data_len(), msg_data.len()));

        let mut parsed_message = [0u8; 32];
        parsed_message[0..msg_data.len()].copy_from_slice(&msg_data[0..msg_data.len()]);

        let msg_acct = Message {
            msg: parsed_message,
        };

        pinocchio::msg!("writing");

        msg_acct.account_write(self.msg)?;

        pinocchio::msg!("reading");

        let _ = Message::try_from_bytes(&self.msg.try_borrow_data()?).unwrap();

        Ok(())
    }
    fn validations(&self, instruction: &Instructions) -> ProgramResult {
        let Instructions::Hello { msg: msg_data } = instruction else {
            return Err(ProgramError::InvalidInstructionData);
        };

        if msg_data.len() > 32 {
            panic!("msg data is too large")
        }

        assert!(pinocchio_system::check_id(self.system_program.key()));

        assert!(self.payer.is_signer() && self.payer.is_writable());

        assert!(self.msg.is_writable());

        Ok(())
    }
}
