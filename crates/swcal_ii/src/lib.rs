use std::collections::HashMap;

use swcal_ir::function::IRFunction;

pub type VReg = [u8;8];
const REG_SIZE: usize = size_of::<VReg>();
pub type FFICall = fn(&[VReg], Option<&mut VReg>);

pub enum RunError {
}

pub struct Interpreter {
    mem: Vec<u8>,
    execu_frame: Vec<u8>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            mem: vec![],
            execu_frame: vec![]
        }
    }

    pub fn resize_exec_frame(&mut self, len: usize) {
        self.execu_frame.resize(len, 0);
    }

    pub fn resize_mem(&mut self, len: usize) {
        self.mem.resize(len, 0);
    }

    pub fn run(&mut self, pgm: &Program) -> Result<(), RunError> {
        // 加载静态数据
        self.mem.append(&mut pgm.sdata.clone());
        self.run_f(0, 0, pgm)
    }

    pub fn run_f(&mut self, cur_frame_i: usize, fn_id: usize, pgm: &Program) -> Result<(), RunError>{
        let cur_fun = &pgm.funs[fn_id];
        let cur_frame_len = pgm.funs[fn_id].alloca_reg_count() * REG_SIZE;
        self.resize_exec_frame(cur_frame_i + cur_frame_len);
        let mut blk_id = 0;
        while blk_id < cur_fun.blocks.len() {
            for inst in &cur_fun.blocks[blk_id].insts {
                match inst {
                    Inst::Load { ty, reg, addr } => {
                        let (regs, _) = self.exec_frame[cur_frame_i..cur_frame_len].as_chunks_mut::<REG_SIZE>();
                        for i in 0..ty.size() {
                            regs[reg.0 as usize][i] = self.mem[addr + i];
                        }
                    },
                    Inst::Store {ty , reg, addr} => {
                        let (regs, _) = self.exec_frame[cur_frame_i..cur_frame_len].as_chunks::<REG_SIZE>();
                        for i in 0..ty.size() {
                            self.mem[addr + i] = regs[reg.0 as usize][i];
                        }
                    },
                    Inst::BInst { op, ty, dst, rhs, lhs } => {
                        let (regs, _) = self.exec_frame[cur_frame_i..cur_frame_len].as_chunks_mut::<REG_SIZE>();
                        let dst = regs[dst.0 as usize];
                        let rhs = regs[rhs.0 as usize];
                        let lhs = regs[lhs.0 as usize];
                        match op {
                            BinOp::Add => {

                            },
                            BinOp::Sub => {

                            },
                            BinOp::Mul => {

                            },
                            BinOp::Div => {

                            },
                            BinOp::Eq => {

                            },
                            BinOp::NE => {

                            },
                            BinOp::BT => {

                            },
                            BinOp::LT => {

                            },
                            BinOp::And => {

                            },
                            BinOp::Nand => {

                            },
                            BinOp::Xor => {

                            }
                        }
                    },
                    Inst::Jmp(label) => {},
                    Inst::Br { cond, br_t, br_f } => {},
                    Inst::Call { f_label, ret } => {
                        todo!()
                    }
                    Inst::CallFFi { ffi_f_label, ret } => {
                        let called_frame_i = cur_frame_i + cur_frame_len;
                        let nlen = ffi_f_label.args.args.len() * REG_SIZE;
                        self.resize_exec_frame(called_frame_i + nlen);
                        let ffi_fun = pgm.ffi_funs[ffi_f_label.id as usize];
                        let (frame, called_frame) = self.exec_frame.split_at_mut(called_frame_i);
                        let (cur_frame, _) = frame[cur_frame_i..cur_frame_len].as_chunks_mut::<REG_SIZE>();
                        let (args, _) = called_frame.as_chunks_mut::<REG_SIZE>();
                        for (i, arg) in ffi_f_label.args.args.iter().enumerate() {
                           args[i] = cur_frame[arg.0 as usize];
                        }
                        ffi_fun(args, ret.map(|x| &mut cur_frame[x.0 as usize]));
                    }
                    Inst::Ret => {},
                    Inst::Halt => return Ok(()),
                }
            }
            blk_id += 1;
        }
        Ok(())
    }
}

pub enum IFnKind {
    IRFn,
    FFIFn,
}

pub struct Program {
    pub f_table: HashMap<String, usize>,
    pub funs: Vec<IRFunction>,
    pub ffi_funs: Vec<FFICall>,
    pub sdata: Vec<u8>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            f_table: HashMap::new(),
            funs: vec![],
            ffi_funs: vec![],
            sdata: vec![],
        }
    }

    pub fn add_fn(&mut self, name: impl Into<String>, f: impl FnOnce(&mut Self) -> Function) -> &mut Self {
        self.f_table.insert(name.into(), self.funs.len());
        let fun = f(self);
        self.funs.push(fun);
        self
    }

    pub fn add_ffi_fn(&mut self, name: impl Into<String>, f: FFICall) -> &mut Self {
        self.f_table.insert(name.into(), self.ffi_funs.len());
        self.ffi_funs.push(f);
        self
    }

}

pub fn print_result(data: &[VReg], _: Option<&mut VReg>) {
    for (i, reg) in data.iter().enumerate() {
        println!("reg{i} = {reg:x?}");
    }
}
