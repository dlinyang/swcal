pub trait RegEnc {
    fn from_id(id: u8) -> Self;
    fn encode(&self) -> u8;
    fn is_extend(&self) -> bool;
}

impl RegEnc for u8 {
    fn from_id(id: u8) -> Self {
        id
    }

    fn encode(&self) -> u8 {
        *self
    }

    fn is_extend(&self) -> bool {
        (self & 0b1000) == 0b1000
    }
}
