use crate::inst::Inst;

/// excutable and linkable Table
#[derive(Debug)]
pub struct EL {
    pub sections: Vec<Section>,
    pub globals: Vec<String>,
}
use std::fmt;

impl fmt::Display for EL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "global symbol: ")?;
        for global_symbol in &self.globals {
            writeln!(f, "{global_symbol}")?;
        }
        for section in &self.sections {
            write!(f,"{section}")?;
        }

        Ok(())
    }
}

impl EL {
    pub fn new() -> Self {
        Self { sections: vec![], globals: vec![] }
    }
}

#[derive(Debug)]
pub struct Section {
    pub name: Option<String>,
    pub data: Vec<Data>,
    pub labels: Vec<(String, usize)>,
    pub relocation: Vec<(usize, String)>,
}

impl Section {
    pub fn new() -> Self {
        Self {
            name: None,
            data: vec![],
            labels: vec![],
            relocation: vec![],
        }
    }
}


impl fmt::Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            writeln!(f, "section: {name}")?;
        }
        for (label, addr) in &self.labels {
            writeln!(f, "label: {label} @ offset {addr}")?;
        }
        for data in &self.data {
            writeln!(f, "{data}")?;
        }
        for (offset, sym) in &self.relocation {
            writeln!(f, "relocation: @{offset} -> {sym}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Data {
    Inst(Inst),
    RawData(Vec<u8>),
    Res(u64),
    Align(u8),
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data::Inst(inst) => write!(f, "{inst}"),
            Data::RawData(bytes) => {
                write!(f, "rawdata: [")?;
                for (i, byte) in bytes.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "0x{:02x}", byte)?;
                }
                write!(f, "]")
            }
            Data::Res(size) => write!(f, "res: {size}"),
            Data::Align(alignment) => write!(f, "align: {alignment}"),
        }
    }
}
