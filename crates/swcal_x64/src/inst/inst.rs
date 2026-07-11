use super::reg::*;
use super::rex::*;
use super::imm::*;
use super::disp::*;
use super::mem::*;
use super::encode::*;

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
pub fn gen_sib(base: u8, index: u8, scale: u8) -> u8 {
    (scale & 0b11) << 6 | (index & 0b111) << 3 | base & 0b111
}

impl ModRM {
    pub fn new() -> Self {
        Self {
            mode: 0,
            reg: 0,
            rm: 0,
        }
    }

    pub fn from_byte(byte: u8) -> Self {
        ModRM {
            mode: (byte >> 6) & 0b11,
            reg: (byte >> 3) & 0b111,
            rm: byte & 0b111,
        }
    }

    pub fn byte(&self) -> u8 {
        (self.mode << 6) | (self.reg << 3) | self.rm
    }

    pub fn encode(&self, buf: &mut impl CodeSink) {
        buf.putb(self.byte());
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

impl Operand {
    pub fn width(&self) -> u8 {
        match self {
            Operand::Reg(reg) => reg.width(),
            Operand::Mem(mem) => mem.width,
            Operand::Imm(imm) => imm.width(),
        }
    }

    pub fn is_rm(&self) -> bool {
        match self {
            Operand::Mem(_) | Operand::Reg(_) => true,
            _ => false,
        }
    }

    pub fn is_reg(&self) -> bool {
        match self {
            Operand::Reg(_) => true,
            _ => false,
        }
    }

    pub fn is_imm(&self) -> bool {
        match self {
            Operand::Imm(_) => true,
            _ => false,
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

#[inline]
pub fn operand_is_rm(operand_opt: &Option<Operand>) -> bool {
    operand_opt.is_some_and(|x| x.is_rm())
}

#[inline]
pub fn operand_is_reg(operand_opt: &Option<Operand>) -> bool {
    operand_opt.is_some_and(|x| x.is_reg())
}

#[inline]
pub fn operand_is_imm(operand_opt: &Option<Operand>) -> bool {
    operand_opt.is_some_and(|x| x.is_imm())
}

impl Inst {
    pub fn width_validate<const I: u8>(&self) -> Result<(), String> {
        if let Some(src) = self.src {
            if src.width() == I {
                Ok(())
            }
            else {
                Err(format!("unmatched with {} {}", I, self))
            }
        }
        else if let Some(dst) = self.dst {
            if dst.width() == I {
                Ok(())
            }
            else {
                Err(format!("unmatched with {} {}", I, self))
            }
        }
        else {
            Ok(())
        }
    }

    /// Encode x86-64 legacy prefixes (REX, segment overrides, operand size, address size)
    pub fn encode_prefix_lagecy(&self, buf: &mut impl CodeSink) -> Result<(), String> {
        // TODO: Additional prefix handling could be added here for:
        // - Segment overrides (CS, DS, ES, FS, GS, SS) - 0x2E, 0x3E, 0x26, 0x64, 0x65, 0x36
        // - Address size override (0x67)
        // - Lock prefix (0xF0)
        // - REP/REPNE prefixes (0xF3, 0xF2)

        // Operand size override (0x66)
        if let Some(dst) = self.dst {
            if dst.width() == 2 {
                buf.putb(0x66);
            }
        }

        // Rex Prefix
        let mut rex = Rex::new();

        // Check operands for REX requirements
        match self.dst {
            Some(Operand::Reg(reg)) => {
                rex.w = reg.is_w64();
                rex.r = reg.is_extended();
            }
            Some(Operand::Mem(mem)) => {
                if let Some((index, _)) = mem.sib_opt {
                    rex.x = index.is_extended()
                } else {
                    rex.b = mem.reg.is_extended();
                }
            }
            _ => {
                return Err(format!("wrong dst type {}", self));
            }
        }

        match self.src {
            Some(Operand::Reg(reg)) => {
                rex.w = reg.is_w64();
                rex.r = reg.is_extended();
            }
            Some(Operand::Mem(mem)) => {
                if let Some((index, _)) = mem.sib_opt {
                    rex.x = index.is_extended()
                } else {
                    rex.b = mem.reg.is_extended();
                }
            }
            Some(Operand::Imm(imm)) => {
                rex.w = imm.width() == 8;
            }
            _ => {}
        }

        if rex.need() {
            buf.putb(rex.byte());
        }

        Ok(())
    }

    pub fn encode_modrm(&self, buf: &mut impl CodeSink) -> Result<(), String> {
        if let Some(dst) = self.dst && let Some(src) = self.src {
            match (dst, src) {
                (Operand::Reg(dst), Operand::Reg(src)) => {
                    let mut modrm = ModRM::new();
                    modrm.mode = ModRMMode::Reg as u8;
                    modrm.reg = dst.id();
                    modrm.rm = src.id();
                    modrm.encode(buf);
                    Ok(())
                }
                (Operand::Reg(reg), Operand::Mem(mem)) | (Operand::Mem(mem), Operand::Reg(reg)) => {
                    let mut modrm = ModRM::new();
                    // modrm.mode
                    let mode = match mem.disp_opt {
                            Some(Disp::Disp8(_)) => ModRMMode::Disp8,
                            Some(Disp::Disp32(_)) => ModRMMode::Disp32,
                            None => ModRMMode::Mem
                    };
                    modrm.mode = mode as u8;

                    // modrm.reg
                    modrm.reg = reg.id();

                    // modrm.rm
                    if let Some((index, scale)) = &mem.sib_opt {
                        modrm.rm = Reg::RSP.id();
                        modrm.encode(buf);
                        buf.putb(gen_sib(mem.reg.id(), index.id(), *scale));
                    }
                    else {
                        modrm.mode = ModRMMode::Mem as u8;
                        modrm.rm = mem.reg.id();
                        modrm.encode(buf);
                    }

                    // disp
                    match mem.disp_opt {
                        Some(disp) => disp.encode(buf),
                        None => {},
                    }

                    Ok(())
                }
                _ => Err(format!("wrong operand type {}", self)),
            }
        } else {
            Err(format!("instruction wrong operand parameter {}", self))
        }
    }

    pub fn encode_modrm_reg_ext_op<const I: u8>(&self, buf: &mut impl CodeSink) -> Result<(), String> {
        if let Some(dst) = self.dst && let Some(src) = self.src {
            match (dst, src) {
                (Operand::Reg(dst), Operand::Imm(src)) => {
                    let mut modrm = ModRM::new();
                    modrm.mode = ModRMMode::Reg as u8;
                    modrm.reg = I;
                    modrm.rm = dst.id();
                    modrm.encode(buf);
                    src.encode(buf);
                    Ok(())
                },
                (Operand::Mem(dst), Operand::Imm(src)) => {
                    let mut modrm = ModRM::new();
                    // modrm.mode
                    let mode = match dst.disp_opt {
                            Some(Disp::Disp8(_)) => ModRMMode::Disp8,
                            Some(Disp::Disp32(_)) => ModRMMode::Disp32,
                            None => ModRMMode::Mem
                    };
                    modrm.mode = mode as u8;

                    //modrm.reg
                    modrm.reg = I;

                    // modrm.rm
                    if let Some((index, scale)) = &dst.sib_opt {
                        modrm.rm = Reg::RSP.id();
                        modrm.encode(buf);
                        buf.putb(gen_sib(dst.reg.id(), index.id(), *scale));
                    }
                    else {
                        modrm.mode = ModRMMode::Mem as u8;
                        modrm.rm = dst.reg.id();
                        modrm.encode(buf);
                    }

                    // disp
                    match dst.disp_opt {
                        Some(disp) => disp.encode(buf),
                        None => {},
                    }

                    modrm.encode(buf);
                    src.encode(buf);
                    Ok(())
                },
                _ => Err(format!("wrong operand type {}", self)),
            }
        }
        else {
            Err(format!("instruction wrong operand parameter {}", self))
        }
    }

    pub fn encode_reg_enc_op(&self, buf: &mut impl CodeSink) -> Result<(), String> {
        if let Some(dst) = self.dst && let Some(src) = self.src {
            match (dst, src) {
                (Operand::Reg(dst), Operand::Imm(src)) => {
                    buf.modify(|op| {
                        *op = *op | (dst.id() & 0b111);
                    });
                    src.encode(buf);
                    Ok(())
                }
                _ => Err(format!("wrong operand type {}", self)),
            }
        }
        else {
            Err(format!("instruction wrong operand parameter {}", self))
        }
    }
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
