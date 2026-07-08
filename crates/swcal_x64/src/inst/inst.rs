use super::reg::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Imm {
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
    Imm64(u64),
}

impl Imm {
    pub fn imm_size(&self) -> usize {
        match self {
            Imm::Imm8(_) => 1,
            Imm::Imm16(_) => 2,
            Imm::Imm32(_) => 4,
            Imm::Imm64(_) => 8,
        }
    }

    pub fn need_extend(&self) -> bool {
        self.imm_size() == 8
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disp {
    Disp8(i8),
    Disp32(i32),
}

impl std::fmt::Display for Disp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Disp::Disp8(v) => write!(f, "disp8 {}", v),
            Disp::Disp32(v) => write!(f, "disp32 {}", v),
        }
    }
}

/// ModRM.mod field encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModRMMode {
    /// [register] - direct memory, no displacement
    Mem = 0b00,
    /// [register + disp8] - memory with 8-bit displacement
    Disp8 = 0b01,
    /// [register + disp32] - memory with 32-bit displacement
    Disp32 = 0b10,
    /// register (not memory)
    Reg = 0b11,
}

pub struct ModRM {
    pub mode: u8,
    pub reg: u8,
    pub rm: u8,
}

#[inline]
pub fn gen_modrm(mode: u8, reg: u8, rm: u8) -> u8 {
    ((mode & 0b11) << 6) | ((reg & 0b111) << 3) | rm & 0b111
}

#[inline]
pub fn gen_sib(scale: u8, index: u8, base: u8) -> u8 {
    (scale & 0b11) << 6 | (index & 0b111) << 3 | base & 0b111
}

impl ModRM {
    pub fn new(byte: u8) -> Self {
        ModRM {
            mode: (byte >> 6) & 0b11,
            reg: (byte >> 3) & 0b111,
            rm: byte & 0b111,
        }
    }

    pub fn encode(&self) -> u8 {
        (self.mode << 6) | (self.reg << 3) | self.rm
    }
}

/// width \[reg + (index * scale)? + (disp)?\]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mem {
    /// width-byte
    pub width: u8,
    pub reg: Reg,
    pub sib_opt: Option<(Reg, u8)>,
    pub disp_opt: Option<Disp>,
}

impl Mem {
    pub fn is_extend(&self) -> bool {
        self.reg.is_extended()
    }

    pub fn need_rex(&self) -> bool {
        self.reg.needs_rex()
    }

    pub fn check_reg_valid(&self) -> bool {
        if let Some((index, _)) = self.sib_opt {
            index.is_extended() == self.reg.is_extended()
        } else {
            true
        }
    }
}

impl std::fmt::Display for Mem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.width {
            1 => write!(f, "byte")?,
            2 => write!(f, "word")?,
            4 => write!(f, "dword")?,
            8 => write!(f, "qword")?,
            _ => write!(f, "{}bit", self.width * 8)?,
        }
        write!(f, " [")?;
        write!(f, "{}", self.reg)?;
        if let Some((index, scale)) = self.sib_opt {
            write!(f, "+{index}*{scale}")?;
        }
        if let Some(disp) = self.disp_opt {
            write!(f, "+{disp}")?;
        }
        write!(f, "]")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Reg(Reg),
    Mem(Mem),
    Imm(Imm),
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(reg) => write!(f, "{}", reg),
            Operand::Mem(rm) => write!(f, "{}", rm),
            Operand::Imm(imm) => write!(f, "{}", imm),
        }
    }
}

#[derive(Debug)]
pub struct Inst {
    pub mnemonic: String,
    pub dst: Option<Operand>,
    pub src: Option<Operand>,
    pub src_ext: Option<Operand>,
}

impl std::fmt::Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.mnemonic)?;
        if let Some(dst) = self.dst {
            write!(f, " {}", dst)?;
        }
        if let Some(src) = self.src {
            write!(f, ", {}", src)?;
        }
        if let Some(src_ext) = self.src_ext {
            write!(f, ", {}", src_ext)?;
        }
        Ok(())
    }
}
