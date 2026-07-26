use crate::inst::base::Label;

#[derive(Debug)]
pub struct PreRelocation {
    pub label: Label,
    pub data_idx: (usize, usize),
}

#[derive(Debug)]
pub struct Relocation {
    pub label_name: String,
    pub inst_idx: (usize,usize),
    pub reloc_type: RelocType,
    pub disp: i32,
}

#[derive(Debug)]
pub enum RelocType {
    Relative,
    Abstract,
}
