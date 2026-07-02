use std::sync::{Arc, LazyLock, RwLock};

static X86_64_SYSTEM_MODE: LazyLock<Arc<RwLock<u8>>> = LazyLock::new( ||  Arc::new(RwLock::new(64))) ;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Imm {
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
    Imm64(u64),
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
pub enum AddrWidth {
    B8,
    B16,
    B32,
    B64,
}

impl std::fmt::Display for AddrWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddrWidth::B8 => write!(f, "byte"),
            AddrWidth::B16 => write!(f, "word"),
            AddrWidth::B32 => write!(f, "dword"),
            AddrWidth::B64 => write!(f, "qword"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RM {
    Reg(Reg),
    //w [Reg]
    AddrReg(AddrWidth, Reg),
    //width [Reg+disp]
    AddrRegDisp(AddrWidth, Reg, Imm),
    // ModRM,noModRM.rm == 100
    //[base + index * scale]
    AddrSIB(AddrWidth, Reg, Reg, u8),
}

impl std::fmt::Display for RM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RM::Reg(reg) => write!(f, "{}", reg),
            RM::AddrReg(width, reg) => write!(f, "{width} [{reg}]"),
            RM::AddrRegDisp(width, reg, imm) => write!(f, "{width} [{reg} + {imm}]"),
            RM::AddrSIB(width, base, index, scale) => write!(f, "{width} [{base} + {index} * {scale}]"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Zero,
    Imm(Imm),
    RM(RM),
    Imm2RM { rm: RM, imm: Imm },
    Reg2RM { reg: Reg, rm: RM },
    RM2Reg { reg: Reg, rm: RM },
    RmOpImm2reg { src: Reg, rm: RM, imm: Imm}
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Zero => write!(f, ""),
            Operand::Imm(imm) => write!(f, "{}", imm),
            Operand::RM(rm) => write!(f, "{}", rm),
            Operand::Imm2RM { rm: reg, imm } => write!(f, "{}, {}", reg, imm),
            Operand::Reg2RM { reg, rm } => write!(f, "{}, {}", reg, rm),
            Operand::RM2Reg { reg, rm } => write!(f, "{}, {}", rm, reg),
            Operand::RmOpImm2reg { src, rm, imm } => write!(f, "{}, {}, {}", src, rm, imm),
        }
    }
}


#[derive(Debug)]
pub struct Inst {
    pub mnemonic: String,
    pub operand: Operand,
}

impl std::fmt::Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.mnemonic, self.operand)
    }
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
    // 8-bit extends reg
    R8B = 0b0001_1_000 | 0,
    R9B = 0b0001_1_000 | 1,
    R10B = 0b0001_1_000 | 2,
    R11B = 0b0001_1_000 | 3,
    R12B = 0b0001_1_000 | 4,
    R13B = 0b0001_1_000 | 5,
    R14B = 0b0001_1_000 | 6,
    R15B = 0b0001_1_000 | 7,
    // 16-bit 寄存器
    AX = 0b0010_0_000 | 0,
    CX = 0b0010_0_000 | 1,
    DX = 0b0010_0_000 | 2,
    BX = 0b0010_0_000 | 3,
    SP = 0b0010_0_000 | 4,
    BP = 0b0010_0_000 | 5,
    SI = 0b0010_0_000 | 6,
    DI = 0b0010_0_000 | 7,
    // 16-bit extende reg
    R8D = 0b0010_1_000 | 0,
    R9D = 0b0010_1_000 | 1,
    R10D = 0b0010_1_000 | 2,
    R11D = 0b0010_1_000 | 3,
    R12D = 0b0010_1_000 | 4,
    R13D = 0b0010_1_000 | 5,
    R14D = 0b0010_1_000 | 6,
    R15D = 0b0010_1_000 | 7,
    // 32-bit 寄存器
    EAX = 0b0011_0_000 | 0,
    ECX = 0b0011_0_000 | 1,
    EDX = 0b0011_0_000 | 2,
    EBX = 0b0011_0_000 | 3,
    ESP = 0b0011_0_000 | 4,
    EBP = 0b0011_0_000 | 5,
    ESI = 0b0011_0_000 | 6,
    EDI = 0b0011_0_000 | 7,
    // 32-bit extend reg
    R8W = 0b0011_1_000 | 0,
    R9W = 0b0011_1_000 | 1,
    R10W = 0b0011_1_000 | 2,
    R11W = 0b0011_1_000 | 3,
    R12W = 0b0011_1_000 | 4,
    R13W = 0b0011_1_000 | 5,
    R14W = 0b0011_1_000 | 6,
    R15W = 0b0011_1_000 | 7,
    // 64-bit 寄存器
    RAX = 0b0100_0_000 | 0,
    RCX = 0b0100_0_000 | 1,
    RDX = 0b0100_0_000 | 2,
    RBX = 0b0100_0_000 | 3,
    RSP = 0b0100_0_000 | 4,
    RBP = 0b0100_0_000 | 5,
    RSI = 0b0100_0_000 | 6,
    RDI = 0b0100_0_000 | 7,
    // 64-bit extend reg
    R8 = 0b0100_1_000 | 0,
    R9 = 0b0100_1_000 | 1,
    R10 = 0b0100_1_000 | 2,
    R11 = 0b0100_1_000 | 3,
    R12 = 0b0100_1_000 | 4,
    R13 = 0b0100_1_000 | 5,
    R14 = 0b0100_1_000 | 6,
    R15 = 0b0100_1_000 | 7,
}

/// 寄存器类别枚举（高4位编码）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegKind {
    /// 8位低字节寄存器 (AL-DIL) 及扩展 (R8B-R15B)
    R8 = 0b0001_0_000,
    /// 16位寄存器 (AX-DI) 及扩展 (R8W-R15W)
    R16 = 0b0010_0_000,
    /// 32位寄存器 (EAX-EDI) 及扩展 (R8D-R15D)
    R32 = 0b011_0_000,
    /// 64位寄存器 (RAX-RDI) 及扩展 (R8-R15)
    R64 = 0b0100_0_000,
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
            RegKind::R8  => 1,
            RegKind::R16 => 2,
            RegKind::R32 => 4,
            RegKind::R64 => 8,
        }
    }

    pub fn bit_width(&self) -> AddrWidth {
        match self.kind() {
            RegKind::R8  => AddrWidth::B8,
            RegKind::R16 => AddrWidth::B16,
            RegKind::R32 => AddrWidth::B32,
            RegKind::R64 => AddrWidth::B64,
        }
    }

    pub fn zero_imm(&self) -> Imm {
        match self.kind() {
            RegKind::R8 => Imm::Imm8(0),
            RegKind::R16 => Imm::Imm16(0),
            RegKind::R32 => Imm::Imm32(0),
            RegKind::R64 => Imm::Imm64(0),
        }
    }

    /// 返回低 8 位子寄存器（用于 REX 前缀的 low-byte 访问）
    pub fn low_byte(&self) -> Option<Self> {
        let id = self.id();
        let cat = self.kind();
        // 已经是 8 位寄存器则返回 None
        if cat == RegKind::R8 {
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

impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::AL => "al",
            Self::CL => "cl",
            Self::DL => "dl",
            Self::BL => "bl",
            Self::SPL => "spl",
            Self::BPL => "bpl",
            Self::SIL => "sil",
            Self::DIL => "dil",
            Self::R8B => "r8b",
            Self::R9B => "r9b",
            Self::R10B => "r10b",
            Self::R11B => "r11b",
            Self::R12B => "r12b",
            Self::R13B => "r13b",
            Self::R14B => "r14b",
            Self::R15B => "r15b",
            Self::AX => "ax",
            Self::CX => "cx",
            Self::DX => "dx",
            Self::BX => "bx",
            Self::SP => "sp",
            Self::BP => "bp",
            Self::SI => "si",
            Self::DI => "di",
            Self::R8W => "r8w",
            Self::R9W => "r9w",
            Self::R10W => "r10w",
            Self::R11W => "r11w",
            Self::R12W => "r12w",
            Self::R13W => "r13w",
            Self::R14W => "r14w",
            Self::R15W => "r15w",
            Self::EAX => "eax",
            Self::ECX => "ecx",
            Self::EDX => "edx",
            Self::EBX => "ebx",
            Self::ESP => "esp",
            Self::EBP => "ebp",
            Self::ESI => "esi",
            Self::EDI => "edi",
            Self::R8D => "r8d",
            Self::R9D => "r9d",
            Self::R10D => "r10d",
            Self::R11D => "r11d",
            Self::R12D => "r12d",
            Self::R13D => "r13d",
            Self::R14D => "r14d",
            Self::R15D => "r15d",
            Self::RAX => "rax",
            Self::RCX => "rcx",
            Self::RDX => "rdx",
            Self::RBX => "rbx",
            Self::RSP => "rsp",
            Self::RBP => "rbp",
            Self::RSI => "rsi",
            Self::RDI => "rdi",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
        };
        write!(f, "{}", name)
    }
}

impl std::str::FromStr for Reg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "al" => Ok(Self::AL),
            "cl" => Ok(Self::CL),
            "dl" => Ok(Self::DL),
            "bl" => Ok(Self::BL),
            "spl" => Ok(Self::SPL),
            "bpl" => Ok(Self::BPL),
            "sil" => Ok(Self::SIL),
            "dil" => Ok(Self::DIL),
            "r8b" => Ok(Self::R8B),
            "r9b" => Ok(Self::R9B),
            "r10b" => Ok(Self::R10B),
            "r11b" => Ok(Self::R11B),
            "r12b" => Ok(Self::R12B),
            "r13b" => Ok(Self::R13B),
            "r14b" => Ok(Self::R14B),
            "r15b" => Ok(Self::R15B),
            "ax" => Ok(Self::AX),
            "cx" => Ok(Self::CX),
            "dx" => Ok(Self::DX),
            "bx" => Ok(Self::BX),
            "sp" => Ok(Self::SP),
            "bp" => Ok(Self::BP),
            "si" => Ok(Self::SI),
            "di" => Ok(Self::DI),
            "r8w" => Ok(Self::R8W),
            "r9w" => Ok(Self::R9W),
            "r10w" => Ok(Self::R10W),
            "r11w" => Ok(Self::R11W),
            "r12w" => Ok(Self::R12W),
            "r13w" => Ok(Self::R13W),
            "r14w" => Ok(Self::R14W),
            "r15w" => Ok(Self::R15W),
            "eax" => Ok(Self::EAX),
            "ecx" => Ok(Self::ECX),
            "edx" => Ok(Self::EDX),
            "ebx" => Ok(Self::EBX),
            "esp" => Ok(Self::ESP),
            "ebp" => Ok(Self::EBP),
            "esi" => Ok(Self::ESI),
            "edi" => Ok(Self::EDI),
            "r8d" => Ok(Self::R8D),
            "r9d" => Ok(Self::R9D),
            "r10d" => Ok(Self::R10D),
            "r11d" => Ok(Self::R11D),
            "r12d" => Ok(Self::R12D),
            "r13d" => Ok(Self::R13D),
            "r14d" => Ok(Self::R14D),
            "r15d" => Ok(Self::R15D),
            "rax" => Ok(Self::RAX),
            "rcx" => Ok(Self::RCX),
            "rdx" => Ok(Self::RDX),
            "rbx" => Ok(Self::RBX),
            "rsp" => Ok(Self::RSP),
            "rbp" => Ok(Self::RBP),
            "rsi" => Ok(Self::RSI),
            "rdi" => Ok(Self::RDI),
            "r8" => Ok(Self::R8),
            "r9" => Ok(Self::R9),
            "r10" => Ok(Self::R10),
            "r11" => Ok(Self::R11),
            "r12" => Ok(Self::R12),
            "r13" => Ok(Self::R13),
            "r14" => Ok(Self::R14),
            "r15" => Ok(Self::R15),
            _ => Err(format!("unknown register: {}", s)),
        }
    }
}
