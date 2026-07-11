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
