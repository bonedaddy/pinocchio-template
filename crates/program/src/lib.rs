use pinocchio::pubkey::Pubkey;

//#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub mod instructions;
pub mod processor;
pub mod state;

pub const ID: Pubkey = pinocchio_pubkey::pubkey!("9L3SSFhNeLgE5nVMwXmSfuc4khzxKpJvJNafJquxUd8v");
