use borsh_derive::{BorshDeserialize, BorshSerialize};
use putils::{
    account::{AccountDeserialize, AccountSerialize, AccountWrite},
    discriminator::AccountDiscriminator,
};

#[derive(Clone)]
pub struct Message {
    pub msg: [u8; 32],
}

impl AccountDiscriminator for Message {
    const DISCRIMINATOR: u8 = 69;
}

impl AccountSerialize for Message {
    const SERIALIZED_SIZE: usize = 33;
    fn to_bytes_inner(&self) -> Vec<u8> {
        self.msg.to_vec()
    }
}

impl AccountDeserialize for Message {
    fn from_bytes(data: &[u8]) -> Self {
        let mut msg = Self { msg: [0u8; 32] };
        msg.msg = data.try_into().expect("insufficient length");
        msg
    }
}

impl AccountWrite for Message {}
