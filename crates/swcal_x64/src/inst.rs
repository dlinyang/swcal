#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Imm {
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
    Imm64(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RM {
    //[Reg]
    AddrReg(Reg),
    //[Reg+disp]
    AddrRegDisp(Reg, Imm),
    //[base + index * scale]
    AddrSIB(Reg, Reg, u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Zero,
    Imm2Reg {
        reg: Reg,
        imm: Imm,
    },
    Reg2RM {
        reg: Reg,
        rm: RM,
    },
    RM2Reg {
        reg: Reg,
        rm: RM,
    },
    Reg2Reg {
        src_reg: Reg,
        dst_reg: Reg,
    }
}

pub struct Inst {
    pub mnemonic: String,
    pub operand: Operand,
}

/// x86_64 通用寄存器枚举
/// 寄存器的编码格式：
/// - 低 3 位 (bit 0-2): 寄存器在指令编码中的 id（寄存器编号 0-7）
/// - 第 4 位 (bit 3): 是否为扩展寄存器 (R8-R15 系列)
/// - 高 4 位 (bit 4-7): 寄存器类别（宽度等）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg {
    // 8-bit 寄存器 - 低8位
    AL = 0b0001_0_000 | 0,
    CL = 0b0001_0_000 | 1,
    DL = 0b0001_0_000 | 2,
    BL = 0b0001_0_000 | 3,
    // 8-bit 寄存器 - 低8位扩展 (SPL, BPL, SIL, DIL)
    SPL = 0b0001_0_000 | 4,
    BPL = 0b0001_0_000 | 5,
    SIL = 0b0001_0_000 | 6,
    DIL = 0b0001_0_000 | 7,
    // 8-bit 寄存器 - 高8位
    AH = 0b0010_0_000 | 4,
    CH = 0b0010_0_000 | 5,
    DH = 0b0010_0_000 | 6,
    BH = 0b0010_0_000 | 7,
    // 16-bit 寄存器
    AX = 0b0011_0_000 | 0,
    CX = 0b0011_0_000 | 1,
    DX = 0b0011_0_000 | 2,
    BX = 0b0011_0_000 | 3,
    SP = 0b0011_0_000 | 4,
    BP = 0b0011_0_000 | 5,
    SI = 0b0011_0_000 | 6,
    DI = 0b0011_0_000 | 7,
    // 32-bit 寄存器
    EAX = 0b0100_0_000 | 0,
    ECX = 0b0100_0_000 | 1,
    EDX = 0b0100_0_000 | 2,
    EBX = 0b0100_0_000 | 3,
    ESP = 0b0100_0_000 | 4,
    EBP = 0b0100_0_000 | 5,
    ESI = 0b0100_0_000 | 6,
    EDI = 0b0100_0_000 | 7,
    // 64-bit 寄存器
    RAX = 0b0101_0_000 | 0,
    RCX = 0b0101_0_000 | 1,
    RDX = 0b0101_0_000 | 2,
    RBX = 0b0101_0_000 | 3,
    RSP = 0b0101_0_000 | 4,
    RBP = 0b0101_0_000 | 5,
    RSI = 0b0101_0_000 | 6,
    RDI = 0b0101_0_000 | 7,
    // 扩展寄存器 R8-R15
    R8  = 0b0101_1_000 | 0,
    R9  = 0b0101_1_000 | 1,
    R10 = 0b0101_1_000 | 2,
    R11 = 0b0101_1_000 | 3,
    R12 = 0b0101_1_000 | 4,
    R13 = 0b0101_1_000 | 5,
    R14 = 0b0101_1_000 | 6,
    R15 = 0b0101_1_000 | 7,
    R8D  = 0b0100_1_000 | 0,
    R9D  = 0b0100_1_000 | 1,
    R10D = 0b0100_1_000 | 2,
    R11D = 0b0100_1_000 | 3,
    R12D = 0b0100_1_000 | 4,
    R13D = 0b0100_1_000 | 5,
    R14D = 0b0100_1_000 | 6,
    R15D = 0b0100_1_000 | 7,
    R8W  = 0b0011_1_000 | 0,
    R9W  = 0b0011_1_000 | 1,
    R10W = 0b0011_1_000 | 2,
    R11W = 0b0011_1_000 | 3,
    R12W = 0b0011_1_000 | 4,
    R13W = 0b0011_1_000 | 5,
    R14W = 0b0011_1_000 | 6,
    R15W = 0b0011_1_000 | 7,
    R8B  = 0b0001_1_000 | 0,
    R9B  = 0b0001_1_000 | 1,
    R10B = 0b0001_1_000 | 2,
    R11B = 0b0001_1_000 | 3,
    R12B = 0b0001_1_000 | 4,
    R13B = 0b0001_1_000 | 5,
    R14B = 0b0001_1_000 | 6,
    R15B = 0b0001_1_000 | 7,
}

/// 寄存器类别枚举（高4位编码）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegKind {
    /// 8位低字节寄存器 (AL-DIL) 及扩展 (R8B-R15B)
    R8  = 0b0001_0_000,
    /// 8位高字节寄存器 (AH-BH)
    R8H = 0b0010_0_000,
    /// 16位寄存器 (AX-DI) 及扩展 (R8W-R15W)
    R16 = 0b0011_0_000,
    /// 32位寄存器 (EAX-EDI) 及扩展 (R8D-R15D)
    R32 = 0b0100_0_000,
    /// 64位寄存器 (RAX-RDI) 及扩展 (R8-R15)
    R64 = 0b0101_0_000,
}

/// 寄存器类别掩码（高4位）
const REGKIND_MASK: u8 = 0b1111_0000;
/// 扩展位掩码（第4位）
const REG_EXT_MASK: u8 = 0b0000_1000;
/// 寄存器 id 掩码（低3位）
const REG_ID_MASK: u8 = 0b0000_0111;

impl Reg {
    /// 返回寄存器在 x86_64 指令编码中的 id 值 (3位)
    pub fn id(&self) -> u8 {
        (*self as u8) & REG_ID_MASK
    }

    /// 返回寄存器类别
    pub fn kind(&self) -> RegKind {
        let val = (*self as u8) & REGKIND_MASK;
        match val {
            x if x == RegKind::R8 as u8 => RegKind::R8,
            x if x == RegKind::R8H as u8 => RegKind::R8H,
            x if x == RegKind::R16 as u8 => RegKind::R16,
            x if x == RegKind::R32 as u8 => RegKind::R32,
            _ => RegKind::R64,
        }
    }

    /// 是否为扩展寄存器 (R8-R15 系列)
    pub fn is_extended(&self) -> bool {
        ((*self as u8) & REG_EXT_MASK) != 0
    }

    /// 返回寄存器是否需要 REX 前缀
    pub fn needs_rex(&self) -> bool {
        // 扩展寄存器 (R8-R15 任何宽度) 需要 REX
        if self.is_extended() {
            return true;
        }
        // SPL, BPL, SIL, DIL 也需要 REX（它们是 8L 类别中 id 为 4-7 的寄存器）
        let cat = self.kind();
        let id = self.id();
        cat == RegKind::R8 && id >= 4
    }

    /// 返回寄存器的宽度（字节数）
    pub fn width(&self) -> usize {
        match self.kind() {
            RegKind::R8 | RegKind::R8H => 1,
            RegKind::R16 => 2,
            RegKind::R32 => 4,
            RegKind::R64 => 8,
        }
    }

    /// 返回低 8 位子寄存器（用于 REX 前缀的 low-byte 访问）
    pub fn low_byte(&self) -> Option<Self> {
        let id = self.id();
        let cat = self.kind();
        // 已经是 8 位寄存器则返回 None
        if cat == RegKind::R8 || cat == RegKind::R8H {
            return None;
        }
        let is_ext = self.is_extended();
        match cat {
            RegKind::R64 | RegKind::R32 | RegKind::R16 => {
                if is_ext {
                    // 扩展寄存器的低8位
                    let extended = match id {
                        0 => Self::R8B,
                        1 => Self::R9B,
                        2 => Self::R10B,
                        3 => Self::R11B,
                        4 => Self::R12B,
                        5 => Self::R13B,
                        6 => Self::R14B,
                        7 => Self::R15B,
                        _ => return None,
                    };
                    Some(extended)
                } else {
                    // 普通寄存器的低8位（仅 AL, CL, DL, BL 有低字节表示）
                    match id {
                        0 => Some(Self::AL),
                        1 => Some(Self::CL),
                        2 => Some(Self::DL),
                        3 => Some(Self::BL),
                        _ => None,
                    }
                }
            }
            _ => None,
        }
    }
}
