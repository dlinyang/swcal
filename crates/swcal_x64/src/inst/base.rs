pub trait RegEnc {
    fn from_id(id: u8) -> Self;
    fn encode(&self) -> u8;
    fn is_extend(&self) -> bool;
}

impl RegEnc for u8 {
    fn from_id(id: u8) -> Self {
        id
    }

    fn encode(&self) -> u8 {
        *self
    }

    fn is_extend(&self) -> bool {
        (self & 0b1000) == 0b1000
    }
}

pub fn width_as_str(width: u16) -> String {
    match width {
        8 => format!("byte"),
        16 => format!("word"),
        32 => format!("dword"),
        64 => format!("qword"),
        _ => format!("{}bit", width),
    }
}

#[derive(Debug, Clone)]
pub enum Label {
    // disp can be i64
    Addr{ name: String, disp: i32 },
    Mem { name: String, disp: i32 },
}
