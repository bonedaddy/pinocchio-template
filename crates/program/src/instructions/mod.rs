use pinocchio::program_error::ProgramError;
use putils::{discriminator::InstructionDiscriminator, instruction_packer::InstructionPacker};

/// Identifies the different instructions supported by the program
#[derive(Clone)]
pub enum Instructions {
    /// Accounts
    /// 0. [writeable] - msg account
    /// 1. [] - system_program
    Hello {
        msg: Vec<u8>,
    },
    Noop {
        msg: Vec<u8>,
    },
}

impl InstructionDiscriminator for Instructions {
    fn discriminator(&self) -> u8 {
        match self {
            Self::Hello { .. } => 0,
            Self::Noop { .. } => 1,
        }
    }
}

impl InstructionPacker for Instructions {
    /// Used to pack the instruction into instruction data to be included as a program instruction
    fn pack(&self) -> Vec<u8> {
        match self {
            Self::Hello { msg } => {
                let mut buf = Vec::with_capacity(1 + msg.len());

                buf.push(self.discriminator());
                buf.extend_from_slice(&msg);

                buf
            }
            Self::Noop { msg } => {
                let mut buf = Vec::with_capacity(1 + msg.len());

                buf.push(self.discriminator());
                buf.extend_from_slice(&msg);

                buf
            }
        }
    }

    /// Unpacks the given data into a program instruction
    ///
    /// # Errors
    ///
    /// * Returns [`ProgramError::InvalidInstructionData`] if instruction unpacking fails
    fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        let (first, rest) = data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match first {
            0 => Ok(Self::Hello { msg: rest.to_vec() }),
            1 => Ok(Self::Noop { msg: rest.to_vec() }),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
