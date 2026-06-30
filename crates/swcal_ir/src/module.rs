use crate::{function::IRFunction, types::IRType};

pub struct Module {
    pub name: String,
    pub custom_type: Vec<(String, IRType)>,
    pub functions: Vec<IRFunction>,
}
