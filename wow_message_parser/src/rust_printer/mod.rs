use heck::ToPascalCase;
use std::fmt::Write;

pub(crate) use enums::{print_enum, print_enum_for_base};
pub(crate) use flags::print_flag;
pub(crate) use opcodes::print_login_opcodes;
pub(crate) use opcodes::print_world_opcodes;
pub(crate) use structs::print_struct;

use crate::file_info::FileInfo;

mod enums;
mod flags;
mod opcode_to_name;
mod opcodes;
pub mod rust_view;
mod structs;
mod update_mask;

use crate::parser::types::sizes::Sizes;
use crate::parser::types::tags::MemberTags;
use crate::{ObjectTags, Objects};
pub use opcode_to_name::print_opcode_to_name;
pub use update_mask::*;

#[derive(Debug, Clone)]
pub(crate) struct Writer {
    inner: String,
    prefix: String,
    indentation_level: u8,
    docc_indentation_level: u8,
}

pub(crate) const EXPECTED_OPCODE_ERROR: &str = "crate::errors::ExpectedOpcodeError";
pub(crate) const PARSE_ERROR: &str = "crate::errors::ParseError";

pub(crate) const CLIENT_MESSAGE_TRAIT_NAME: &str = "ClientMessage";
pub(crate) const SERVER_MESSAGE_TRAIT_NAME: &str = "ServerMessage";

pub(crate) const LOGIN_CLIENT_MESSAGE_ENUM_NAME: &str = "ClientOpcodeMessage";
pub(crate) const LOGIN_SERVER_MESSAGE_ENUM_NAME: &str = "ServerOpcodeMessage";
pub(crate) const WORLD_CLIENT_MESSAGE_ENUM_NAME: &str = "ClientOpcodeMessage";
pub(crate) const WORLD_SERVER_MESSAGE_ENUM_NAME: &str = "ServerOpcodeMessage";

pub(crate) const SYNC_IMPORT: &str = "use std::io::{Read, Write};";
pub(crate) const TOKIO_IMPORT: &str = "use tokio::io::{AsyncReadExt, AsyncWriteExt};";
pub(crate) const ASYNC_STD_IMPORT: &str = "use async_std::io::{ReadExt, WriteExt};";

const CFG_SYNC_AND_ENCRYPTION: &str = "#[cfg(all(feature = \"sync\", feature = \"encryption\"))]";
const CFG_ASYNC_TOKIO_AND_ENCRYPTION: &str =
    "#[cfg(all(feature = \"tokio\", feature = \"encryption\"))]";
const CFG_ASYNC_ASYNC_STD_AND_ENCRYPTION: &str =
    "#[cfg(all(feature = \"async-std\", feature = \"encryption\"))]";

const CFG_SYNC: &str = "#[cfg(feature = \"sync\")]";
const CFG_ASYNC_TOKIO: &str = "#[cfg(feature = \"tokio\")]";
const CFG_ASYNC_ASYNC_STD: &str = "#[cfg(feature = \"async-std\")]";

impl Writer {
    pub(crate) const INDENTATION: &'static str = "    ";
    const METADATA: bool = true;

    pub(crate) fn new() -> Self {
        Self {
            inner: String::with_capacity(4000),
            prefix: "".to_string(),
            indentation_level: 0,
            docc_indentation_level: 0,
        }
    }

    pub(crate) fn with_prefix(prefix: &str) -> Self {
        Self {
            inner: String::with_capacity(4000),
            prefix: prefix.to_string(),
            indentation_level: 0,
            docc_indentation_level: 0,
        }
    }

    pub(crate) fn into_inner(self) -> String {
        self.inner
    }

    pub(crate) fn inner(&self) -> &str {
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

    pub(crate) fn variable_size(
        &mut self,
        name: impl AsRef<str>,
        function_name: &str,
        const_fn: bool,
        variable_sized: impl Fn(&mut Self),
    ) {
        self.open_curly(format!("impl {}", name.as_ref()));
        let const_fn = match const_fn {
            true => " const",
            false => "",
        };
        self.open_curly(format!(
            "pub(crate){const_fn} fn {function_name}(&self) -> usize"
        ));

        variable_sized(self);

        self.closing_curly();
        self.closing_curly_newline();
    }

    fn print_write_decl(&mut self, it: ImplType) {
        self.wln(it.cfg());

        if !it.is_async() {
            self.open_curly(format!(
                "fn {prefix}write<W: {write}>(&self, mut w: W) -> Result<(), std::io::Error>",
                prefix = it.prefix(),
                write = it.write(),
            ));

            return;
        }

        self.wln(format!(
            "fn {prefix}write<'life0, 'async_trait, W>(",
            prefix = it.prefix(),
        ));
        self.inc_indent();

        self.wln("&'life0 self,");
        self.wln("mut w: W,");
        self.dec_indent();

        self.wln(") -> core::pin::Pin<Box<");
        self.inc_indent();

        self.wln("dyn core::future::Future<Output = Result<(), std::io::Error>>");
        self.inc_indent();

        self.wln("+ Send + 'async_trait");
        self.dec_indent();
        self.dec_indent();

        self.wln(">> where");

        self.inc_indent();
        self.wln(format!("W: 'async_trait + {},", it.write()));
        self.wln("'life0: 'async_trait,");
        self.wln("Self: 'async_trait,");
        self.dec_indent();

        self.open_curly("");
        self.open_curly("Box::pin(async move");
    }

    fn print_read_decl(&mut self, it: ImplType) {
        if !it.is_async() {
            self.open_curly(format!(
                "fn {prefix}read<R: {read}, I: crate::private::Sealed>(mut r: R) -> Result<Self, {error}>",
                prefix = it.prefix(),
                read = it.read(),
                error = PARSE_ERROR,
            ));

            return;
        }

        self.wln(it.cfg());
        self.wln(format!(
            "fn {}read<'async_trait, R, I: crate::private::Sealed>(",
            it.prefix()
        ));

        self.inc_indent();
        self.wln("mut r: R,");
        self.dec_indent();

        self.wln(") -> core::pin::Pin<Box<");

        self.inc_indent();
        self.wln(format!(
            "dyn core::future::Future<Output = Result<Self, {PARSE_ERROR}>>",
        ));
        self.inc_indent();

        self.wln("+ Send + 'async_trait,");
        self.dec_indent();
        self.dec_indent();

        self.wln(">> where");

        self.inc_indent();
        self.wln(format!("R: 'async_trait + {},", it.read()));
        self.wln("Self: 'async_trait,");
        self.dec_indent();

        self.open_curly("");
        self.open_curly("Box::pin(async move");
    }

    fn call_as_bytes(&mut self, it: ImplType, sizes: Sizes) {
        let size = if let Some(size) = sizes.is_constant() {
            size.to_string()
        } else {
            "self.size() + 1".to_string()
        };

        self.wln(format!("let mut v = Vec::with_capacity({size});"));
        self.wln("self.write_into_vec(&mut v)?;");
        self.wln(format!("w.write_all(&v){postfix}", postfix = it.postfix()));
    }

    pub(crate) fn write_into_vec(
        &mut self,
        type_name: impl AsRef<str>,
        write_function: impl Fn(&mut Self, ImplType),
        visibility: &str,
    ) {
        self.open_curly(format!("impl {}", type_name.as_ref()));

        self.open_curly(format!(
            "{visibility} fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error>",
        ));

        write_function(self, ImplType::Std);

        self.wln("Ok(())");

        self.closing_curly();
        self.closing_curly_newline();
    }

    pub fn constructor(
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

    pub fn pub_const_fn(
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
    pub(crate) fn w_break_at(&mut self, s: impl AsRef<str>, break_at: usize) {
        let column = self.get_column();
        if column >= break_at {
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

    fn write_prefix(&mut self) {
        self.inner.write_str(&self.prefix).unwrap();
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum ImplType {
    Std,
    Tokio,
    AsyncStd,
}

impl ImplType {
    pub(crate) fn postfix(&self) -> &str {
        match self {
            ImplType::Std => "",
            _ => ".await",
        }
    }

    pub(crate) fn prefix(&self) -> &str {
        match self {
            ImplType::Std => "",
            ImplType::Tokio => "tokio_",
            ImplType::AsyncStd => "astd_",
        }
    }

    pub(crate) fn func(&self) -> &str {
        match self {
            ImplType::Std => "",
            ImplType::Tokio | ImplType::AsyncStd => "async ",
        }
    }

    pub(crate) fn write(&self) -> &str {
        match self {
            ImplType::Std => "Write",
            ImplType::Tokio => "tokio::io::AsyncWriteExt + Unpin + Send",
            ImplType::AsyncStd => "async_std::io::WriteExt + Unpin + Send",
        }
    }

    pub(crate) fn read(&self) -> &str {
        match self {
            ImplType::Std => "Read",
            ImplType::Tokio => "tokio::io::AsyncReadExt + Unpin + Send",
            ImplType::AsyncStd => "async_std::io::ReadExt + Unpin + Send",
        }
    }

    pub(crate) fn cfg(&self) -> &str {
        match self {
            ImplType::Std => CFG_SYNC,
            ImplType::Tokio => CFG_ASYNC_TOKIO,
            ImplType::AsyncStd => CFG_ASYNC_ASYNC_STD,
        }
    }

    pub(crate) fn cfg_and_encryption(&self) -> &str {
        match self {
            ImplType::Std => CFG_SYNC_AND_ENCRYPTION,
            ImplType::Tokio => CFG_ASYNC_TOKIO_AND_ENCRYPTION,
            ImplType::AsyncStd => CFG_ASYNC_ASYNC_STD_AND_ENCRYPTION,
        }
    }

    pub(crate) fn test_macro(&self) -> &str {
        match self {
            ImplType::Std => "#[cfg_attr(feature = \"sync\", test)]",
            ImplType::Tokio => "#[cfg_attr(feature = \"tokio\", tokio::test)]",
            ImplType::AsyncStd => "#[cfg_attr(feature = \"async-std\", async_std::test)]",
        }
    }

    pub(crate) fn is_async(&self) -> bool {
        !matches!(self, ImplType::Std)
    }

    pub(crate) fn types() -> Vec<Self> {
        vec![ImplType::Std, ImplType::Tokio, ImplType::AsyncStd]
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum DefinerType {
    Enum,
    Flag,
}

fn get_new_type_name(original_ty: &str, definer_ty: &str) -> String {
    format!("{original_ty}_{definer_ty}")
}

fn get_optional_type_name(original_ty: &str, optional_name: &str) -> String {
    format!("{original_ty}_{optional_name}")
}

fn get_new_flag_type_name(original_ty: &str, enumerator_name: &str) -> String {
    format!("{original_ty}_{enumerator_name}")
}

fn print_member_docc_description_and_comment(
    s: &mut Writer,
    tags: &MemberTags,
    o: &Objects,
    object_tags: &ObjectTags,
) {
    if let Some(description) = tags.description() {
        for line in description.as_rust_doc_lines(o, object_tags) {
            s.docc(line);
        }

        s.docc_newline();
    }

    if let Some(comment) = tags.comment() {
        for line in comment.as_rust_doc_lines(o, object_tags) {
            s.docc(line);
        }

        s.docc_newline();
    }
}

fn print_serde_derive(s: &mut Writer, in_base: bool) {
    if in_base {
        s.wln("#[cfg_attr(feature = \"serde\", derive(serde::Deserialize, serde::Serialize))]");
    }
}

fn print_docc_description_and_comment(
    s: &mut Writer,
    tags: &ObjectTags,
    o: &Objects,
    object_tags: &ObjectTags,
) {
    if let Some(description) = tags.description() {
        for line in description.as_rust_doc_lines(o, object_tags) {
            s.docc(line);
        }

        s.docc_newline();
    }

    if let Some(comment) = tags.comment() {
        for line in comment.as_rust_doc_lines(o, object_tags) {
            s.docc(line);
        }

        s.docc_newline();
    }
}

pub(crate) fn field_name_to_rust_name(s: &str) -> String {
    let name = s.to_pascal_case();
    if name == "Self" {
        "SelfX".to_string() // Self is a reserved keyword
    } else if name == "Error" {
        "ErrorX".to_string() // Makes it ambiguous with Self::Error in traits
    } else {
        name
    }
}
