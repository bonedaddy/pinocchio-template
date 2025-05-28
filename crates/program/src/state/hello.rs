use borsh_derive::{BorshDeserialize, BorshSerialize};

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Message {
    pub msg: Vec<u8>,
}
