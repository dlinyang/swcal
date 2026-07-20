pub trait CodeSink {
    fn putb(&mut self, byte: u8);
    fn putw(&mut self, word: u16);
    fn putd(&mut self, dword: u32);
    fn putq(&mut self, qword: u64);
    fn modify(&mut self, f: impl FnOnce(&mut u8));
}

pub type Buffer = Vec<u8>;

impl CodeSink for Buffer {
    fn putb(&mut self, byte: u8) {
        self.push(byte);
    }

    fn putw(&mut self, word: u16) {
        self.extend_from_slice(&word.to_le_bytes());
    }

    fn putd(&mut self, dword: u32) {
        self.extend_from_slice(&dword.to_le_bytes());
    }

    fn putq(&mut self, qword: u64) {
        self.extend_from_slice(&qword.to_le_bytes());
    }

    fn modify(&mut self, f: impl FnOnce(&mut u8)) {
        if let Some(last) = self.last_mut() {
            f(last)
        } else {
            panic!("modify failed");
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstBin {
    data: [u8;15],
    len: usize,
}

impl InstBin {
    pub fn new() -> Self {
        Self { data: [0;_], len: 0 }
    }

    pub fn less(&self, other: Self) -> Self {
        if self.len < other.len {
            *self
        } else {
            other
        }
    }
}

impl CodeSink for InstBin {
    fn putb(&mut self, byte: u8) {
        if self.len < 15 {
            self.data[self.len] = byte;
            self.len += 1;
        } else {
            panic!("InstBin overflow: cannot put byte");
        }
    }

    fn putw(&mut self, word: u16) {
        let bytes = word.to_le_bytes();
        if self.len + 2 <= 15 {
            self.data[self.len..self.len + 2].copy_from_slice(&bytes);
            self.len += 2;
        } else {
            panic!("InstBin overflow: cannot put word");
        }
    }

    fn putd(&mut self, dword: u32) {
        let bytes = dword.to_le_bytes();
        if self.len + 4 <= 15 {
            self.data[self.len..self.len + 4].copy_from_slice(&bytes);
            self.len += 4;
        } else {
            panic!("InstBin overflow: cannot put dword");
        }
    }

    fn putq(&mut self, qword: u64) {
        let bytes = qword.to_le_bytes();
        if self.len + 8 <= 15 {
            self.data[self.len..self.len + 8].copy_from_slice(&bytes);
            self.len += 8;
        } else {
            panic!("InstBin overflow: cannot put qword");
        }
    }

    fn modify(&mut self, f: impl FnOnce(&mut u8)) {
        if self.len == 0 {
            panic!("modify failed: InstBin is empty");
        }
        f(&mut self.data[self.len - 1]);
    }
}

impl std::fmt::Display for InstBin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.data[..self.len].iter() {
            write!(f, "{:02x} ", byte)?;
        }
        Ok(())
    }
}
