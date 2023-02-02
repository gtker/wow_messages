use std::fmt::Write;

pub(crate) struct Writer {
    inner: String,
    indentation: u8,
}

impl Writer {
    const INDENTATION: &'static str = "    ";

    pub fn new() -> Self {
        Self {
            inner: String::with_capacity(8000),
            indentation: 0,
        }
    }

    pub fn inc_indent(&mut self) {
        if self.indentation == 0xff {
            panic!("attempting to overflow indentation")
        }

        self.indentation += 1;
    }

    pub fn dec_indent(&mut self) {
        if self.indentation == 0 {
            panic!("attempting to underflow indentation");
        }

        self.indentation -= 1;
    }

    pub fn w_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub fn w(&mut self, s: impl AsRef<str>) {
        for _ in 0..self.indentation {
            self.inner.write_str(Self::INDENTATION).unwrap();
        }

        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub fn newline(&mut self) {
        self.inner.write_char('\n').unwrap();
    }

    pub fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.newline();
    }

    pub fn open_curly(&mut self, s: impl AsRef<str>) {
        self.wln(format!("{} {{", s.as_ref()));
        self.inc_indent();
    }

    pub fn closing_curly(&mut self) {
        self.dec_indent();
        self.wln("}");
    }

    pub fn pub_const_fn_new(&mut self, args: impl Fn(&mut Self), self_body: impl Fn(&mut Self)) {
        self.wln("pub const fn new(");
        self.inc_indent();

        args(self);

        self.dec_indent();
        self.wln(") -> Self {");
        self.inc_indent();

        self.open_curly("Self");
        self_body(self);
        self.closing_curly(); // Self

        self.closing_curly(); // fn body
    }

    pub fn constructor(
        &mut self,
        name: impl AsRef<str>,
        ty_name: impl AsRef<str>,
        args: impl Fn(&mut Self),
        self_body: impl Fn(&mut Self),
    ) {
        self.wln(format!("pub const fn {}(", name.as_ref()));
        self.inc_indent();

        args(self);

        self.dec_indent();
        self.wln(format!(") -> {} {{", ty_name.as_ref()));
        self.inc_indent();

        self.open_curly(ty_name);
        self_body(self);
        self.closing_curly(); // Self

        self.closing_curly(); // fn body
    }

    pub fn wln_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
        self.newline();
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }
}
