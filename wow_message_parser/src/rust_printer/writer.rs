use crate::file_info::FileInfo;
use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct Writer {
    inner: String,
    prefix: String,
    indentation_level: u8,
    docc_indentation_level: u8,
    initial_indentation_level: u8,
}

impl Writer {
    pub const INDENTATION: &'static str = "    ";
    const METADATA: bool = true;
    const COLUMN_LENGTH: usize = 80;

    pub(crate) fn new() -> Self {
        Self {
            inner: String::with_capacity(4000),
            prefix: "".to_string(),
            indentation_level: 0,
            docc_indentation_level: 0,
            initial_indentation_level: 0,
        }
    }
    pub(crate) fn at_indentation(indentation: u8) -> Self {
        Self {
            inner: String::with_capacity(4000),
            prefix: "".to_string(),
            indentation_level: indentation,
            docc_indentation_level: 0,
            initial_indentation_level: indentation,
        }
    }

    pub(crate) fn start_with(inner: String) -> Self {
        Self {
            inner,
            prefix: "".to_string(),
            indentation_level: 0,
            docc_indentation_level: 0,
            initial_indentation_level: 0,
        }
    }

    pub(crate) fn with_prefix(prefix: &str) -> Self {
        Self {
            inner: String::with_capacity(4000),
            prefix: prefix.to_string(),
            indentation_level: 0,
            docc_indentation_level: 0,
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

    pub(crate) fn new_struct(&mut self, name: impl AsRef<str>, f: impl Fn(&mut Self)) {
        self.open_curly(format!("pub struct {}", name.as_ref()));

        f(self);

        self.closing_curly_newline();
    }

    pub(crate) fn new_flag(
        &mut self,
        name: impl AsRef<str>,
        ty: impl AsRef<str>,
        declarations: impl Fn(&mut Self),
    ) {
        self.open_curly(format!("pub struct {name}", name = name.as_ref()));

        self.wln(format!("inner: {ty},", ty = ty.as_ref()));
        declarations(self);

        self.closing_curly_newline();
    }

    pub(crate) fn new_enum(
        &mut self,
        visibility: impl AsRef<str>,
        name: impl AsRef<str>,
        f: impl Fn(&mut Self),
    ) {
        self.open_curly(format!(
            "{visibility} enum {name}",
            visibility = visibility.as_ref(),
            name = name.as_ref()
        ));

        f(self);

        self.closing_curly_newline();
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

    pub(crate) fn funcn_const(
        &mut self,
        name_and_args: impl AsRef<str>,
        return_type: impl AsRef<str>,
        f: impl Fn(&mut Self),
    ) {
        self.open_curly(format!(
            "pub(crate) const fn {} -> {}",
            name_and_args.as_ref(),
            return_type.as_ref()
        ));

        f(self);

        self.closing_curly_newline();
    }

    pub(crate) fn funcn_pub_const(
        &mut self,
        name_and_args: impl AsRef<str>,
        return_type: impl AsRef<str>,
        f: impl Fn(&mut Self),
    ) {
        self.open_curly(format!(
            "pub const fn {} -> {}",
            name_and_args.as_ref(),
            return_type.as_ref()
        ));

        f(self);

        self.closing_curly_newline();
    }

    pub(crate) fn funcn(
        &mut self,
        name_and_args: impl AsRef<str>,
        return_type: impl AsRef<str>,
        f: impl Fn(&mut Self),
    ) {
        self.open_curly(format!(
            "pub(crate) fn {} -> {}",
            name_and_args.as_ref(),
            return_type.as_ref()
        ));

        f(self);

        self.closing_curly_newline();
    }

    pub(crate) fn funcn_pub(
        &mut self,
        name_and_args: impl AsRef<str>,
        return_type: impl AsRef<str>,
        f: impl Fn(&mut Self),
    ) {
        self.open_curly(format!(
            "pub fn {} -> {}",
            name_and_args.as_ref(),
            return_type.as_ref()
        ));

        f(self);

        self.closing_curly_newline();
    }

    pub(crate) fn impl_for(
        &mut self,
        s: impl AsRef<str>,
        s2: impl AsRef<str>,
        f: impl Fn(&mut Self),
    ) {
        self.open_curly(format!("impl {} for {}", s.as_ref(), s2.as_ref()));

        f(self);

        self.closing_curly_newline();
    }

    pub(crate) fn bodyn(&mut self, s: impl AsRef<str>, f: impl Fn(&mut Self)) {
        self.open_curly(s);

        f(self);

        self.closing_curly_newline();
    }

    pub(crate) fn body_else_with_closing(
        &mut self,
        curly_text: impl AsRef<str>,
        closing: impl AsRef<str>,
        if_statement: impl Fn(&mut Self),
        else_statement: impl Fn(&mut Self),
    ) {
        self.open_curly(curly_text);
        if_statement(self);
        self.closing_curly_with(" else {");
        self.inc_indent();
        else_statement(self);
        self.closing_curly_with(closing.as_ref());
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

    pub(crate) fn body_no_indent_or_space_with_semicolon(&mut self, f: impl Fn(&mut Self)) {
        self.wln_no_indent("{");
        self.inc_indent();

        f(self);

        self.closing_curly_with(";");
    }

    pub(crate) fn body_closing_with_semicolon(
        &mut self,
        s: impl AsRef<str>,
        f: impl Fn(&mut Self),
    ) {
        self.open_curly(s);

        f(self);

        self.closing_curly_with(";");
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

    pub(crate) fn metadata_comment(&mut self, s: impl AsRef<str>) {
        if !Self::METADATA {
            return;
        }

        self.w(format!("// {}", s.as_ref()));
        self.newline();
    }

    pub(crate) fn docc_newline(&mut self) {
        if !Self::METADATA {
            return;
        }
        self.w("///");
        self.newline();
    }

    pub(crate) fn docc(&mut self, s: impl AsRef<str>) {
        if !Self::METADATA {
            return;
        }
        self.w("/// ");
        for _ in 0..self.docc_indentation_level {
            self.w(Self::INDENTATION);
        }
        self.w_no_indent(s.as_ref());
        self.newline();
    }

    pub(crate) fn docc_wowm(&mut self, f: impl Fn(&mut Self), fileinfo: &FileInfo) {
        if !Self::METADATA {
            return;
        }
        self.docc(format!(
            "Auto generated from the original `wowm` in file {github_link}:",
            github_link = fileinfo.original_file_github_link(),
        ));
        self.docc("```text");
        f(self);
        self.docc("```");
    }

    pub(crate) fn w_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
    }
    pub(crate) fn w_break_at(&mut self, s: impl AsRef<str>) {
        let column = self.get_column();
        if column >= Self::COLUMN_LENGTH {
            self.newline();
            self.write_prefix();
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

    pub(crate) fn space(&mut self) {
        self.inner.write_str(" ").unwrap();
    }

    pub(crate) fn w(&mut self, s: impl AsRef<str>) {
        self.write_prefix();

        for _ in 0..self.indentation_level {
            self.inner.write_str(Self::INDENTATION).unwrap();
        }

        self.inner.write_str(s.as_ref()).unwrap();
    }

    fn get_column(&self) -> usize {
        self.inner.len() - (self.inner.rfind(|a| a == '\n').unwrap() + 1)
    }

    pub(crate) fn bytes<'a>(&mut self, bytes: impl Iterator<Item = &'a u8>) {
        for b in bytes {
            let text = format!("{b}, ");
            self.w(&text);
        }
    }

    pub(crate) fn add_table<const N: usize>(&mut self, table: WriterTable<N>) {
        let mut column_sizes = table.header.iter().map(|a| a.len() + 1).collect::<Vec<_>>();

        for body in &table.body {
            for (i, column) in body.iter().enumerate() {
                if column.len() + 1 > column_sizes[i] {
                    column_sizes[i] = column.len();
                }
            }
        }

        for (i, row) in table.header.iter().enumerate() {
            let pad = column_sizes[i];
            self.w_no_indent(format!("| {row: <pad$} ", pad = pad));
        }
        self.wln_no_indent("|");

        for (i, _) in table.header.iter().enumerate() {
            let pad = column_sizes[i];
            let character = "-";
            self.w_no_indent(format!("|-{character:-<pad$}-", pad = pad));
        }
        self.wln_no_indent("|");

        for body in &table.body {
            for (i, row) in body.iter().enumerate() {
                let pad = column_sizes[i];
                self.w_no_indent(format!("| {row: <pad$} ", pad = pad));
            }
            self.wln_no_indent("|");
        }
    }

    fn write_prefix(&mut self) {
        self.inner.write_str(&self.prefix).unwrap();
    }
}

pub(crate) struct WriterTable<const N: usize> {
    header: [String; N],
    body: Vec<[String; N]>,
}

impl<const N: usize> WriterTable<N> {
    pub(crate) fn new(header: [impl AsRef<str>; N]) -> Self {
        Self {
            header: header.map(|a| a.as_ref().to_string()),
            body: vec![],
        }
    }

    pub(crate) fn add_row(&mut self, row: [impl AsRef<str>; N]) {
        self.body.push(row.map(|a| a.as_ref().to_string()))
    }
}
