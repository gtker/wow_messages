pub mod complex_print;
mod enums;
mod flags;
pub mod new_enums;
mod opcodes;
mod structs;

pub use enums::print_enum;
pub use flags::print_flag;
pub use opcodes::print_login_opcodes;
pub use opcodes::print_world_opcodes;
pub use structs::print_struct;

use crate::file_info::FileInfo;
use std::fmt::Write;

#[derive(Debug)]
pub struct Writer {
    inner: String,
    imports: String,
    indentation_level: u8,
    docc_indentation_level: u8,
    import_path: String,
}

pub const LOGIN_CLIENT_MESSAGE_TRAIT_NAME: &str = "ClientMessage";
pub const LOGIN_SERVER_MESSAGE_TRAIT_NAME: &str = "ServerMessage";
pub const LOGIN_CLIENT_MESSAGE_ENUM_NAME: &str = "ClientOpcodeMessage";
pub const LOGIN_SERVER_MESSAGE_ENUM_NAME: &str = "ServerOpcodeMessage";
pub const WORLD_CLIENT_MESSAGE_ENUM_NAME: &str = "WorldClientOpcodeMessage";
pub const WORLD_SERVER_MESSAGE_ENUM_NAME: &str = "WorldServerOpcodeMessage";

pub const WORLD_BODY_TRAIT_NAME: &str = "WorldMessageBody";
pub const WORLD_CLIENT_HEADER_TRAIT_NAME: &str = "WorldClientMessageWrite";
pub const WORLD_SERVER_HEADER_TRAIT_NAME: &str = "WorldServerMessageWrite";

impl Writer {
    const INDENTATION: &'static str = "    ";
    const METADATA: bool = true;

    pub fn new(import_path: &str) -> Self {
        Self {
            inner: String::with_capacity(4000),
            imports: String::with_capacity(4000),
            indentation_level: 0,
            docc_indentation_level: 0,
            import_path: import_path.to_string(),
        }
    }

    #[allow(unused)]
    pub fn print(&self) {
        println!("{}", self.inner);
    }

    pub fn open_curly<S: AsRef<str>>(&mut self, s: S) {
        self.w(s);
        self.inner.write_str(" {").unwrap();
        self.newline();
        self.inc_indent();
    }

    fn add_import<S: AsRef<str>>(&mut self, name: S) {
        self.imports
            .write_fmt(format_args!(
                "pub use {import_path}::{name};\n",
                import_path = self.import_path,
                name = name.as_ref(),
            ))
            .unwrap();
    }

    pub fn new_struct<S: AsRef<str>, F: Fn(&mut Self)>(&mut self, name: S, f: F) {
        self.add_import(&name);

        self.open_curly(format!("pub struct {}", name.as_ref()));

        f(self);

        self.closing_curly_newline();
    }

    pub fn new_flag<S: AsRef<str>, S1: AsRef<str>, F: Fn(&mut Self)>(
        &mut self,
        name: S,
        ty: S1,
        declarations: F,
    ) {
        self.add_import(name.as_ref());

        self.open_curly(format!("pub struct {name}", name = name.as_ref()));

        self.wln(format!("inner: {ty},", ty = ty.as_ref()));
        declarations(self);

        self.closing_curly_newline();
    }

    pub fn new_enum<S: AsRef<str>, F: Fn(&mut Self)>(&mut self, name: S, f: F) {
        self.add_import(&name);

        self.open_curly(format!("pub enum {}", name.as_ref()));

        f(self);

        self.closing_curly_newline();
    }

    pub fn const_fn<S: AsRef<str>, F: Fn(&mut Self)>(
        &mut self,
        name_and_args: S,
        return_type: S,
        f: F,
    ) {
        self.open_curly(format!(
            "pub const fn {} -> {}",
            name_and_args.as_ref(),
            return_type.as_ref()
        ));

        f(self);

        self.closing_curly_newline();
    }

    pub fn constant_sized<S: AsRef<str>, F: Fn(&mut Self)>(&mut self, name: S, f: F) {
        self.open_curly(format!("impl ConstantSized for {}", name.as_ref()));
        self.open_curly("fn size() -> usize");

        self.wln("Self::maximum_possible_size()");

        self.closing_curly();
        self.closing_curly_newline();

        self.open_curly(format!("impl MaximumPossibleSized for {}", name.as_ref()));
        self.open_curly("fn maximum_possible_size() -> usize");

        f(self);

        self.closing_curly();
        self.closing_curly_newline();
    }

    pub fn variable_size<S: AsRef<str>, F: Fn(&mut Self), F1: Fn(&mut Self)>(
        &mut self,
        name: S,
        variable_sized: F,
        maximum_size: F1,
    ) {
        self.open_curly(format!("impl VariableSized for {}", name.as_ref()));
        self.open_curly("fn size(&self) -> usize");

        variable_sized(self);

        self.closing_curly();
        self.closing_curly_newline();

        self.open_curly(format!("impl MaximumPossibleSized for {}", name.as_ref()));
        self.open_curly("fn maximum_possible_size() -> usize");

        maximum_size(self);

        self.closing_curly();
        self.closing_curly_newline();
    }

    pub fn impl_world_read_and_writable_with_error(
        &mut self,
        type_name: impl AsRef<str>,
        error_name: impl AsRef<str>,
        read_function: impl Fn(&mut Self),
        write_function: impl Fn(&mut Self),
    ) {
        self.open_curly(format!(
            "impl {} for {}",
            WORLD_BODY_TRAIT_NAME,
            type_name.as_ref()
        ));
        self.wln(format!(
            "type Error = {err_ty};",
            err_ty = error_name.as_ref()
        ));
        self.newline();

        self.open_curly(
            "fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error>",
        );
        read_function(self);
        self.closing_curly_newline();

        self.open_curly("fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error>");
        write_function(self);
        self.closing_curly();

        self.closing_curly_newline();
    }

    pub fn impl_read_and_writable_with_error<
        S: AsRef<str>,
        S1: AsRef<str>,
        F: Fn(&mut Self),
        F2: Fn(&mut Self),
    >(
        &mut self,
        type_name: S,
        error_name: S1,
        read_function: F,
        write_function: F2,
    ) {
        self.open_curly(format!(
            "impl ReadableAndWritable for {}",
            type_name.as_ref()
        ));
        self.wln(format!(
            "type Error = {err_ty};",
            err_ty = error_name.as_ref()
        ));
        self.newline();

        self.open_curly(
            "fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error>",
        );
        read_function(self);
        self.closing_curly_newline();

        self.open_curly("fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error>");
        write_function(self);
        self.closing_curly_newline();
        self.closing_curly_newline();
    }

    pub fn impl_readable_and_writable<S: AsRef<str>, F: Fn(&mut Self), F2: Fn(&mut Self)>(
        &mut self,
        type_name: S,
        read_function: F,
        write_function: F2,
    ) {
        self.impl_read_and_writable_with_error(
            type_name.as_ref(),
            format!("{t}Error", t = type_name.as_ref()),
            read_function,
            write_function,
        );
    }

    pub fn funcn_pub_const<S: AsRef<str>, S1: AsRef<str>, F: Fn(&mut Self)>(
        &mut self,
        name_and_args: S,
        return_type: S1,
        f: F,
    ) {
        self.open_curly(format!(
            "pub const fn {} -> {}",
            name_and_args.as_ref(),
            return_type.as_ref()
        ));

        f(self);

        self.closing_curly_newline();
    }

    pub fn funcn_pub<S: AsRef<str>, S1: AsRef<str>, F: Fn(&mut Self)>(
        &mut self,
        name_and_args: S,
        return_type: S1,
        f: F,
    ) {
        self.open_curly(format!(
            "pub fn {} -> {}",
            name_and_args.as_ref(),
            return_type.as_ref()
        ));

        f(self);

        self.closing_curly_newline();
    }

    pub fn func_pub<S: AsRef<str>, F: Fn(&mut Self)>(
        &mut self,
        name_and_args: S,
        return_type: impl AsRef<str>,
        f: F,
    ) {
        self.open_curly(format!(
            "pub fn {} -> {}",
            name_and_args.as_ref(),
            return_type.as_ref()
        ));

        f(self);

        self.closing_curly();
    }

    pub fn impl_for<S: AsRef<str>, S2: AsRef<str>, F: Fn(&mut Self)>(
        &mut self,
        s: S,
        s2: S2,
        f: F,
    ) {
        self.open_curly(format!("impl {} for {}", s.as_ref(), s2.as_ref()));

        f(self);

        self.closing_curly_newline();
    }

    pub fn impl_from<S: AsRef<str>, S2: AsRef<str>, F: Fn(&mut Self)>(
        &mut self,
        s: S,
        s2: S2,
        f: F,
    ) {
        self.open_curly(format!("impl From<{}> for {}", s.as_ref(), s2.as_ref()));
        self.open_curly(format!("fn from(e: {}) -> Self", s.as_ref()));

        f(self);

        self.closing_curly();
        self.closing_curly_newline();
    }

    pub fn bodyn<S: AsRef<str>, F: Fn(&mut Self)>(&mut self, s: S, f: F) {
        self.open_curly(s);

        f(self);

        self.closing_curly_newline();
    }

    pub fn body_else_with_closing<
        S: AsRef<str>,
        S1: AsRef<str>,
        F: Fn(&mut Self),
        F1: Fn(&mut Self),
    >(
        &mut self,
        curly_text: S,
        closing: S1,
        if_statement: F,
        else_statement: F1,
    ) {
        self.open_curly(curly_text);
        if_statement(self);
        self.closing_curly_with(" else {");
        self.inc_indent();
        else_statement(self);
        self.closing_curly_with(closing.as_ref());
    }

    pub fn body_else<S: AsRef<str>, F: Fn(&mut Self), F1: Fn(&mut Self)>(
        &mut self,
        s: S,
        if_statement: F,
        else_statement: F1,
    ) {
        self.open_curly(s);
        if_statement(self);
        self.closing_curly_with(" else {");
        self.inc_indent();
        else_statement(self);
        self.closing_curly();
    }

    pub fn body<S: AsRef<str>, F: Fn(&mut Self)>(&mut self, s: S, f: F) {
        self.open_curly(s);

        f(self);

        self.closing_curly();
    }

    pub fn body_closing_with<S: AsRef<str>, S1: AsRef<str>, F: Fn(&mut Self)>(
        &mut self,
        s: S,
        f: F,
        ending: S1,
    ) {
        self.open_curly(s);

        f(self);

        self.closing_curly_with(ending.as_ref());
    }

    pub fn closing_curly(&mut self) {
        self.dec_indent();
        self.wln("}");
    }

    pub fn closing_curly_with<S: AsRef<str>>(&mut self, s: S) {
        self.dec_indent();
        self.wln(format!("}}{}", s.as_ref()));
    }

    pub fn closing_curly_newline(&mut self) {
        self.dec_indent();
        self.wln("}");
        self.newline();
    }

    pub fn dec_indent(&mut self) {
        if self.indentation_level == 0 {
            panic!("attempted to underflow identation level");
        }
        self.indentation_level -= 1;
    }

    pub fn inc_indent(&mut self) {
        if self.indentation_level == 0xff {
            panic!("attempted to overflow identation level");
        }
        self.indentation_level += 1;
    }

    pub fn wln<S: AsRef<str>>(&mut self, s: S) {
        self.w(s);
        self.newline();
    }

    pub fn metadata_comment<S: AsRef<str>>(&mut self, s: S) {
        if !Self::METADATA {
            return;
        }

        self.w(format!("// {}", s.as_ref()));
        self.newline();
    }

    pub fn docc<S: AsRef<str>>(&mut self, s: S) {
        self.w("/// ");
        for _ in 0..self.docc_indentation_level {
            self.w(Self::INDENTATION);
        }
        self.w_no_indent(s.as_ref());
        self.newline();
    }

    pub fn docc_w<S: AsRef<str>>(&mut self, s: S) {
        self.w("/// ");
        for _ in 0..self.docc_indentation_level {
            self.w(Self::INDENTATION);
        }
        self.w_no_indent(s.as_ref());
    }

    pub fn docc_dec(&mut self) {
        if self.docc_indentation_level == 0 {
            panic!("attmpted to underflow docc level")
        }
        self.docc_indentation_level -= 1;
    }

    pub fn docc_inc(&mut self) {
        if self.docc_indentation_level == 0xff {
            panic!("attempted to overflow docc level");
        }
        self.docc_indentation_level += 1;
    }

    pub fn docc_wowm(&mut self, f: impl Fn(&mut Self), repo: impl AsRef<str>, fileinfo: &FileInfo) {
        self.docc(format!(
            "Auto generated from the original `wowm` in file [`{name}:{line}`]({github_repo}/tree/main/{name}#L{line}):",
            name = fileinfo.name(),
            line = fileinfo.start_line(),
            github_repo = repo.as_ref(),
        ));
        self.docc("```text");
        f(self);
        self.docc("```");
    }

    #[allow(unused)]
    pub fn w_no_indent<S: AsRef<str>>(&mut self, s: S) {
        self.inner.write_str(s.as_ref()).unwrap();
    }
    pub fn w_break_at<S: AsRef<str>>(&mut self, s: S, break_at: usize) {
        if self.get_column() >= break_at {
            self.newline();
            self.w(s.as_ref());
        } else {
            self.w_no_indent(s.as_ref());
        }
    }

    pub fn wln_no_indent<S: AsRef<str>>(&mut self, s: S) {
        self.inner.write_str(s.as_ref()).unwrap();
        self.newline();
    }

    pub fn newline(&mut self) {
        self.inner.write_str("\n").unwrap();
    }

    pub fn w<S: AsRef<str>>(&mut self, s: S) {
        for _ in 0..self.indentation_level {
            self.inner.write_str(Self::INDENTATION).unwrap();
        }

        self.inner.write_str(s.as_ref()).unwrap();
    }

    fn get_column(&self) -> usize {
        self.inner.len() - self.inner.rfind(|a| a == '\n').unwrap()
    }

    pub fn proper_as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub fn imports(&self) -> &str {
        self.imports.as_str()
    }
}
