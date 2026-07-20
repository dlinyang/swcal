use std::fmt;
use crate::generate::*;

#[derive(Debug)]
pub struct Opcode {
    pub fst: u8,
    pub snd: Option<u8>,
    pub trd: Option<u8>,
}

#[macro_export]
macro_rules! opcode {
    ($p1:expr) => {
        Opcode {
            fst: $p1,
            snd: None,
            trd: None,
        }
    };
    ($p1:expr, $p2: expr) => {
        Opcode {
            fst: $p1,
            snd: Some($p2),
            trd: None,
        }
    };
    ($p1:expr, $p2: expr, $p3: expr) => {
        Opcode {
            fst: $p1,
            snd: Some($p2),
            trd: Some($p3),
        }
    };
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}", self.fst)?;
        if let Some(snd) = self.snd {
            write!(f, " {:02x}", snd)?;
        }
        if let Some(trd) = self.trd {
            write!(f, " {:02x}", trd)?;
        }
        Ok(())
    }
}

impl SrcGen for Opcode {
    fn var_name(&self) -> String {
        String::from("op_code")
    }

    fn type_name(&self) -> String {
        String::from("Opcode")
    }

    fn lit_name(&self) -> String {
        match (self.snd, self.snd) {
            (None, None) => format!("[{}u8]", self.fst),
            (Some(snd), None) => format!("[{}u8,{}u8]", self.fst, snd),
            (Some(snd), Some(trd)) => format!("[{}u8,{}u8,{}u8]",self.fst, snd, trd),
            _ => panic!("wrong opcode {} {:?} {:?}", self.fst, self.snd, self.snd),
        }
    }
}
