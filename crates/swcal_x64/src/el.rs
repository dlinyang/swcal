use std::collections::HashMap;
use crate::inst::*;
use crate::inst::base::width_as_str;
use crate::reloc::*;

/// excutable and linkable Table
pub struct El {
    pub sections: Vec<Vec<u8>>,
    pub labels: HashMap<String, (usize, usize)>,
    pub relocation: Vec<Relocation>,
    pub globals: Vec<String>,
}

/// pre excutable and linkable table or program
#[derive(Debug)]
pub struct PreEL {
    pub sections: Vec<Section>,
    /// label ->  (section idx, data idx)
    pub labels: HashMap<String, (usize, usize)>,
    pub relocation: Vec<PreRelocation>,
    pub globals: Vec<String>,
}

impl std::fmt::Display for PreEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "global symbol: ")?;
        for global_symbol in &self.globals {
            writeln!(f, "{global_symbol}")?;
        }
        // TODO label
        for section in &self.sections {
            write!(f,"{section}")?;
        }

        Ok(())
    }
}

impl PreEL {
    pub fn new() -> Self {
        Self { sections: vec![], globals: vec![], relocation: vec![], labels: HashMap::new() }
    }
}

#[derive(Debug)]
pub struct Section {
    pub name: Option<String>,
    pub data: Vec<Data>,
    // pub labels: Vec<(String, usize)>,
    // pub relocation: Vec<(usize, String)>,
}

impl Section {
    pub fn new() -> Self {
        Self {
            name: None,
            data: vec![],
            // labels: vec![],
            // relocation: vec![],
        }
    }
}

impl std::fmt::Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            writeln!(f, "section: {name}")?;
        }
        // for (label, addr) in &self.labels {
        //     writeln!(f, "label: {label} @ offset {addr}")?;
        // }
        for data in &self.data {
            writeln!(f, "{data}")?;
        }
        // for (offset, sym) in &self.relocation {
        //     writeln!(f, "relocation: @{offset} -> {sym}")?;
        // }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Data {
    Inst(Inst),
    RawData{
        width: u16,
        data: Vec<u8>,
    },
    Res(u64),
    Align(u8),
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Inst(inst) => write!(f, "{inst}"),
            Data::RawData{ width, data } => {
                write!(f, "{}: [", width_as_str(*width))?;
                for (i, byte) in data.iter().enumerate() {
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
