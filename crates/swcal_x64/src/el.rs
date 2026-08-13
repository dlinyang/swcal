use std::collections::HashMap;
use crate::inst::imm::Imm;
use crate::inst::operand::Operand;
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
pub struct Program {
    pub sections: Vec<Section>,
    /// label ->  (section idx, data idx)
    pub labels: HashMap<String, (usize, usize)>,
    pub relocations: Vec<PreRelocation>,
    pub globals: Vec<String>,
}

impl Program {
    pub fn scan_reloc_and_modify_inst(&mut self) {
        for reloc in &self.relocations {
            let label_idx = self.labels
                .get(&reloc.label.name().clone())
                .expect(format!("not exist label {} in program", reloc.label.name()).as_str());

            let len = self.guess_len_between_data(reloc.data_idx, *label_idx);
            let sec_i = reloc.data_idx.0;
            let dat_i = reloc.data_idx.1;
            assert!(sec_i < self.sections.len());
            assert!(dat_i < self.sections[sec_i].data.len());
            let data = &mut self.sections[sec_i].data[dat_i];
            match &reloc.label {
                base::Label::Addr { .. } => {
                    let inst = data.get_mut_inst().expect("relocation wrong with address");
                    match (&inst.dst, &inst.src, &inst.src_ext) {
                        (Some(Operand::Label), None, None) => {
                            inst.dst = Some(Operand::Imm(Imm::fit_val(len)));
                        },
                        (Some(Operand::Label), Some(src), None) => {
                            inst.dst = Some(Operand::Imm(Imm::try_from_width(src.width())));
                        },
                        (Some(dst), Some(Operand::Label), None) => {
                            inst.src = Some(Operand::Imm(Imm::try_from_width(dst.width())));
                        },
                        (Some(_), Some(_), Some(_)) => todo!(),
                        _ => panic!("not should like this"),
                    }
                },
                base::Label::Mem {..} => {},
            }
        }
    }

    pub fn guess_len_between_data(&self, init: (usize, usize), end: (usize, usize)) -> usize {
        assert!(init.0 < self.sections.len());
        assert!(end.0 < self.sections.len());
        let mut ret = 0;
        if init.0 == end.0 {
            let section = &self.sections[init.0];
            assert!(init.1 < section.data.len());
            assert!(end.1 < section.data.len());
            let start = init.1.min(end.1);
            let end = init.1.max(end.1);
            for x in start..end+1 {
                ret += section.data[x].len();
            }
        }
        else if init.0 < end.0 {
            let init_sec = &self.sections[init.0];
            for x in init.1..init_sec.data.len() {
                ret += init_sec.data[x].len();
            }
            for x in init.0 + 1..end.0 {
                let sec = &self.sections[x];
                for data in sec.data.iter() {
                    ret += data.len();
                }
            }
            let end_sec = &self.sections[end.0];
            for x in 0..end.1 + 1 {
                ret += end_sec.data[x].len();
            }
        }
        else {
            let init_sec = &self.sections[end.0];
            for x in end.1..init_sec.data.len() {
                ret += init_sec.data[x].len();
            }
            for x in end.0 + 1..init.0 {
                let sec = &self.sections[x];
                for data in sec.data.iter() {
                    ret += data.len();
                }
            }
            let end_sec = &self.sections[init.0];
            for x in 0..init.1 + 1 {
                ret += end_sec.data[x].len();
            }
        }

        ret
    }
}

impl std::fmt::Display for Program {
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

impl Program {
    pub fn new() -> Self {
        Self { sections: vec![], globals: vec![], relocations: vec![], labels: HashMap::new() }
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
    Inst(AsmInst),
    RawData{
        width: u16,
        data: Vec<u8>,
    },
    Res(u64),
    Align(u8),
}

impl Data  {
    pub fn get_mut_inst(&mut self) -> Option<&mut AsmInst> {
        match self {
            Data::Inst(inst) => Some(inst),
            _ => None
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Data::Inst(_) => 15, // asumpt bad situation is the max length inst
            Data::RawData { width:_, data } => data.len(),
            Data::Res(len) => *len as usize,
            Data::Align(align) => *align as usize,
        }
    }
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
