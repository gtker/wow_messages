use std::fmt::Write;

pub use enums::print_enum;
pub use flags::print_flag;
pub use opcodes::print_login_opcodes;
pub use opcodes::print_world_opcodes;
pub use structs::print_struct;

use crate::file_info::FileInfo;

pub mod complex_print;
mod enums;
mod flags;
pub mod new_enums;
mod opcodes;
pub mod rust_view;
mod structs;

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
pub const WORLD_CLIENT_MESSAGE_ENUM_NAME: &str = "ClientOpcodeMessage";
pub const WORLD_SERVER_MESSAGE_ENUM_NAME: &str = "ServerOpcodeMessage";

pub const WORLD_BODY_TRAIT_NAME: &str = "MessageBody";
pub const WORLD_CLIENT_HEADER_TRAIT_NAME: &str = "ClientMessageWrite";
pub const WORLD_SERVER_HEADER_TRAIT_NAME: &str = "ServerMessageWrite";
pub const OPCODE_MESSAGE_TRAIT_NAME: &str = "OpcodeMessage";

pub const TOKIO_IMPORT: &str = "use tokio::io::{AsyncReadExt, AsyncWriteExt};";
pub const ASYNC_STD_IMPORT: &str = "use async_std::io::{ReadExt, WriteExt};";

const CFG_SYNC: &str = "#[cfg(feature = \"sync\")]";
const CFG_ASYNC_TOKIO: &str = "#[cfg(feature = \"async_tokio\")]";
const CFG_ASYNC_ASYNC_STD: &str = "#[cfg(feature = \"async_std\")]";

impl Writer {
    pub(crate) const INDENTATION: &'static str = "    ";
    const METADATA: bool = false;

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

    pub fn new_enum(
        &mut self,
        visibility: impl AsRef<str>,
        name: impl AsRef<str>,
        f: impl Fn(&mut Self),
    ) {
        self.add_import(&name);

        self.open_curly(format!(
            "{visibility} enum {name}",
            visibility = visibility.as_ref(),
            name = name.as_ref()
        ));

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
        self.wln(format!("impl ConstantSized for {} {{}}", name.as_ref()));
        self.newline();

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
        opcode: u16,
        is_constant_sized: bool,
        read_function: impl Fn(&mut Self, ImplType),
        write_function: impl Fn(&mut Self, ImplType),
    ) {
        self.open_curly(format!(
            "impl {} for {}",
            WORLD_BODY_TRAIT_NAME,
            type_name.as_ref()
        ));

        self.wln(format!("const OPCODE: u16 = {:#06x};", opcode));

        self.newline();

        self.open_curly("fn size_without_size_or_opcode_fields(&self) -> u16");
        self.wln(format!(
            "{}size() as u16",
            match is_constant_sized {
                true => "Self::",
                false => "self.",
            }
        ));
        self.closing_curly();

        self.newline();
        self.wln(format!(
            "type Error = {err_ty};",
            err_ty = error_name.as_ref()
        ));
        self.newline();

        for it in ImplType::types() {
            self.print_read_decl(it, "_body");

            read_function(self, it);

            if it.is_async() {
                self.closing_curly_with(")"); // Box::pin
            }
            self.closing_curly_newline();

            self.print_write_decl(it, "_body");

            write_function(self, it);

            if it.is_async() {
                self.closing_curly_with(")"); // Box::pin
            }
            self.closing_curly_newline();
        }

        self.closing_curly_newline(); // impl
    }

    fn print_read_encrypted(&mut self, it: ImplType) {
        if !it.is_async() {
            self.wln(format!("{}", it.cfg()));
            self.open_curly(format!("{func}fn {prefix}read_encrypted<R: {read}, D: Decrypter{decrypter}>(r: &mut R, d: &mut D) -> std::result::Result<Self, Self::Error>", func = it.func(), prefix = it.prefix(), read = it.read(), decrypter = it.decrypter()));

            return;
        }

        self.wln(format!(
            "fn {prefix}read_encrypted<'life0, 'life1, 'async_trait, R, D>(",
            prefix = it.prefix()
        ));

        self.inc_indent();
        self.wln("r: &'life0 mut R,");
        self.wln("d: &'life1 mut D,");
        self.dec_indent();

        self.wln(") -> core::pin::Pin<Box<");
        self.inc_indent();

        self.wln("dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>");

        self.inc_indent();
        self.wln("+ Send + 'async_trait");

        self.dec_indent();
        self.dec_indent();
        self.wln(">> where");

        self.inc_indent();
        self.wln(format!("R: 'async_trait + {},", it.read()));
        self.wln(format!("D: 'async_trait + Decrypter{},", it.decrypter()));
        self.wln("'life0: 'async_trait,");
        self.wln("'life1: 'async_trait,");
        self.wln("Self: 'async_trait,");
        self.dec_indent();

        self.open_curly("");
        self.open_curly("Box::pin(async move");
    }

    fn print_write_encrypted(&mut self, it: ImplType) {
        if !it.is_async() {
            self.wln(format!("{}", it.cfg()));
            self.open_curly(format!("{func}fn {prefix}write_encrypted<W: {write}, E: Encrypter{decrypter}>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error>", decrypter = it.decrypter(), prefix = it.prefix(), func = it.func(), write = it.write()));

            return;
        }

        self.wln(format!(
            "fn {prefix}write_encrypted<'life0, 'life1, 'life2, 'async_trait, W, E>(",
            prefix = it.prefix()
        ));

        self.inc_indent();
        self.wln("&'life0 self,");
        self.wln("w: &'life1 mut W,");
        self.wln("e: &'life2 mut E,");
        self.dec_indent();

        self.wln(") -> core::pin::Pin<Box<");
        self.inc_indent();

        self.wln("dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>");

        self.inc_indent();
        self.wln("+ Send + 'async_trait");

        self.dec_indent();
        self.dec_indent();
        self.wln(">> where");

        self.inc_indent();
        self.wln(format!("W: 'async_trait + {},", it.write()));
        self.wln(format!("E: 'async_trait + Encrypter{},", it.decrypter()));
        self.wln("'life0: 'async_trait,");
        self.wln("'life1: 'async_trait,");
        self.wln("'life2: 'async_trait,");
        self.wln("Self: 'async_trait,");
        self.dec_indent();

        self.open_curly("");
        self.open_curly("Box::pin(async move");
    }

    fn print_write_decl(&mut self, it: ImplType, world_text: &str) {
        if !it.is_async() {
            self.wln(it.cfg());
            self.open_curly(format!("fn {prefix}write{world_text}<W: {write}>(&self, w: &mut W) -> std::result::Result<(), std::io::Error>",
                                    prefix = it.prefix(),
                                    write = it.write(),
                                    world_text = world_text,
            ));

            return;
        }

        self.wln(format!(
            "fn {prefix}write{world_text}<'life0, 'life1, 'async_trait, W>(",
            prefix = it.prefix(),
            world_text = world_text,
        ));
        self.inc_indent();

        self.wln("&'life0 self,");
        self.wln("w: &'life1 mut W,");
        self.dec_indent();

        self.wln(") -> core::pin::Pin<Box<");
        self.inc_indent();

        self.wln("dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>");
        self.inc_indent();

        self.wln("+ Send + 'async_trait");
        self.dec_indent();
        self.dec_indent();

        self.wln(">> where");

        self.inc_indent();
        self.wln(format!("W: 'async_trait + {},", it.write()));
        self.wln("'life0: 'async_trait,");
        self.wln("'life1: 'async_trait,");
        self.wln("Self: 'async_trait,");
        self.dec_indent();

        self.open_curly("");
        self.open_curly("Box::pin(async move");
    }

    fn print_read_decl(&mut self, it: ImplType, world_text: &str) {
        if !it.is_async() {
            self.wln(it.cfg());
            let body_size = if world_text == "_body" {
                ", body_size: u32"
            } else {
                ""
            };
            self.open_curly(format!(
                "fn {prefix}read{world_text}<R: {read}>(r: &mut R{body_size}) -> std::result::Result<Self, Self::Error>",
                world_text = world_text,
                prefix = it.prefix(),
                read = it.read(),
                body_size = body_size,
            ));

            return;
        }

        self.wln(format!(
            "fn {}read{}<'life0, 'async_trait, R>(",
            it.prefix(),
            world_text
        ));

        self.inc_indent();
        self.wln("r: &'life0 mut R,");
        if world_text == "_body" {
            self.wln("body_size: u32,");
        }
        self.dec_indent();

        self.wln(") -> core::pin::Pin<Box<");

        self.inc_indent();
        self.wln("dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>");
        self.inc_indent();

        self.wln("+ Send + 'async_trait,");
        self.dec_indent();
        self.dec_indent();

        self.wln(">> where");

        self.inc_indent();
        self.wln(format!("R: 'async_trait + {},", it.read()));
        self.wln("'life0: 'async_trait,");
        self.wln("Self: 'async_trait,");
        self.dec_indent();

        self.open_curly("");
        self.open_curly("Box::pin(async move");
    }

    pub fn impl_read_and_writable_with_error<
        S: AsRef<str>,
        S1: AsRef<str>,
        F: Fn(&mut Self, ImplType),
        F2: Fn(&mut Self, ImplType),
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

        for it in ImplType::types() {
            self.print_read_decl(it, "");

            read_function(self, it);
            if it.is_async() {
                self.closing_curly_with(")"); // Box::pin
            }
            self.closing_curly_newline();

            self.print_write_decl(it, "");

            write_function(self, it);

            if it.is_async() {
                self.closing_curly_with(")"); // Box::pin
            }

            self.closing_curly_newline(); // Write Function
        }

        self.closing_curly_newline(); // impl
    }

    pub fn impl_readable_and_writable<
        S: AsRef<str>,
        F: Fn(&mut Self, ImplType),
        F2: Fn(&mut Self, ImplType),
    >(
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

    pub fn async_funcn_pub<S: AsRef<str>, S1: AsRef<str>, F: Fn(&mut Self, ImplType)>(
        &mut self,
        name: impl AsRef<str>,
        args_sync: S,
        args_tokio: S,
        args_astd: S,
        return_type: S1,
        f: F,
    ) {
        self.wln(CFG_SYNC);
        self.open_curly(format!(
            "pub fn {}{} -> {}",
            name.as_ref(),
            args_sync.as_ref(),
            return_type.as_ref()
        ));

        f(self, ImplType::Std);

        self.closing_curly_newline();

        self.wln(CFG_ASYNC_TOKIO);
        self.open_curly(format!(
            "pub async fn {prefix}{name}{args} -> {return_value}",
            prefix = ImplType::Tokio.prefix(),
            name = name.as_ref(),
            args = args_tokio.as_ref(),
            return_value = return_type.as_ref()
        ));

        f(self, ImplType::Tokio);

        self.closing_curly_newline();

        self.wln(CFG_ASYNC_ASYNC_STD);
        self.open_curly(format!(
            "pub async fn {prefix}{name}{args} -> {return_value}",
            prefix = ImplType::AsyncStd.prefix(),
            name = name.as_ref(),
            args = args_astd.as_ref(),
            return_value = return_type.as_ref()
        ));

        f(self, ImplType::AsyncStd);

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

    pub fn write_async_includes(&mut self) {
        self.wln(CFG_ASYNC_TOKIO);
        self.wln(TOKIO_IMPORT);

        self.wln(CFG_ASYNC_ASYNC_STD);
        self.wln(ASYNC_STD_IMPORT);
    }

    pub fn metadata_comment<S: AsRef<str>>(&mut self, s: S) {
        if !Self::METADATA {
            return;
        }

        self.w(format!("// {}", s.as_ref()));
        self.newline();
    }

    pub fn docc<S: AsRef<str>>(&mut self, s: S) {
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

    pub fn docc_wowm(&mut self, f: impl Fn(&mut Self), fileinfo: &FileInfo) {
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

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum ImplType {
    Std,
    Tokio,
    AsyncStd,
}

impl ImplType {
    pub fn postfix(&self) -> &str {
        match self {
            ImplType::Std => "",
            _ => ".await",
        }
    }

    pub fn prefix(&self) -> &str {
        match self {
            ImplType::Std => "",
            ImplType::Tokio => "tokio_",
            ImplType::AsyncStd => "astd_",
        }
    }

    pub fn func(&self) -> &str {
        match self {
            ImplType::Std => "",
            ImplType::Tokio | ImplType::AsyncStd => "async ",
        }
    }

    pub fn write(&self) -> &str {
        match self {
            ImplType::Std => "std::io::Write",
            ImplType::Tokio => "AsyncWriteExt + Unpin + Send",
            ImplType::AsyncStd => "WriteExt + Unpin + Send",
        }
    }

    pub fn read(&self) -> &str {
        match self {
            ImplType::Std => "std::io::Read",
            ImplType::Tokio => "AsyncReadExt + Unpin + Send",
            ImplType::AsyncStd => "ReadExt + Unpin + Send",
        }
    }

    pub fn decrypter(&self) -> &str {
        match self {
            ImplType::Std => "",
            ImplType::AsyncStd | ImplType::Tokio => " + Send",
        }
    }

    pub fn cfg(&self) -> &str {
        match self {
            ImplType::Std => CFG_SYNC,
            ImplType::Tokio => CFG_ASYNC_TOKIO,
            ImplType::AsyncStd => CFG_ASYNC_ASYNC_STD,
        }
    }

    pub fn test_macro(&self) -> &str {
        match self {
            ImplType::Std => "#[cfg_attr(feature = \"sync\", test)]",
            ImplType::Tokio => "#[cfg_attr(feature = \"async_tokio\", tokio::test)]",
            ImplType::AsyncStd => "#[cfg_attr(feature = \"async_std\", async_std::test)]",
        }
    }

    pub fn is_async(&self) -> bool {
        match self {
            ImplType::Std => false,
            _ => true,
        }
    }

    pub fn types() -> Vec<Self> {
        vec![ImplType::Std, ImplType::Tokio, ImplType::AsyncStd]
    }
}
