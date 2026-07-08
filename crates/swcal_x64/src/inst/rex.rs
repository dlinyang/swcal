/// REX prefix structure for x86-64 instructions.
///
/// The REX (Register EXtension) prefix is used in x86-64 instructions to
/// extend the register encoding from 3 bits to 4 bits, providing access to
/// the extended general-purpose registers (r8-r15) and new byte registers.
///
/// # Structure
///
/// The REX prefix is a single byte with the following bit layout:
///
/// ```text
/// 0   1   0   0   W   R   X   B
/// ```
///
/// | Bit | Name | Description                          |
/// |-----|------|--------------------------------------|
/// | 7-4 | `0100` | Fixed pattern (0x4)                 |
/// | 3   | W     | Width: 1 = 64-bit operand size       |
/// | 2   | R     | Extends the ModRM.reg field (bit 2)  |
/// | 1   | X     | Extends the SIB.index field (bit 2)  |
/// | 0   | B     | Extends the ModRM.rm field, SIB.base, or opcode reg field (bit 2) |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rex {
    /// Width: 64-bit operand size
    pub w: bool,
    /// Extends the ModRM.reg field
    pub r: bool,
    /// Extends the SIB.index field
    pub x: bool,
    /// Extends the ModRM.rm field, SIB.base, or opcode reg field
    pub b: bool,
}

impl Rex {
    /// w r x b is false
    pub fn new() -> Self {
        Self { w: false, r: false, x: false, b: false }
    }

    pub fn from_byte(byte: u8) -> Self {
        Self {
            w: (byte & 0x08) != 0,
            r: (byte & 0x04) != 0,
            x: (byte & 0x02) != 0,
            b: (byte & 0x01) != 0,
        }
    }

    /// check if need rex
    pub fn need(&self) -> bool {
        self.w || self.r || self.b || self.x
    }

    /// Returns the raw byte representation of the REX prefix.
    pub fn byte(&self) -> u8 {
        0x40
            | (if self.w { 0x08 } else { 0 })
            | (if self.r { 0x04 } else { 0 })
            | (if self.x { 0x02 } else { 0 })
            | (if self.b { 0x01 } else { 0 })
    }
}

impl core::fmt::Display for Rex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "REX(0x{:02x}) [W={}, R={}, X={}, B={}]",
            self.byte(),
            self.w as u8,
            self.r as u8,
            self.x as u8,
            self.b as u8
        )
    }
}
