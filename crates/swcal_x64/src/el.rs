use crate::inst::Inst;

/// excutable and linkable Table
pub struct EL {
    pub sections: Vec<Section>,
}

pub struct Section {
    pub name: String,
    pub data: Vec<Data>,
    pub labels: Vec<(String, usize)>,
    pub relocation: Vec<(usize, String)>,
}

impl Section {
    pub fn new() -> Self {
        Self { name: "default".to_string(),
            data: vec![], labels: vec![], relocation: vec![] }
    }
}

pub enum Data {
    Inst(Inst),
    RawData(Vec<u8>),
    Res(u64),
}
