use crate::inst::base::RegEnc;

#[derive(Debug, Clone, Copy)]
pub struct  Gpr {
    id: u8,
}

impl RegEnc for Gpr {
    fn is_extend(&self) -> bool {
        (self.id & 0b1000) == 0b1000
    }

    fn from_id(id: u8) -> Self {
        Self { id }
    }

    fn encode(&self) -> u8 {
        self.id
    }
}

pub struct Fixed<const I: u8> {}


impl<const I: u8> Fixed<I> {
    pub fn new() -> Self { Self {  }}
}

impl<const I: u8> Default for Fixed<I> {
    fn default() -> Self {
        Self {  }
    }
}
