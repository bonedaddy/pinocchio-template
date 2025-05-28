use litesvm_client::traits::ProgramLoader;
use solana_sdk::pubkey::Pubkey;

pub struct TemplateProgramLoader {}

impl ProgramLoader for TemplateProgramLoader {
    fn data(&self) -> Vec<u8> {
        std::fs::read("../../target/deploy/program.so").unwrap()
    }
    fn pubkey(&self) -> Pubkey {
        sdk::ID
    }
}
