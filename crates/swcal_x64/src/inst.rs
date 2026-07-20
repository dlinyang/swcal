pub mod encode;
pub mod rex;
pub mod vex;
pub mod operand;
pub mod reg;
pub mod mem;
pub mod imm;
pub mod disp;
pub mod modrm;

use encode::*;
use rex::*;
use reg::*;
use operand::*;
use disp::*;
use modrm::*;

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
    pub fn width_validate<const I: u8, const S: u8, const T: u8>(&self) -> Result<(), String> {
        if let Some(src) = self.src {
            if src.width() == S {
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
    pub fn encode_prefix_legacy(&self, buf: &mut impl CodeSink) -> Result<(), String> {
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
