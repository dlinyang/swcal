use crate::inst::Inst;

/// excutable and linkable Table
#[derive(Debug)]
pub struct EL {
    pub sections: Vec<Section>,
    pub globals: Vec<String>,
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

#[derive(Debug)]
pub enum Data {
    Inst(Inst),
    RawData(Vec<u8>),
    Res(u64),
    Align(u8),
}
