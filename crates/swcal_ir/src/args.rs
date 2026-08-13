use crate::types::IRType;
use crate::expr::IRVar;

#[derive(Debug, Clone)]
pub struct Args {
    args: Vec<IRVar>,
}

impl Args {
    pub fn new() -> Self {
        Self {
            args: vec![],
        }
    }

    pub fn add_arg(mut self, var: IRVar) -> Self {
        self.args.push(var);
        self
    }
}

#[derive(Debug, Clone)]
pub struct Params {
    params: Vec<IRType>,
}

impl Params {
    pub fn new() -> Self {
        Self {
            params: vec![],
        }
    }

    pub fn add_param(mut self, param: IRType) -> Self {
        self.params.push(param);
        self
    }
}
