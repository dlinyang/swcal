use crate::inst::rex::Rex;

// use super::reg::*;
use super::inst::*;
use super::format::*;

impl InstFormat {

    pub fn encode(&self, inst: &Inst) -> Result<BinInst, String> {
        let mut bin = BinInst::new();
        // check operand kind match inst.dst inst.src inst.src_ext
        match self.operand_kind {
            OperandKind::NoOperand => {
                if inst.dst == None && inst.src == None && inst.src_ext == None {
                    self.push_opcode(&mut bin);
                    Ok(bin)
                }
                else {
                    Err(format!("unmatch operand {}", inst))
                }
            },
            OperandKind::RM => {
                if let Some(_dst) = inst.dst && inst.src == None && inst.src_ext == None {
                    todo!()
                }
                else {
                    Err(format!("unmatch operand {}", inst))
                }
            },
            OperandKind::Imm2RM => {
                if let Some(_dst) = inst.dst && let Some(_src) = inst.src && inst.src_ext == None {
                    // prefix check
                    todo!();

                    // encode rm
                    // if let Operand::Reg(reg) = dst {
                    // };

                    // encode imm
                    // if let Operand::Imm(imm) = src {
                    //     encode_imm(&mut bin, &imm);
                    //     return Ok(bin);
                    // }

                    // Err(format!("unmatch operand {}", inst))
                }
                else {
                    Err(format!("unmatch operand {}", inst))
                }
            },
            OperandKind::Reg2RM => {
                if let Some(_dst) = inst.dst && let Some(_src) = inst.src && inst.src_ext == None {
                    todo!()
                }
                else {
                    Err(format!("unmatch operand {}", inst))
                }
            },
            OperandKind::RM2Reg => {
                if let Some(_dst) = inst.dst && let Some(_src) = inst.src && inst.src_ext == None {
                    todo!()
                }
                else {
                    Err(format!("unmatch operand {}", inst))
                }
            },
            OperandKind::Reg2Reg => {
                if let Some(_dst) = inst.dst && let Some(_src) = inst.src && inst.src_ext == None {
                    todo!()
                }
                else {
                    Err(format!("unmatch operand {}", inst))
                }
            },
        }
    }

    pub fn encode_prefix(&self, inst: &Inst, bin: &mut BinInst) {
        match self.prefix {
            Prefix::Legacy => {
                // Encode instruction prefix bytes (e.g., REX, operand-size override, etc.)
                // based on the instruction format and operand sizes.
                // REX prefix for 64-bit operands or extended registers
                let mut rex = Rex::new();

                if let Some(dst) = &inst.dst {
                    if let Operand::Reg(reg) = dst {
                        // need check reg is in modrm or sib or op/r
                        rex.r = reg.is_extended();
                    }
                }
                if let Some(src) = &inst.src {
                    if let Operand::Reg(reg) = src {
                        rex.r &= reg.is_extended();
                    }
                    if let Operand::Mem(mem) = src {
                        rex.r &= mem.reg.is_extended();
                    }
                }

                if let Some(_src) = &inst.src_ext {
                    todo!()
                }

                // Check for 64-bit operand size requiring REX.W
                rex.w = self.operand_size == 8;

                if rex.need() {
                    bin.push(rex.byte());
                }

                // Operand-size override prefix (0x66) for 16-bit operations
                // if self.operand_size == 2 {
                //     bin.push(0x66);
                // }
                // Address-size override prefix (0x67) for 16/32-bit addressing
                // if self. {
                //     bin.push(0x67);
                // }
            },
            Prefix::EvexVex => todo!(),
        }
    }

    fn push_opcode(&self, bin: &mut BinInst) {
        bin.push(self.opcode.fst);
        if let Some(byte) = self.opcode.snd {
            bin.push(byte);
        }
        if let Some(byte) = self.opcode.trd {
            bin.push(byte);
        }
    }
}

// /// 编码 ModRM 字节（带 r/m 操作数）
// fn encode_modrm_with_rm(result: &mut BinInst, reg: &Reg, rm: &Mem) {
//     let reg_id = reg.id();
//     match rm {
//         Mem::Reg(rm) => {
//             result.push(gen_modrm(ModRMMode::Reg as u8, reg_id, rm.id()));
//         }
//         Mem::AddrReg(_, base_reg) => {
//             // [reg] 仅寄存器间接寻址
//             let rm_id = base_reg.id();
//             if *base_reg == Reg::RSP || *base_reg == Reg::R12 {
//                 // RSP/R12 需要 SIB 字节
//                 let modrm: u8 = 0b00_000_000 | (reg_id << 3) | 0b100;
//                 push_byte(result, modrm);
//                 // SIB: base=RSP(index=RSP means none), scale=0, index=RSP(means none)
//                 let sib: u8 = 0b00_100_100;
//                 push_byte(result, sib);
//             } else if *base_reg == Reg::RBP || *base_reg == Reg::R13 {
//                 // RBP/R13 需要 disp8=0
//                 let modrm: u8 = 0b01_000_000 | (reg_id << 3) | rm_id;
//                 push_byte(result, modrm);
//                 push_byte(result, 0u8); // disp8=0
//             } else {
//                 let modrm: u8 = 0b00_000_000 | (reg_id << 3) | rm_id;
//                 push_byte(result, modrm);
//             }
//         }
//         Mem::AddrRegDisp(_, base_reg, disp) => {
//             // [reg+disp]
//             let rm_id = base_reg.id();
//             let (mod_val, disp_bytes): (u8, Vec<u8>) = match disp {
//                 Imm::Imm8(val) => {
//                     let signed = *val as i8;
//                     if signed == 0 && *base_reg != Reg::RBP && *base_reg != Reg::R13 {
//                         (0u8, vec![])
//                     } else {
//                         (1u8, vec![*val])
//                     }
//                 }
//                 Imm::Imm16(val) => {
//                     let signed = *val as i16;
//                     if *base_reg == Reg::RBP || *base_reg == Reg::R13 {
//                         if signed >= -128 && signed <= 127 {
//                             (1u8, vec![*val as u8])
//                         } else {
//                             (2u8, val.to_le_bytes().to_vec())
//                         }
//                     } else {
//                         (2u8, val.to_le_bytes().to_vec())
//                     }
//                 }
//                 Imm::Imm32(val) => {
//                     let signed = *val as i32;
//                     if *base_reg == Reg::RBP || *base_reg == Reg::R13 {
//                         if signed >= -128 && signed <= 127 {
//                             (1u8, vec![*val as u8])
//                         } else {
//                             (2u8, val.to_le_bytes().to_vec())
//                         }
//                     } else {
//                         if signed >= -128 && signed <= 127 && signed != 0 {
//                             (1u8, vec![*val as u8])
//                         } else if signed == 0 {
//                             (0u8, vec![])
//                         } else {
//                             (2u8, val.to_le_bytes().to_vec())
//                         }
//                     }
//                 }
//                 Imm::Imm64(_val) => {
//                     panic!("64-bit displacement not supported");
//                 }
//             };

//             if *base_reg == Reg::RSP || *base_reg == Reg::R12 {
//                 // RSP/R12 需要 SIB 字节
//                 let modrm: u8 = (mod_val << 6) | (reg_id << 3) | 0b100;
//                 push_byte(result, modrm);
//                 // SIB: base=RSP(index=RSP means none), scale=0, index=RSP(means none)
//                 let sib: u8 = 0b00_100_100;
//                 push_byte(result, sib);
//             } else {
//                 let modrm: u8 = (mod_val << 6) | (reg_id << 3) | rm_id;
//                 push_byte(result, modrm);
//             }

//             // 写入 displacement 字节
//             for b in disp_bytes {
//                 push_byte(result, b);
//             }
//         }
//         Mem::AddrSIB(_, base, index, scale) => {
//             // [base + index * scale]
//             let base_id = base.id();
//             let index_id = index.id();
//             let scale_enc = match scale {
//                 1 => 0u8,
//                 2 => 1u8,
//                 4 => 2u8,
//                 8 => 3u8,
//                 _ => panic!("Invalid scale factor: {}", scale),
//             };

//             let modrm: u8 = 0b00_000_000 | (reg_id << 3) | 0b100;
//             push_byte(result, modrm);
//             // SIB: scale(2) + index(3) + base(3)
//             let sib: u8 = (scale_enc << 6) | (index_id << 3) | base_id;
//             push_byte(result, sib);
//         }
//     }
// }

// fn encode_modrm(result: &mut BinInst, reg: u8, rm: &Mem) {
//     match rm {
//         Mem::Reg(rm) => {
//             result.push(gen_modrm(ModRMMode::Reg as u8, reg, rm.id()));
//         }
//         Mem::AddrReg(addr_width, rm) => {
//             result.push(gen_modrm(ModRMMode::Mem as u8, reg, rm.id()));
//         },
//         Mem::AddrRegDisp(addr_width, rm, imm) => {
//             match imm {
//                 Imm::Imm8(disp8) => {
//                     result.push(gen_modrm(ModRMMode::Disp8 as u8, reg, rm.id()));
//                     result.push(*disp8);
//                 },
//                 Imm::Imm32(disp32) => {
//                     result.push(gen_modrm(ModRMMode::Disp32 as u8, reg, rm.id()));
//                     let bytes = disp32.to_le_bytes();
//                     for b in bytes {
//                         result.push(b);
//                     }
//                 },
//                 Imm::Imm16(_) | Imm::Imm64(_) => todo!(),
//             }
//         },
//         Mem::AddrSIB(addr_width, base, index, scale) => {
//             // modrm
//             // TODO: Disp support in modrm sib disp
//             result.push(gen_modrm(ModRMMode::Mem as u8, reg, Reg::RSI.id()));
//             //sib
//             result.push(gen_sib(base.id(), index.id(), *scale));
//         },
//     }
// }

/// 编码立即数
fn _encode_imm(result: &mut BinInst, imm: &Imm) {
    match imm {
        Imm::Imm8(val) => {
            result.push(*val);
        }
        Imm::Imm16(val) => {
            let bytes = val.to_le_bytes();
            for b in &bytes {
                result.push(*b);
            }
        }
        Imm::Imm32(val) => {
            let bytes = val.to_le_bytes();
            for b in &bytes {
                result.push(*b);
            }
        }
        Imm::Imm64(val) => {
            let bytes = val.to_le_bytes();
            for b in &bytes {
                result.push(*b);
            }
        }
    }
}

/// 编码一条指令为 BinInst
pub fn encode_instruction(inst: &Inst, format: &InstFormat) -> BinInst {
    format.encode(inst).unwrap_or_else(|e| {
        panic!("Failed to encode instruction: {}", e);
    })
}
