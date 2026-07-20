use std::fmt;

/// Represents the VEX prefix for x86_64 AVX instructions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VexPrefix {
    /// The raw three-byte VEX prefix (0xC4 or 0xC5 followed by two bytes)
    pub bytes: [u8; 3],
}

impl VexPrefix {
    /// Create a VEX prefix from individual fields.
    ///
    /// # Arguments
    ///
    /// * `r` - R bit (1 for VEX.R, complements modRM.reg when 0)
    /// * `x` - X bit (1 for VEX.X, complements SIB index when 0)
    /// * `b` - B bit (1 for VEX.B, complements base register when 0)
    /// * `m_mmmm` - 5-bit map select (0b00001 = 0F, 0b00010 = 0F38, 0b00011 = 0F3A)
    /// * `w` - W bit (opcode extension or operand size)
    /// * `vvvv` - 4-bit source or dest register (128-bit or 256-bit)
    /// * `l` - L bit (0 = 128-bit, 1 = 256-bit)
    /// * `pp` - 2-bit implied prefix (0b00 = none, 0b01 = 66, 0b10 = F3, 0b11 = F2)
    pub fn from_fields(
        r: u8,
        x: u8,
        b: u8,
        m_mmmm: u8,
        w: u8,
        vvvv: u8,
        l: u8,
        pp: u8,
    ) -> Self {
        debug_assert!(r <= 1, "r must be 0 or 1");
        debug_assert!(x <= 1, "x must be 0 or 1");
        debug_assert!(b <= 1, "b must be 0 or 1");
        debug_assert!(m_mmmm <= 0b11111, "m_mmmm must fit in 5 bits");
        debug_assert!(w <= 1, "w must be 0 or 1");
        debug_assert!(vvvv <= 0b1111, "vvvv must fit in 4 bits");
        debug_assert!(l <= 1, "l must be 0 or 1");
        debug_assert!(pp <= 0b11, "pp must fit in 2 bits");

        // VEX 3-byte encoding (0xC4)
        let byte2 = (1 << 7)       // ~R (inverted)
            | ((!r & 1) << 7)     // R (inverted)
            | ((!x & 1) << 6)     // X (inverted)
            | ((!b & 1) << 5)     // B (inverted)
            | (m_mmmm & 0b11111); // m_mmmm

        let byte3 = (w & 1) << 7
            | ((!vvvv) & 0b1111) << 3 // vvvv (inverted)
            | (l & 1) << 2
            | (pp & 0b11);

        VexPrefix {
            bytes: [0xC4, byte2, byte3],
        }
    }

    /// Create a 2-byte VEX prefix (0xC5 form, only available when m_mmmm == 0b00001).
    ///
    /// The fields `x`, `b`, and `m_mmmm` are not present in the 2-byte form.
    pub fn from_fields_2byte(
        r: u8,
        vvvv: u8,
        l: u8,
        pp: u8,
    ) -> Self {
        debug_assert!(r <= 1, "r must be 0 or 1");
        debug_assert!(vvvv <= 0b1111, "vvvv must fit in 4 bits");
        debug_assert!(l <= 1, "l must be 0 or 1");
        debug_assert!(pp <= 0b11, "pp must fit in 2 bits");

        let byte2 = (1 << 7)       // ~R (inverted)
            | ((!r & 1) << 7)     // R (inverted)
            | ((!vvvv) & 0b1111) << 3 // vvvv (inverted)
            | (l & 1) << 2
            | (pp & 0b11);

        VexPrefix {
            bytes: [0xC5, byte2, 0],
        }
    }

    /// Returns the R bit (inverted from the stored encoding).
    pub fn r(&self) -> u8 {
        (!(self.bytes[1] >> 7)) & 1
    }

    /// Returns the X bit (inverted from the stored encoding), 3-byte only.
    pub fn x(&self) -> u8 {
        if self.bytes[0] == 0xC5 {
            1 // not present in 2-byte form, defaults to 1
        } else {
            (!(self.bytes[1] >> 6)) & 1
        }
    }

    /// Returns the B bit (inverted from the stored encoding), 3-byte only.
    pub fn b(&self) -> u8 {
        if self.bytes[0] == 0xC5 {
            1 // not present in 2-byte form, defaults to 1
        } else {
            (!(self.bytes[1] >> 5)) & 1
        }
    }

    /// Returns the m_mmmm field (3-byte only).
    pub fn m_mmmm(&self) -> u8 {
        if self.bytes[0] == 0xC5 {
            0b00001 // implied as 0F in 2-byte form
        } else {
            self.bytes[1] & 0b11111
        }
    }

    /// Returns the W bit.
    pub fn w(&self) -> u8 {
        if self.bytes[0] == 0xC5 {
            0 // not present in 2-byte form, defaults to 0
        } else {
            (self.bytes[2] >> 7) & 1
        }
    }

    /// Returns the vvvv field (inverted from the stored encoding).
    pub fn vvvv(&self) -> u8 {
        let raw = (self.bytes[if self.bytes[0] == 0xC5 { 1 } else { 2 }] >> 3) & 0b1111;
        (!raw) & 0b1111
    }

    /// Returns the L bit.
    pub fn l(&self) -> u8 {
        let idx = if self.bytes[0] == 0xC5 { 1 } else { 2 };
        (self.bytes[idx] >> 2) & 1
    }

    /// Returns the pp field (implied prefix).
    pub fn pp(&self) -> u8 {
        let idx = if self.bytes[0] == 0xC5 { 1 } else { 2 };
        self.bytes[idx] & 0b11
    }

    /// Check if this is a 2-byte VEX prefix.
    pub fn is_2byte(&self) -> bool {
        self.bytes[0] == 0xC5
    }

    /// Returns the raw bytes of the VEX prefix (2 or 3 bytes depending on form).
    pub fn as_bytes(&self) -> &[u8] {
        if self.bytes[0] == 0xC5 {
            &self.bytes[..2]
        } else {
            &self.bytes[..3]
        }
    }

    /// Decode a VEX prefix from raw instruction bytes starting at the given offset.
    /// Returns the decoded prefix and the length consumed (2 or 3).
    pub fn decode(bytes: &[u8], offset: usize) -> Option<(Self, usize)> {
        if offset >= bytes.len() {
            return None;
        }
        match bytes[offset] {
            0xC4 => {
                // 3-byte VEX prefix
                if offset + 3 > bytes.len() {
                    return None;
                }
                Some((
                    VexPrefix {
                        bytes: [bytes[offset], bytes[offset + 1], bytes[offset + 2]],
                    },
                    3,
                ))
            }
            0xC5 => {
                // 2-byte VEX prefix
                if offset + 2 > bytes.len() {
                    return None;
                }
                Some((
                    VexPrefix {
                        bytes: [bytes[offset], bytes[offset + 1], 0],
                    },
                    2,
                ))
            }
            _ => None,
        }
    }
}

impl fmt::Display for VexPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_2byte() {
            write!(
                f,
                "VEX.2byte: R={}, vvvv={}, L={}, pp={}",
                self.r(),
                self.vvvv(),
                self.l(),
                self.pp()
            )
        } else {
            write!(
                f,
                "VEX.3byte: R={}, X={}, B={}, m_mmmm={}, W={}, vvvv={}, L={}, pp={}",
                self.r(),
                self.x(),
                self.b(),
                self.m_mmmm(),
                self.w(),
                self.vvvv(),
                self.l(),
                self.pp()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3byte_encode_decode() {
        let vex = VexPrefix::from_fields(1, 0, 1, 0b00001, 0, 0b0110, 1, 0b01);
        assert_eq!(vex.bytes[0], 0xC4);
        assert_eq!(vex.bytes[1], 0b_0_0_1_1_00001); // R=1 X=0 B=1 m_mmmm=00001
        assert_eq!(vex.bytes[2], 0b_0_1001_1_01); // W=0 vvvv=0110 L=1 pp=01

        assert_eq!(vex.r(), 1);
        assert_eq!(vex.x(), 0);
        assert_eq!(vex.b(), 1);
        assert_eq!(vex.m_mmmm(), 0b00001);
        assert_eq!(vex.w(), 0);
        assert_eq!(vex.vvvv(), 0b0110);
        assert_eq!(vex.l(), 1);
        assert_eq!(vex.pp(), 0b01);
    }

    #[test]
    fn test_2byte_encode_decode() {
        let vex = VexPrefix::from_fields_2byte(1, 0b0110, 1, 0b01);
        assert_eq!(vex.bytes[0], 0xC5);
        assert_eq!(vex.bytes[1], 0b_1_1001_1_01); // R=1 vvvv=0110 L=1 pp=01
        assert!(vex.is_2byte());

        assert_eq!(vex.r(), 1);
        assert_eq!(vex.vvvv(), 0b0110);
        assert_eq!(vex.l(), 1);
        assert_eq!(vex.pp(), 0b01);
        // Defaults for 2-byte form
        assert_eq!(vex.x(), 1);
        assert_eq!(vex.b(), 1);
        assert_eq!(vex.m_mmmm(), 0b00001);
        assert_eq!(vex.w(), 0);
    }

    #[test]
    fn test_decode() {
        let code = &[0xC4, 0b_0_1_0_00010, 0b_1_0011_0_10, 0x58, 0xC0];
        let (vex, len) = VexPrefix::decode(code, 0).unwrap();
        assert_eq!(len, 3);
        assert_eq!(vex.r(), 1);
        assert_eq!(vex.x(), 0);
        assert_eq!(vex.b(), 1);
        assert_eq!(vex.m_mmmm(), 0b00010);
        assert_eq!(vex.w(), 1);
        assert_eq!(vex.vvvv(), 0b0011);
        assert_eq!(vex.l(), 0);
        assert_eq!(vex.pp(), 0b10);
    }

    #[test]
    fn test_decode_2byte() {
        let code = &[0xC5, 0b_0_0110_0_01, 0x58, 0xD8];
        let (vex, len) = VexPrefix::decode(code, 0).unwrap();
        assert_eq!(len, 2);
        assert!(vex.is_2byte());
        assert_eq!(vex.r(), 1); // inverted from bit 7
        assert_eq!(vex.vvvv(), 0b0110);
        assert_eq!(vex.l(), 0);
        assert_eq!(vex.pp(), 0b01);
    }
}
