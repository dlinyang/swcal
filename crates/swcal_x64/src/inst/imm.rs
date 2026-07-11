use crate::inst::encode::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Imm {
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
    Imm64(u64),
}

impl Imm {
    pub fn width(&self) -> u8 {
        match self {
            Imm::Imm8(_) => 1,
            Imm::Imm16(_) => 2,
            Imm::Imm32(_) => 4,
            Imm::Imm64(_) => 8,
        }
    }

    pub fn is_w64(&self) -> bool {
        self.width() == 8
    }

    pub fn encode(&self, buf: &mut impl CodeSink){
        match self {
            Imm::Imm8(imm) => buf.putb(*imm),
            Imm::Imm16(imm) => buf.putw(*imm),
            Imm::Imm32(imm) => buf.putd(*imm),
            Imm::Imm64(imm) => buf.putq(*imm),
        }
    }
}

impl std::fmt::Display for Imm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Imm::Imm8(v) => write!(f, "byte {}", v),
            Imm::Imm16(v) => write!(f, "word {}", v),
            Imm::Imm32(v) => write!(f, "dword {}", v),
            Imm::Imm64(v) => write!(f, "qword {}", v),
        }
    }
}
