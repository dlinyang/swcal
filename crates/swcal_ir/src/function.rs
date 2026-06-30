use crate::{block::BasicBlock, types::IRType};

pub struct IRFunction {
    pub name: String,
    pub params: Vec<IRType>,
    pub ret_type: IRType,
    pub blocks: Vec<BasicBlock>,
}

impl std::fmt::Display for IRFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {} ", self.name)?;
        write!(f, "(")?;
        for p in &self.params {
            write!(f, "{}", p)?;
        }
        write!(f, ")")?;
        write!(f, "->")?;
        write!(f, "{}", self.ret_type)
    }
}
