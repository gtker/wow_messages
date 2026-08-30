use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Writer {
    inner: String,
    indentation_level: u8,
    initial_indentation_level: u8,
}

impl Writer {
    pub const INDENTATION: &'static str = "    ";
    const COLUMN_LENGTH: usize = 80;

    pub(crate) fn new() -> Self {
        Self {
            inner: String::with_capacity(4000),
            indentation_level: 0,
            initial_indentation_level: 0,
        }
    }
    pub(crate) fn into_inner(self) -> String {
        assert_eq!(
            self.indentation_level, self.initial_indentation_level,
            "indentation level is not initial level {}, instead it is {}",
            self.initial_indentation_level, self.indentation_level
        );
        self.inner
    }

    pub(crate) fn inner(&self) -> &str {
        assert_eq!(
            self.indentation_level, self.initial_indentation_level,
            "indentation level is not initial level {}, instead it is {}",
            self.initial_indentation_level, self.indentation_level
        );
        &self.inner
    }

    pub(crate) fn open_curly(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.inner.write_str(" {").unwrap();
        self.newline();
        self.inc_indent();
    }

    pub(crate) fn constructor(
        &mut self,
        name: impl AsRef<str>,
        ty_name: impl AsRef<str>,
        args: impl Fn(&mut Self),
        body: impl Fn(&mut Self),
    ) {
        let ty_name = ty_name.as_ref();

        self.wln(format!("pub const fn {}(", name.as_ref()));
        self.inc_indent();

        args(self);

        self.dec_indent();
        self.wln(format!(") -> {ty_name} {{"));
        self.inc_indent();

        self.wln(format!("{ty_name}::new("));
        self.inc_indent();

        body(self);

        self.dec_indent();
        self.wln(")");

        self.closing_curly(); // fn body
    }

    pub(crate) fn pub_const_fn(
        &mut self,
        name: impl AsRef<str>,
        return_ty: impl AsRef<str>,
        f: impl Fn(&mut Self),
    ) {
        self.open_curly(format!(
            "pub const fn {}(&self) -> {}",
            name.as_ref(),
            return_ty.as_ref()
        ));

        f(self);

        self.closing_curly(); // fn body
    }

    pub(crate) fn pub_const_fn_new(
        &mut self,
        args: impl Fn(&mut Self),
        self_body: impl Fn(&mut Self),
    ) {
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

    pub(crate) fn bodyn(&mut self, s: impl AsRef<str>, f: impl Fn(&mut Self)) {
        self.open_curly(s);

        f(self);

        self.closing_curly_newline();
    }

    pub(crate) fn body_else(
        &mut self,
        s: impl AsRef<str>,
        if_statement: impl Fn(&mut Self),
        else_statement: impl Fn(&mut Self),
    ) {
        self.open_curly(s);
        if_statement(self);
        self.closing_curly_with(" else {");
        self.inc_indent();
        else_statement(self);
        self.closing_curly();
    }

    pub(crate) fn body(&mut self, s: impl AsRef<str>, f: impl Fn(&mut Self)) {
        self.open_curly(s);

        f(self);

        self.closing_curly();
    }

    pub(crate) fn body_closing_with(
        &mut self,
        s: impl AsRef<str>,
        f: impl Fn(&mut Self),
        ending: impl AsRef<str>,
    ) {
        self.open_curly(s);

        f(self);

        self.closing_curly_with(ending.as_ref());
    }

    pub(crate) fn closing_curly(&mut self) {
        self.dec_indent();
        self.wln("}");
    }

    pub(crate) fn closing_curly_with(&mut self, s: impl AsRef<str>) {
        self.dec_indent();
        self.wln(format!("}}{}", s.as_ref()));
    }

    pub(crate) fn closing_curly_newline(&mut self) {
        self.dec_indent();
        self.wln("}");
        self.newline();
    }

    pub(crate) fn dec_indent(&mut self) {
        if self.indentation_level == 0 {
            panic!("attempted to underflow identation level");
        }
        self.indentation_level -= 1;
    }

    pub(crate) fn inc_indent(&mut self) {
        if self.indentation_level == 0xff {
            panic!("attempted to overflow identation level");
        }
        self.indentation_level += 1;
    }

    pub(crate) fn wln<S: AsRef<str>>(&mut self, s: S) {
        self.w(s);
        self.newline();
    }

    pub(crate) fn w_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
    }
    pub(crate) fn w_break_at(&mut self, s: impl AsRef<str>) {
        let column = self.get_column();
        if column >= Self::COLUMN_LENGTH {
            self.newline();
            self.w(s.as_ref());
        } else if column == 0 {
            self.w(s.as_ref());
        } else {
            self.w_no_indent(s.as_ref());
        }
    }

    pub(crate) fn wln_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
        self.newline();
    }

    pub(crate) fn newline(&mut self) {
        self.inner.write_str("\n").unwrap();
    }

    pub(crate) fn w(&mut self, s: impl AsRef<str>) {
        for _ in 0..self.indentation_level {
            self.inner.write_str(Self::INDENTATION).unwrap();
        }

        self.inner.write_str(s.as_ref()).unwrap();
    }

    fn get_column(&self) -> usize {
        self.inner.len() - (self.inner.rfind(|a| a == '\n').unwrap() + 1)
    }
}

