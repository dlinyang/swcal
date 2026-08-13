macro_rules! reg_enum {
    {
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident $(= $value:expr)?),* $(,)?
        }
    } => {
        $(#[$meta])*
        $vis enum $name {
            $($variant $(= $value)?),*
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_uppercase().as_str() {
                    $(stringify!($variant) => Ok($name::$variant),)*
                    _ => Err(format!("unmatched register {}", s))
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let ret = match self {
                    $($name::$variant => stringify!($variant),)*
                };
                write!(f, "{}", ret.to_lowercase().as_str())
            }
        }
    };
}

reg_enum!{
    /// x86_64 Register:
    ///
    /// |7 ~ 5 |4| 3~0|
    /// |---|--|--|
    /// | kind  |e| reg |
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Reg {
        // 8-bit reg
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
        // 16-bit reg
        AX = 0b0010_0_000 | 0,
        CX = 0b0010_0_000 | 1,
        DX = 0b0010_0_000 | 2,
        BX = 0b0010_0_000 | 3,
        SP = 0b0010_0_000 | 4,
        BP = 0b0010_0_000 | 5,
        SI = 0b0010_0_000 | 6,
        DI = 0b0010_0_000 | 7,
        // 16-bit extende reg
        R8W = 0b0010_1_000 | 0,
        R9W = 0b0010_1_000 | 1,
        R10W = 0b0010_1_000 | 2,
        R11W = 0b0010_1_000 | 3,
        R12W = 0b0010_1_000 | 4,
        R13W = 0b0010_1_000 | 5,
        R14W = 0b0010_1_000 | 6,
        R15W = 0b0010_1_000 | 7,
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
        R8D = 0b0011_1_000 | 0,
        R9D = 0b0011_1_000 | 1,
        R10D = 0b0011_1_000 | 2,
        R11D = 0b0011_1_000 | 3,
        R12D = 0b0011_1_000 | 4,
        R13D = 0b0011_1_000 | 5,
        R14D = 0b0011_1_000 | 6,
        R15D = 0b0011_1_000 | 7,
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
}

/// TODO: XMM and CR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegKind {
    /// general register 8bit
    GR8  = 0b0001_0_000,
    /// general register 16bit
    GR16 = 0b0010_0_000,
    /// general register 32bit
    GR32 = 0b0011_0_000,
    /// general register 64bit
    GR64 = 0b0100_0_000,
}

/// 寄存器类别掩码（高4位）
const REGKIND_MASK: u8 = 0b1111_0000;
/// 扩展位掩码（第4位）
const REG_EXT_MASK: u8 = 0b0000_1000;
/// 寄存器 id 掩码（低3位）
const REG_ID_MASK: u8 = 0b0000_0111;

impl Reg {
    /// 返回寄存器在 x86_64 指令编码中的 id 值 (3位)
    pub const fn id(&self) -> u8 {
        (*self as u8) & REG_ID_MASK
    }

    /// 返回寄存器类别
    pub fn kind(&self) -> RegKind {
        let val = (*self as u8) & REGKIND_MASK;
        match val {
            x if x == RegKind::GR8 as u8 => RegKind::GR8,
            x if x == RegKind::GR16 as u8 => RegKind::GR16,
            x if x == RegKind::GR32 as u8 => RegKind::GR32,
            _ => RegKind::GR64,
        }
    }

    /// check exetend register
    pub fn is_extended(&self) -> bool {
        ((*self as u8) & REG_EXT_MASK) != 0
    }

    /// check 64bit mode for REX.w
    pub fn is_w64(&self) -> bool {
        self.kind() == RegKind::GR64
    }

    /// 返回寄存器的宽度（字节数）
    pub fn width(&self) -> u16 {
        match self.kind() {
            RegKind::GR8  => 8,
            RegKind::GR16 => 16,
            RegKind::GR32 => 32,
            RegKind::GR64 => 64,
        }
    }

    /// 返回低 8 位子寄存器（用于 REX 前缀的 low-byte 访问）
    pub fn low_byte(&self) -> Option<Self> {
        let id = self.id();
        let cat = self.kind();
        // 已经是 8 位寄存器则返回 None
        if cat == RegKind::GR8 {
            return None;
        }
        let is_ext = self.is_extended();
        match cat {
            RegKind::GR64 | RegKind::GR32 | RegKind::GR16 => {
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
