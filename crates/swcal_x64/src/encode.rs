use super::format::*;
use super::inst::*;

/// 将指令编码为二进制字节序列
/// 验证指令与指令格式是否相符, 并生成指令
/// 返回 Ok(BinInst) 如果相符，否则返回 Err 包含描述信息
pub fn encode(inst: &Inst, format: &InstFormat) -> Result<BinInst, String> {
    // 验证助记符是否匹配
    if inst.mnemonic != format.mnemonic {
        return Err(format!(
            "unmatch mnemonic {} {}", inst.mnemonic, format.mnemonic
        ));
    }

    // 1. 检查操作数种类是否匹配
    let inst_kind = operand_kind_from_inst(inst);
    if inst_kind != format.operand_kind {
        return Err(format!(
            "指令操作数种类不匹配: 指令为 {:?}, 格式需要 {:?}",
            inst_kind, format.operand_kind
        ));
    }

    // 2. 检查操作数中的寄存器是否有效
    match format.prefix {
        Prefix::Legacy => {
            // Legacy 前缀下，检查是否需要 REX 前缀
            let mut has_ext_reg = false;
            match &inst.operand {
                Operand::Zero => {}
                Operand::Imm2Reg { reg, .. } => {
                    if reg.is_extended() || reg.needs_rex() {
                        has_ext_reg = true;
                    }
                }
                Operand::Reg2RM { reg, rm } => {
                    if reg.is_extended() || reg.needs_rex() {
                        has_ext_reg = true;
                    }
                    if let Some(base_reg) = get_rm_base_reg(rm) {
                        if base_reg.is_extended() || base_reg.needs_rex() {
                            has_ext_reg = true;
                        }
                    }
                }
                Operand::RM2Reg { reg, rm } => {
                    if reg.is_extended() || reg.needs_rex() {
                        has_ext_reg = true;
                    }
                    if let Some(base_reg) = get_rm_base_reg(rm) {
                        if base_reg.is_extended() || base_reg.needs_rex() {
                            has_ext_reg = true;
                        }
                    }
                }
                Operand::Reg2Reg { src_reg, dst_reg } => {
                    if src_reg.is_extended() || src_reg.needs_rex() {
                        has_ext_reg = true;
                    }
                    if dst_reg.is_extended() || dst_reg.needs_rex() {
                        has_ext_reg = true;
                    }
                }
            }
            let _ = has_ext_reg;
        }
        Prefix::EvexVex => {
            // VEX/EVEX 前缀下，检查操作数寄存器是否合规
            // 目前暂未实现详细检查
        }
    }

    // 3. 检查立即数宽度是否与操作码期望一致
    match &inst.operand {
        Operand::Imm2Reg { imm, .. } => {
            let imm_size = imm_size(imm);
            if imm_size > (format.operand_size as usize / 8) {
                return Err(format!(
                    "立即数宽度 ({}) 超过操作数宽度 ({})",
                    imm_size * 8,
                    format.operand_size
                ));
            }
        }
        _ => {}
    }

    // 4. 构建指令编码
    let mut result = BinInst::new();

    // 编码前缀（Legacy 或 EVEX/VEX）
    match format.prefix {
        Prefix::Legacy => {
            let mut rex: u8 = 0b0100_0000;
            if format.operand_size == 64 {
                rex |= 0b1000; // W=1
            }

            // 检查是否需要 REX.B (扩展寄存器，如 r8-r15)
            let has_ext_base = match &inst.operand {
                Operand::Reg2RM { rm, .. } | Operand::RM2Reg { rm, .. } => {
                    get_rm_base_reg(rm).map_or(false, |r| r.is_extended())
                }
                Operand::Reg2Reg { dst_reg, .. } => dst_reg.is_extended(),
                Operand::Imm2Reg { reg, .. } => reg.is_extended(),
                _ => false,
            };
            if has_ext_base {
                rex |= 0b0001; // B=1
            }

            let has_ext_reg = match &inst.operand {
                Operand::Reg2RM { reg, .. } | Operand::RM2Reg { reg, .. } => reg.is_extended(),
                Operand::Reg2Reg { src_reg, .. } => src_reg.is_extended(),
                Operand::Imm2Reg { reg, .. } => reg.is_extended(),
                _ => false,
            };
            if has_ext_reg {
                rex |= 0b0010; // R=1
            }

            // 检查 SIB index 寄存器扩展位
            let has_ext_index = match &inst.operand {
                Operand::Reg2RM { rm, .. } | Operand::RM2Reg { rm, .. } => match rm {
                    RM::AddrSIB(_, index, _) => index.is_extended(),
                    _ => false,
                },
                _ => false,
            };
            if has_ext_index {
                rex |= 0b0100; // X=1
            }

            if rex != 0b0100_0000 {
                push_byte(&mut result, rex);
            }
        }
        Prefix::EvexVex => {
            return Err("VEX/EVEX prefix encoding not yet implemented".to_string());
        }
    }

    // 编码操作码
    let op = &format.opcode;
    push_byte(&mut result, op.fst);
    if let Some(snd) = op.snd {
        push_byte(&mut result, snd);
    }
    if let Some(trd) = op.trd {
        push_byte(&mut result, trd);
    }

    // 编码操作数
    match &inst.operand {
        Operand::Zero => {
            // 无操作数，无需额外编码
        }
        Operand::Imm2Reg { reg, imm } => {
            // 立即数到寄存器：opcode + ModRM
            let modrm: u8 = 0b11_000_000 | (reg.id() << 3) | reg.id();
            push_byte(&mut result, modrm);
            encode_imm(&mut result, imm);
        }
        Operand::Reg2RM { reg, rm } => {
            // 寄存器到内存/寄存器：opcode + ModRM + [SIB] + [disp]
            encode_modrm_with_rm(&mut result, reg, rm);
        }
        Operand::RM2Reg { reg, rm } => {
            // 内存/寄存器到寄存器：opcode + ModRM + [SIB] + [disp]
            encode_modrm_with_rm(&mut result, reg, rm);
        }
        Operand::Reg2Reg { src_reg, dst_reg } => {
            // 寄存器到寄存器：opcode + ModRM
            let modrm: u8 = 0b11_000_000 | (src_reg.id() << 3) | dst_reg.id();
            push_byte(&mut result, modrm);
        }
    }

    Ok(result)
}

/// 从 Inst 的操作数推导出 OperandKind
fn operand_kind_from_inst(inst: &Inst) -> OperandKind {
    match &inst.operand {
        Operand::Zero => OperandKind::ZeroOprand,
        Operand::Imm2Reg { .. } => OperandKind::Reg2Imm,
        Operand::Reg2RM { .. } => OperandKind::Reg2RM,
        Operand::RM2Reg { .. } => OperandKind::RM2Reg,
        Operand::Reg2Reg { .. } => OperandKind::Reg2Reg,
    }
}

/// 从 RM 中获取基址寄存器（如果存在）
fn get_rm_base_reg(rm: &RM) -> Option<Reg> {
    match rm {
        RM::AddrReg(base) => Some(*base),
        RM::AddrRegDisp(base, _) => Some(*base),
        RM::AddrSIB(base, _, _) => Some(*base),
    }
}

/// 获取立即数的字节宽度
fn imm_size(imm: &Imm) -> usize {
    match imm {
        Imm::Imm8(_) => 1,
        Imm::Imm16(_) => 2,
        Imm::Imm32(_) => 4,
        Imm::Imm64(_) => 8,
    }
}

/// 向 BinInst 中压入一个字节
fn push_byte(result: &mut BinInst, byte: u8) {
    if (result.len as usize) < result.data.len() {
        result.data[result.len as usize] = byte;
        result.len += 1;
    } else {
        panic!("Instruction too long (max 15 bytes)");
    }
}

/// 编码 ModRM 字节（带 r/m 操作数）
fn encode_modrm_with_rm(result: &mut BinInst, reg: &Reg, rm: &RM) {
    let reg_id = reg.id();
    match rm {
        RM::AddrReg(base_reg) => {
            // [reg] 仅寄存器间接寻址
            let rm_id = base_reg.id();
            if *base_reg == Reg::RSP || *base_reg == Reg::R12 {
                // RSP/R12 需要 SIB 字节
                let modrm: u8 = 0b00_000_000 | (reg_id << 3) | 0b100;
                push_byte(result, modrm);
                // SIB: base=RSP(index=RSP means none), scale=0, index=RSP(means none)
                let sib: u8 = 0b00_100_100;
                push_byte(result, sib);
            } else if *base_reg == Reg::RBP || *base_reg == Reg::R13 {
                // RBP/R13 需要 disp8=0
                let modrm: u8 = 0b01_000_000 | (reg_id << 3) | rm_id;
                push_byte(result, modrm);
                push_byte(result, 0u8); // disp8=0
            } else {
                let modrm: u8 = 0b00_000_000 | (reg_id << 3) | rm_id;
                push_byte(result, modrm);
            }
        }
        RM::AddrRegDisp(base_reg, disp) => {
            // [reg+disp]
            let rm_id = base_reg.id();
            let (mod_val, disp_bytes): (u8, Vec<u8>) = match disp {
                Imm::Imm8(val) => {
                    let signed = *val as i8;
                    if signed == 0 && *base_reg != Reg::RBP && *base_reg != Reg::R13 {
                        (0u8, vec![])
                    } else {
                        (1u8, vec![*val])
                    }
                }
                Imm::Imm16(val) => {
                    let signed = *val as i16;
                    if *base_reg == Reg::RBP || *base_reg == Reg::R13 {
                        if signed >= -128 && signed <= 127 {
                            (1u8, vec![*val as u8])
                        } else {
                            (2u8, val.to_le_bytes().to_vec())
                        }
                    } else {
                        (2u8, val.to_le_bytes().to_vec())
                    }
                }
                Imm::Imm32(val) => {
                    let signed = *val as i32;
                    if *base_reg == Reg::RBP || *base_reg == Reg::R13 {
                        if signed >= -128 && signed <= 127 {
                            (1u8, vec![*val as u8])
                        } else {
                            (2u8, val.to_le_bytes().to_vec())
                        }
                    } else {
                        if signed >= -128 && signed <= 127 && signed != 0 {
                            (1u8, vec![*val as u8])
                        } else if signed == 0 {
                            (0u8, vec![])
                        } else {
                            (2u8, val.to_le_bytes().to_vec())
                        }
                    }
                }
                Imm::Imm64(_val) => {
                    panic!("64-bit displacement not supported");
                }
            };

            if *base_reg == Reg::RSP || *base_reg == Reg::R12 {
                // RSP/R12 需要 SIB 字节
                let modrm: u8 = (mod_val << 6) | (reg_id << 3) | 0b100;
                push_byte(result, modrm);
                // SIB: base=RSP(index=RSP means none), scale=0, index=RSP(means none)
                let sib: u8 = 0b00_100_100;
                push_byte(result, sib);
            } else {
                let modrm: u8 = (mod_val << 6) | (reg_id << 3) | rm_id;
                push_byte(result, modrm);
            }

            // 写入 displacement 字节
            for b in disp_bytes {
                push_byte(result, b);
            }
        }
        RM::AddrSIB(base, index, scale) => {
            // [base + index * scale]
            let base_id = base.id();
            let index_id = index.id();
            let scale_enc = match scale {
                1 => 0u8,
                2 => 1u8,
                4 => 2u8,
                8 => 3u8,
                _ => panic!("Invalid scale factor: {}", scale),
            };

            let modrm: u8 = 0b00_000_000 | (reg_id << 3) | 0b100;
            push_byte(result, modrm);
            // SIB: scale(2) + index(3) + base(3)
            let sib: u8 = (scale_enc << 6) | (index_id << 3) | base_id;
            push_byte(result, sib);
        }
    }
}

/// 编码立即数
fn encode_imm(result: &mut BinInst, imm: &Imm) {
    match imm {
        Imm::Imm8(val) => {
            push_byte(result, *val);
        }
        Imm::Imm16(val) => {
            let bytes = val.to_le_bytes();
            for b in &bytes {
                push_byte(result, *b);
            }
        }
        Imm::Imm32(val) => {
            let bytes = val.to_le_bytes();
            for b in &bytes {
                push_byte(result, *b);
            }
        }
        Imm::Imm64(val) => {
            let bytes = val.to_le_bytes();
            for b in &bytes {
                push_byte(result, *b);
            }
        }
    }
}

/// 编码一条指令为 BinInst
pub fn encode_instruction(inst: &Inst, format: &InstFormat) -> BinInst {
    encode(inst, format).unwrap_or_else(|e| {
        panic!("Failed to encode instruction: {}", e);
    })
}
// /// 验证指令与指令格式是否相符, 并生成指令
// /// 返回 Ok(BinInst) 如果相符，否则返回 Err 包含描述信息
// pub fn encode(inst: &Inst, format: &InstFormat) -> Result<BinInst, String> {
//     //
//     if inst.mnemonic != format.mnemonic {
//         return Err(format!(
//             "unmatch mnemonic {} {}", inst.mnemonic, format.mnemonic
//         ));
//     }

//     // 1. 检查操作数种类是否匹配
//     let inst_kind = operand_kind_from_inst(inst);
//     if inst_kind != format.operand_kind {
//         return Err(format!(
//             "指令操作数种类不匹配: 指令为 {:?}, 格式需要 {:?}",
//             inst_kind, format.operand_kind
//         ));
//     }

//     // 2. 检查操作数中的寄存器是否有效（如果需要 REX 但格式不支持等）
//     // 对于 Legacy 前缀，检查是否需要 REX 前缀（由寄存器决定）
//     match format.prefix {
//         Prefix::Legacy => {
//             // Legacy 前缀下，检查涉及的寄存器是否需要 REX
//             let mut has_ext_reg = false;
//             match &inst.operand {
//                 Operand::Zero => {}
//                 Operand::Imm2Reg { reg, .. } => {
//                     if reg.is_extended() || reg.needs_rex() {
//                         has_ext_reg = true;
//                     }
//                 }
//                 Operand::Reg2RM { reg, rm } => {
//                     if reg.is_extended() || reg.needs_rex() {
//                         has_ext_reg = true;
//                     }
//                     if let Some(base_reg) = get_rm_base_reg(rm) {
//                         if base_reg.is_extended() || base_reg.needs_rex() {
//                             has_ext_reg = true;
//                         }
//                     }
//                 }
//                 Operand::RM2Reg { reg, rm } => {
//                     if reg.is_extended() || reg.needs_rex() {
//                         has_ext_reg = true;
//                     }
//                     if let Some(base_reg) = get_rm_base_reg(rm) {
//                         if base_reg.is_extended() || base_reg.needs_rex() {
//                             has_ext_reg = true;
//                         }
//                     }
//                 }
//                 Operand::Reg2Reg { src_reg, dst_reg } => {
//                     if src_reg.is_extended() || src_reg.needs_rex() {
//                         has_ext_reg = true;
//                     }
//                     if dst_reg.is_extended() || dst_reg.needs_rex() {
//                         has_ext_reg = true;
//                     }
//                 }
//             }
//             // Legacy 前缀支持扩展寄存器（通过 REX 前缀），所以没问题
//             let _ = has_ext_reg;
//         }
//         Prefix::EvexVex => {
//             // VEX/EVEX 前缀下，检查操作数寄存器是否合规
//             // 目前暂未实现详细检查
//         }
//     }

//     // 3. 检查立即数宽度是否与操作码期望一致（如果有的话）
//     match &inst.operand {
//         Operand::Imm2Reg { imm, .. } => {
//             let imm_size = imm_size(imm);
//             // 根据 format 的 operand_size 做一些合理性检查
//             if imm_size > (format.operand_size as usize / 8) {
//                 return Err(format!(
//                     "立即数宽度 ({}) 超过操作数宽度 ({})",
//                     imm_size * 8,
//                     format.operand_size
//                 ));
//             }
//         }
//         _ => {}
//     }

//     Ok(())
// }

// /// 从 Inst 的操作数推导出 OperandKind
// fn operand_kind_from_inst(inst: &Inst) -> OperandKind {
//     match &inst.operand {
//         Operand::Zero => OperandKind::ZeroOprand,
//         Operand::Imm2Reg { .. } => OperandKind::Reg2Imm,
//         Operand::Reg2RM { .. } => OperandKind::Reg2RM,
//         Operand::RM2Reg { .. } => OperandKind::RM2Reg,
//         Operand::Reg2Reg { .. } => OperandKind::Reg2Reg,
//     }
// }

// /// 从 RM 中获取基址寄存器（如果存在）
// fn get_rm_base_reg(rm: &RM) -> Option<Reg> {
//     match rm {
//         RM::AddrReg(base) => Some(*base),
//         RM::AddrRegDisp(base, _) => Some(*base),
//         RM::AddrSIB(base, _, _) => Some(*base),
//     }
// }

// /// 获取立即数的字节宽度
// fn imm_size(imm: &Imm) -> usize {
//     match imm {
//         Imm::Imm8(_) => 1,
//         Imm::Imm16(_) => 2,
//         Imm::Imm32(_) => 4,
//         Imm::Imm64(_) => 8,
//     }
// }
// /// 编码器结构体，用于将指令编码为二进制字节序列
// pub struct Encoder {
//     pub result: BinInst,
// }

// impl Encoder {
//     pub fn new() -> Self {
//         Self {
//             result: BinInst::new(),
//         }
//     }

//     /// 编码一条指令
//     pub fn encode(&mut self, inst: &Inst, format: &InstFormat) -> &BinInst {
//         self.result = BinInst::new();
//         self.encode_prefix(inst, format);
//         self.encode_opcode(format);
//         self.encode_operands(inst, format);
//         &self.result
//     }

//     /// 编码前缀（Legacy 或 EVEX/VEX）
//     fn encode_prefix(&mut self, inst: &Inst, format: &InstFormat) {
//         match format.prefix {
//             Prefix::Legacy => {
//                 // 对于 Legacy 前缀，目前无需额外操作

//                 // 如果需要 REX 前缀，则稍后在 encode_operands 中处理
//                 // 对于 Legacy 前缀，处理 REX 前缀
//                 // 如果需要 REX 前缀（64位操作数或扩展寄存器）
//                 // REX 字节格式: 0100 WRXB
//                 // W: 0=32位操作数, 1=64位操作数
//                 // R: ModRM.reg 扩展位 (第4位)
//                 // X: SIB.index 扩展位 (第4位)
//                 // B: ModRM.r/m, SIB.base, or opcode.reg 扩展位 (第4位)
//                 let mut rex: u8 = 0b0100_0000;

//                 // 检查是否需要 REX.W (64位操作数)
//                 // 这里通过 format 中的某些属性判断，例如操作数大小
//                 // 默认假设需要 REX.W，具体判断逻辑根据实际情况调整
//                 if format.operand_size == 64 {
//                     rex |= 0b1000; // W=1
//                 }

//                 // 检查是否需要 REX.B (扩展寄存器，如 r8-r15)
//                 // 通过 inst 中的操作数判断
//                 // 这里简化处理，实际需要遍历操作数检查寄存器是否 >= 8
//                 if let Operand::Reg2Reg { src_reg, .. }
//                 | Operand::Reg2RM { reg: src_reg, .. }
//                 | Operand::RM2Reg { reg: src_reg, .. }
//                 | Operand::Imm2Reg { reg: src_reg, .. } = &inst.operand
//                 {
//                     // 检查目标寄存器是否需要 REX.B (r/m 字段扩展)
//                     if src_reg.is_extended() {
//                         rex |= 0b0001; // B=1
//                     }
//                     // // 检查源寄存器是否需要 REX.R (reg 字段扩展)
//                     // if dst_reg.is_extended() {
//                     //     rex |= 0b0010; // R=1
//                     // }
//                 }

//                 // 如果 REX 不是默认值 0x40，则推送 REX 前缀
//                 if rex != 0b0100_0000 {
//                     self.result.push(rex);
//                 }
//             }
//             Prefix::EvexVex => {
//                 // VEX/EVEX 前缀暂未实现
//                 unimplemented!("VEX/EVEX prefix encoding not yet implemented");
//             }
//         }
//     }

//     /// 编码操作码
//     fn encode_opcode(&mut self, format: &InstFormat) {
//         let op = &format.opcode;
//         self.result.push(op.fst);
//         if let Some(snd) = op.snd {
//             self.result.push(snd);
//         }
//         if let Some(trd) = op.trd {
//             self.result.push(trd);
//         }
//     }

//     /// 编码操作数
//     fn encode_operands(&mut self, inst: &Inst, format: &InstFormat) {
//         match &inst.operand {
//             Operand::Zero => {
//                 // 无操作数，无需额外编码
//             }
//             Operand::Imm2Reg { reg, imm } => {
//                 // 立即数到寄存器：opcode + ModRM
//                 self.encode_modrm_reg_only(reg);
//                 self.encode_imm(imm);
//             }
//             Operand::Reg2RM { reg, rm } => {
//                 // 寄存器到内存/寄存器：opcode + ModRM + [SIB] + [disp]
//                 self.encode_modrm_with_rm(reg, rm);
//             }
//             Operand::RM2Reg { reg, rm } => {
//                 // 内存/寄存器到寄存器：opcode + ModRM + [SIB] + [disp]
//                 self.encode_modrm_with_rm(reg, rm);
//             }
//             Operand::Reg2Reg { src_reg, dst_reg } => {
//                 // 寄存器到寄存器：opcode + ModRM
//                 self.encode_modrm_reg_reg(src_reg, dst_reg);
//             }
//         }
//     }

//     /// 编码 ModRM 字节（寄存器到寄存器模式: reg=reg, r/m=reg）
//     fn encode_modrm_reg_reg(&mut self, reg: Reg, rm: RM) {
//         // ModRM = mod(2bits) + reg(3bits) + r/m(3bits)
//         // mod = 0b11 (寄存器直接模式)
//         let modrm: u8 = 0b11_000_000 | (reg << 3) | rm;
//         self.result.push(modrm);
//     }

//     /// 编码 ModRM 字节（仅寄存器操作数 reg 字段，r/m 由 Encoder 内部处理为 ...）
//     fn encode_modrm_reg_only(&mut self, reg: u8) {
//         // 对于 Imm2Reg 之类的指令，r/m 字段为 reg 字段自身（即目标操作数也是寄存器）
//         let modrm: u8 = 0b11_000_000 | (reg << 3) | reg;
//         self.result.push(modrm);
//     }

//     /// 编码 ModRM 字节（带 r/m 操作数）
//     fn encode_modrm_with_rm(&mut self, reg: u8, rm: &RM) {
//         match rm {
//             RM::AddrReg(base_reg) => {
//                 // [reg] 仅寄存器间接寻址
//                 let reg_id = (*base_reg as u8) & 0x07;
//                 if *base_reg == Reg::RSP || *base_reg == Reg::R12 {
//                     // RSP/R12 需要 SIB 字节
//                     let modrm: u8 = 0b00_000_000 | (reg << 3) | 0b100;
//                     self.result.push(modrm);
//                     // SIB: base=RSP(index=RSP means none), scale=0, index=RSP(means none)
//                     let sib: u8 = 0b00_100_100; // scale=0, index=RSP(无), base=RSP
//                     self.result.push(sib);
//                 } else if *base_reg == Reg::RBP || *base_reg == Reg::R13 {
//                     // RBP/R13 需要 disp8=0 或 disp32=0（这里使用 disp8=0）
//                     let modrm: u8 = 0b01_000_000 | (reg << 3) | reg_id;
//                     self.result.push(modrm);
//                     self.result.push(0u8); // disp8=0
//                 } else {
//                     let modrm: u8 = 0b00_000_000 | (reg << 3) | reg_id;
//                     self.result.push(modrm);
//                 }
//             }
//             RM::AddrRegDisp(base_reg, disp) => {
//                 // [reg+disp]
//                 let reg_id = (*base_reg as u8) & 0x07;
//                 let (mod_val, disp_bytes) = match disp {
//                     Imm::Imm8(val) => {
//                         let signed = *val as i8;
//                         if signed == 0 && *base_reg != Reg::RBP && *base_reg != Reg::R13 {
//                             (0u8, vec![])
//                         } else {
//                             (1u8, vec![*val])
//                         }
//                     }
//                     Imm::Imm16(val) => {
//                         // x86_64 中 16位 displacement 不常见，通常为 32位
//                         let signed = *val as i16;
//                         if *base_reg == Reg::RBP || *base_reg == Reg::R13 {
//                             // RBP/R13 强制使用至少 disp8
//                             if (signed >= -128 && signed <= 127) {
//                                 (1u8, vec![*val as u8])
//                             } else {
//                                 (2u8, val.to_le_bytes().to_vec())
//                             }
//                         } else {
//                             (2u8, val.to_le_bytes().to_vec())
//                         }
//                     }
//                     Imm::Imm32(val) => {
//                         let signed = *val as i32;
//                         if *base_reg == Reg::RBP || *base_reg == Reg::R13 {
//                             if signed >= -128 && signed <= 127 {
//                                 (1u8, vec![*val as u8])
//                             } else {
//                                 (2u8, val.to_le_bytes().to_vec())
//                             }
//                         } else {
//                             if signed >= -128 && signed <= 127 && signed != 0 {
//                                 (1u8, vec![*val as u8])
//                             } else if signed == 0 {
//                                 (0u8, vec![])
//                             } else {
//                                 (2u8, val.to_le_bytes().to_vec())
//                             }
//                         }
//                     }
//                     Imm::Imm64(_val) => {
//                         // x86_64 中 displacement 最大为 32位
//                         unimplemented!("64-bit displacement not supported");
//                     }
//                 };

//                 if *base_reg == Reg::RSP || *base_reg == Reg::R12 {
//                     // RSP/R12 需要 SIB 字节
//                     let modrm: u8 = (mod_val << 6) | (reg << 3) | 0b100;
//                     self.result.push(modrm);
//                     // SIB: base=RSP(index=RSP means none), scale=0, index=RSP(means none)
//                     let sib: u8 = 0b00_100_100;
//                     self.result.push(sib);
//                 } else {
//                     let modrm: u8 = (mod_val << 6) | (reg << 3) | reg_id;
//                     self.result.push(modrm);
//                 }

//                 // 写入 displacement 字节
//                 for b in disp_bytes {
//                     self.result.push(b);
//                 }
//             }
//             RM::AddrSIB(base, index, scale) => {
//                 // [base + index * scale]
//                 let base_id = (*base as u8) & 0x07;
//                 let index_id = (*index as u8) & 0x07;
//                 let scale_enc = match scale {
//                     1 => 0u8,
//                     2 => 1u8,
//                     4 => 2u8,
//                     8 => 3u8,
//                     _ => panic!("Invalid scale factor: {}", scale),
//                 };

//                 let modrm: u8 = 0b00_000_000 | (0 << 3) | 0b100; // reg=0 占位，mod=00
//                 self.result.push(modrm);
//                 // SIB: scale(2) + index(3) + base(3)
//                 let sib: u8 = (scale_enc << 6) | (index_id << 3) | base_id;
//                 self.result.push(sib);

//                 // 修正 ModRM 中的 reg 字段
//                 let len = self.result.len;
//                 self.result.data[(len - 3) as usize] = 0b00_000_000 | (0 << 3) | 0b100;
//             }
//         }
//     }

//     /// 编码立即数
//     fn encode_imm(&mut self, imm: &Imm) {
//         match imm {
//             Imm::Imm8(val) => {
//                 self.result.push(*val);
//             }
//             Imm::Imm16(val) => {
//                 let bytes = val.to_le_bytes();
//                 for b in &bytes {
//                     self.result.push(*b);
//                 }
//             }
//             Imm::Imm32(val) => {
//                 let bytes = val.to_le_bytes();
//                 for b in &bytes {
//                     self.result.push(*b);
//                 }
//             }
//             Imm::Imm64(val) => {
//                 let bytes = val.to_le_bytes();
//                 for b in &bytes {
//                     self.result.push(*b);
//                 }
//             }
//         }
//     }
// }

// /// 为 BinInst 添加 push 方法
// impl BinInst {
//     pub fn push(&mut self, byte: u8) {
//         if (self.len as usize) < self.data.len() {
//             self.data[self.len as usize] = byte;
//             self.len += 1;
//         } else {
//             panic!("Instruction too long (max 15 bytes)");
//         }
//     }
// }

// /// 编码一条指令为 BinInst
// pub fn encode_instruction(inst: &Inst, format: &InstFormat) -> BinInst {
//     let mut encoder = Encoder::new();
//     encoder.encode(inst, format);
//     encoder.result
// }
