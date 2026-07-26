// use std::fmt::format;

use crate::{generate::*, type_name};

pub mod prefix;
pub use prefix::*;
pub mod opcode;
pub use opcode::*;
pub mod operand;
pub use operand::*;
pub mod modrm;
pub use modrm::*;

pub struct Encode {
    pub modrm: ModRMKind,
    pub operand: OperandEncode,
}

impl std::fmt::Display for Encode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:<6} {}", self.modrm, self.operand)
    }
}

pub fn encode(modrm: ModRMKind, operand: OperandEncode) -> Encode {
    Encode { modrm, operand }
}

impl SrcGen for Encode {
    fn var_name(&self) -> String {
        todo!()
    }

    fn type_name(&self) -> String {
        type_name!(self.operand, self.modrm)
    }

    fn lit_name(&self) -> String {
        todo!()
    }
}
pub struct InstFormat {
    pub mnemonic: String,
    pub prefix: Prefix,
    pub opcode: Opcode,
    pub encode: Encode,
}

impl InstFormat {
    /// Creates a new `InstFormat` with the given parameters.
    pub fn new(
        mnemonic: impl Into<String>,
        prefix: Prefix,
        opcode: Opcode,
        encode: Encode,
    ) -> Self {
        Self {
            mnemonic: mnemonic.into(),
            prefix,
            opcode,
            encode,
        }
    }
}

#[macro_export]
macro_rules! instf {
    ($mnemonic: expr, $prefix: expr, $opcode: expr, $modrm: expr) => {
        InstFormat::new($mnemonic, $prefix, $opcode, encode($modrm, operand!()))
    };
    ($mnemonic: expr, $prefix: expr, $opcode: expr, $modrm: expr, $ope: expr) => {
        InstFormat::new($mnemonic, $prefix, $opcode, encode($modrm, operand!($ope)))
    };
    ($mnemonic: expr, $prefix: expr, $opcode: expr, $modrm: expr, $dst: expr, $src: expr) => {
        InstFormat::new($mnemonic, $prefix, $opcode, encode($modrm, operand!($dst, $src)))
    };
    ($mnemonic: expr, $prefix: expr, $opcode: expr, $modrm: expr, $dst: expr, $src: expr, $src_other: expr) => {
        InstFormat::new($mnemonic, $prefix, $opcode, encode($modrm, operand!($dst, $src, $src_other)))
    };
}

impl std::fmt::Display for InstFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<7} {:<8} {:<} {}",
            self.mnemonic,
            self.opcode,
            self.encode,
            self.prefix,
        )
    }
}

impl SrcGen for InstFormat {
    fn var_name(&self) -> String {
        todo!()
    }

    fn type_name(&self) -> String {
        format!(
            "{}{}",
            self.mnemonic.to_uppercase(),
            self.encode.type_name(),
        )
    }

    fn lit_name(&self) -> String {
        todo!()
    }
}

impl Validation for InstFormat {
    fn validation(&self) -> String {
        todo!()
    }
}

pub fn build_inst(src: &mut RustBuilder, instf: &InstFormat) {
    // get operand vec form Operand enum
    // operand vec is more easy to use, but operand enum is more good contraint i think
    let operands = instf.encode.operand.to_vec();

    src.line(format!("//{}", instf).as_str());
    src.record(instf.type_name(), |src| {
        genarate_operand_field(src, &instf.encode.operand);
    });
    src.blank();
    src.implement(instf.type_name(), |src| {
        let mut args = String::new();
        for operand in &operands {
            args.push_str(
                format!("{}:{},", operand.var_name(),operand.type_name()).as_str()
            );
        }

        src.function(format!("pub fn new({}) -> Self", args), |src| {
            src.block("Self", |src|{
                for operand in &operands {
                    src.line(format!("{},", operand.var_name()));
                }
            });
        });

        src.blank();
        src.function("pub fn from_inst(inst: &Inst) -> Result<Self, String>", |src| {
            let inst_field = ["dst", "src", "src_ext"];
            let mut i = 0;
            for operand in &operands {
                let var_name = operand.var_name();
                src.line(format!("let {} = inst.{}.ok_or(\"none operand\")?;", var_name, inst_field[i]));
                src.line(format!("let {} = {}.try_into()?;", var_name, var_name));
                i += 1;
            }

            // no operand
            if i == 0 {
                //use inst
                src.line("let _inst = inst;");
            }
            src.paren("Ok", |src|{
                src.block("Self", |src|
                    for operand in &operands {
                        src.line(format!("{},", operand.var_name()));
                    }
                )
            });
        });
        src.blank();
        src.function("pub fn encode(&self, buf: &mut impl CodeSink)", |src| {
            let mut reg_var_name = None;
            let mut rm_var_name = None;
            let mut reloc_var_name = None;
            let mut imm_var_name = None;
            let mut is_fixed_reg = false;

            for operand in &operands {
                match operand.ty {
                    OperandKind::Reg(reg_kind, _rwattr) => {
                        reg_var_name = Some(operand.var_name());
                        match reg_kind {
                            RegKind::Gpr => {},
                            RegKind::XMM => {},
                            RegKind::YMM => {},
                            RegKind::Fgr(_) => {
                                is_fixed_reg = true;
                            },
                        }
                    },
                    OperandKind::RM(_reg_kind, _rwattr) => rm_var_name = Some(operand.var_name()),
                    OperandKind::IMM => imm_var_name = Some(operand.var_name()),
                    OperandKind::Rel => reloc_var_name = Some(operand.var_name()),
                    OperandKind::MOFFSET => {},
                }
            }

            match instf.prefix {
                Prefix::Legacy => {
                    src.line("//legacy prefix and rex prefix");
                    if legacy_prefix_66h(&instf.encode.operand) {
                        src.line("buf.putb(0x66);");
                    }

                    src.line(format!("let rex_w = {};", legacy_prefix_rex_w(&instf.encode.operand)));

                    // check reg is extend
                    if let Some(var_name) = &reg_var_name && (!is_fixed_reg) && instf.encode.modrm != ModRMKind::Reg{
                        src.line(format!("let rex_r = self.{}.is_extend();",var_name));
                    } else {
                        src.line("let rex_r = false;");
                    }

                    // check rm is extend
                    if let Some(var_name) = &rm_var_name {
                            src.line(format!("let rex_x = self.{}.rex_x();",var_name));
                            src.line(format!("let rex_b = self.{}.rex_b();",var_name));
                    }
                    else {
                        src.line("let rex_x = false;");
                        if let ModRMKind::Reg = instf.encode.modrm {
                            let var_name = reg_var_name.as_ref().expect("opcode encode reg");
                            src.line(format!("let rex_b = self.{}.is_extend();",var_name));
                        } else {
                            src.line("let rex_b = false;");
                        }
                    }

                    src.line("Rex::new(rex_w, rex_r, rex_x, rex_b).encode(buf);");
                },
                Prefix::Vex => todo!(),
                Prefix::Evex => todo!(),
            }

            // opcode generate
            if let ModRMKind::Reg = instf.encode.modrm {
                let var_name = reg_var_name.as_ref().expect("Encode reg in opcode unmacted");
                src.line(format!("let op_reg = self.{}.encode();", var_name));
                src.line(format!("buf.putb({:#x}|(op_reg & 0b111));", instf.opcode.fst));
            } else {
                src.line(format!("buf.putb({:#x});",instf.opcode.fst));
            }
            if let Some(op) = &instf.opcode.snd {
                src.line(format!("buf.putb({:#x});",op));
            };
            if let Some(op) = &instf.opcode.trd {
                src.line(format!("buf.putb({:#x});",op));
            };

            match instf.encode.modrm {
                ModRMKind::None => {
                    src.line("//none modrm");
                },
                ModRMKind::Normal => {
                    src.line("//modrm");
                    // operand reg and rm
                    let reg_var_name = reg_var_name.as_ref().expect("modrm unmatched: no reg");
                    let rm_var_name = rm_var_name.as_ref().expect("modrm unmatched: no rm");
                    src.line(format!("encode_modrm(&self.{}, &self.{},buf);", reg_var_name, rm_var_name));
                },
                ModRMKind::Digit(ext_op) => {
                    src.line("//extend opcode in modrm");
                    let rm_var_name = rm_var_name.as_ref().expect(format!("modrm unmatched: no reg {}", instf).as_str());
                    src.line(format!("encode_modrm(&{}, &self.{},buf);", ext_op, rm_var_name));
                },
                ModRMKind::Reg => {
                    src.line("//none modrm and reg encode in opcode");
                },
            }

            if let Some(var_name) = &reloc_var_name {
                src.line("//encode reloc");
                src.line(format!("self.{}.encode(buf);",var_name));
            }

            // encode imm
            if let Some(var_name) = &imm_var_name {
                src.line("//encode imm");
                src.line(format!("self.{}.encode(buf);", var_name));
            }
        });
    });
}
