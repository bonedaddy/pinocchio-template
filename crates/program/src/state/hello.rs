
#[repr(C)]
#[derive(Clone)]
pub struct Message {
    pub msg: Vec<u8>,
}

impl Message {
    pub fn unpack(data: &[u8]) -> Self {
        let mut msg = Vec::with_capacity(data.len());
        msg.extend_from_slice(&(data.len() as u32).to_le_bytes());
        msg.extend_from_slice(data);
        Self { msg }
    }
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.msg.len());
        buf.extend_from_slice(&(self.msg.len() as u32).to_le_bytes()[..]);
        buf.extend_from_slice(&self.msg);
        buf
    }
}
