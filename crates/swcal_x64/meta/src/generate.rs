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

    pub fn block(&mut self, f: impl FnOnce(&mut Self)) {
        self.line("{");
        self.indent(f);
        self.line("}");
    }

    pub fn if_codition(&mut self, condition: impl AsRef<str>, f: impl FnOnce(&mut Self)) {
        self.line(format!("if {} {{", condition.as_ref()));
        self.indent(f);
        self.line("}");
    }

    pub fn function(&mut self, f_decl: impl AsRef<str>, f: impl FnOnce(&mut Self)) {
        self.line(format!("{}{{", f_decl.as_ref()));
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
