use crate::inst::encode::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disp {
    Disp8(i8),
    Disp32(i32),
}

impl Disp {
    /// Size of the displacement in bytes
    pub fn disp_size(&self) -> usize {
        match self {
            Disp::Disp8(_) => 1,
            Disp::Disp32(_) => 4,
        }
    }

    pub fn is_disp8(&self) -> bool {
        matches!(self, Disp::Disp8(_))
    }

    pub fn is_disp32(&self) -> bool {
        matches!(self, Disp::Disp32(_))
    }

    pub fn encode(&self, buf: &mut impl CodeSink) {
        match *self {
            Disp::Disp8(disp) => buf.putb(disp as u8),
            Disp::Disp32(disp) => buf.putd(disp as u32),
        }
    }
}

impl std::fmt::Display for Disp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Disp::Disp8(v) => write!(f, "disp8 {}", v),
            Disp::Disp32(v) => write!(f, "disp32 {}", v),
        }
    }
}
