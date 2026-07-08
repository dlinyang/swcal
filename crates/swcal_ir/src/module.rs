use crate::{function::IRFunction, types::IRType};

pub struct Module {
    pub name: String,
    pub custom_type: Vec<(String, IRType)>,
    pub functions: Vec<IRFunction>,
}

impl Module {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            custom_type: vec![],
            functions: vec![],
        }
    }
}
