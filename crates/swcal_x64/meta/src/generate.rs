pub struct RustBuilder {
    lines: Vec<String>,
    indent: u32,
}

impl RustBuilder {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            indent: 0,
        }
    }

    pub fn indent(&mut self, f: impl FnOnce(&mut Self)) {
        self.indent += 4;
        f(self);
        self.indent -= 4;
    }

    pub fn block(&mut self, ident: impl AsRef<str>, f: impl FnOnce(&mut Self)) {
        self.line(format!("{}{{", ident.as_ref()));
        self.indent(f);
        self.line("}");
    }

    pub fn paren(&mut self, ident: impl AsRef<str>, f: impl FnOnce(&mut Self))  {
        self.line(format!("{}(", ident.as_ref()));
        self.indent(f);
        self.line(")");
    }

    pub fn if_codition(&mut self, condition: impl AsRef<str>, f: impl FnOnce(&mut Self)) {
        self.block(format!("if {} ", condition.as_ref()), f);
    }

    pub fn function(&mut self, f_decl: impl AsRef<str>, f: impl FnOnce(&mut Self)) {
        self.block(f_decl.as_ref(), f);
    }

    pub fn stmt_match(&mut self, var: impl AsRef<str>, f: impl FnOnce(&mut Self)) {
        self.block(format!("match {}", var.as_ref()), f);
    }

    pub fn record(&mut self, recordtype: impl AsRef<str>, f: impl FnOnce(&mut Self)) {
        self.line(format!("pub struct {} {{", recordtype.as_ref()));
        self.indent(f);
        self.line("}");
    }

    pub fn implement(&mut self, recordtype: impl AsRef<str>, f: impl FnOnce(&mut Self)) {
        self.line(format!("impl {} {{", recordtype.as_ref()));
        self.indent(f);
        self.line("}");
    }

    pub fn blank(&mut self) -> &mut Self {
        self.lines.push(String::new());
        self
    }

    pub fn line(&mut self, line: impl AsRef<str>) -> &mut Self {
        let ws = " ".repeat(self.indent as usize);
        self.lines.push(format!("{}{}", ws, line.as_ref()));
        self
    }

    pub fn build(&self) -> String {
        self.lines.join("\n")
    }
}

pub trait SrcGen {
    fn var_name(&self) -> String;
    fn type_name(&self) -> String;
    fn lit_name(&self) -> String;
}

#[macro_export]
macro_rules! type_name {
    ($var: expr) => {
        $var.type_name()
    };
    ($var:expr, $($vars: expr),+) => {
        format!("{}{}", $var.type_name(), type_name!($($vars),+))
    }
}

pub trait Validation {
    fn validation(&self) -> String;
}
