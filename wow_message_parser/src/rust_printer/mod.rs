use heck::CamelCase;
use std::cmp::Ordering;
use std::fmt::Write;

pub(crate) use enums::{print_enum, print_enum_for_base};
pub(crate) use flags::print_flag;
pub(crate) use opcodes::print_login_opcodes;
pub(crate) use opcodes::print_world_opcodes;
pub(crate) use structs::print_struct;

use crate::file_info::FileInfo;

mod enums;
mod flags;
mod opcodes;
pub mod rust_view;
mod structs;
mod update_mask;

use crate::file_utils::{get_import_path, get_shared_module_name, major_version_to_string};
use crate::parser::types::sizes::Sizes;
use crate::parser::types::tags::{LoginVersion, WorldVersion};
use crate::{ContainerType, Objects, Tags};
pub use update_mask::*;

#[derive(Debug)]
pub struct Writer {
    inner: String,
    imports: String,
    indentation_level: u8,
    docc_indentation_level: u8,
    import_path: String,
}

pub const EXPECTED_OPCODE_ERROR: &str = "crate::errors::ExpectedOpcodeError";
pub const PARSE_ERROR: &str = "crate::errors::ParseError";

pub const CLIENT_MESSAGE_TRAIT_NAME: &str = "ClientMessage";
pub const SERVER_MESSAGE_TRAIT_NAME: &str = "ServerMessage";

pub const LOGIN_CLIENT_MESSAGE_ENUM_NAME: &str = "ClientOpcodeMessage";
pub const LOGIN_SERVER_MESSAGE_ENUM_NAME: &str = "ServerOpcodeMessage";
pub const WORLD_CLIENT_MESSAGE_ENUM_NAME: &str = "ClientOpcodeMessage";
pub const WORLD_SERVER_MESSAGE_ENUM_NAME: &str = "ServerOpcodeMessage";

pub const TOKIO_IMPORT: &str = "use tokio::io::{AsyncReadExt, AsyncWriteExt};";
pub const ASYNC_STD_IMPORT: &str = "use async_std::io::{ReadExt, WriteExt};";

const CFG_SYNC: &str = "#[cfg(feature = \"sync\")]";
const CFG_ASYNC_TOKIO: &str = "#[cfg(feature = \"tokio\")]";
const CFG_ASYNC_ASYNC_STD: &str = "#[cfg(feature = \"async-std\")]";

pub(crate) fn get_import_from_shared(name: &str, versions: &[Version]) -> String {
    let mut s = Writer::new(&get_import_path(versions[0]));

    let versions: Vec<WorldVersion> = versions.iter().map(|a| a.as_world()).collect();

    s.wln(format!(
        "pub use crate::shared::{}::{};",
        get_shared_module_name(name, &versions),
        name
    ));
    s.newline();

    s.inner
}

pub(crate) fn get_import_from_base(name: &str, version: Version) -> String {
    let mut s = Writer::new(&get_import_path(version));

    s.wln(format!(
        "pub use wow_world_base::{}::{};",
        major_version_to_string(&version.as_world()),
        name
    ));
    s.newline();

    s.inner
}

impl Writer {
    pub(crate) const INDENTATION: &'static str = "    ";
    const METADATA: bool = true;

    pub(crate) fn new(import_path: &str) -> Self {
        Self {
            inner: String::with_capacity(4000),
            imports: String::with_capacity(4000),
            indentation_level: 0,
            docc_indentation_level: 0,
            import_path: import_path.to_string(),
        }
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

    fn add_import(&mut self, name: impl AsRef<str>) {
        self.imports
            .write_fmt(format_args!(
                "pub use {import_path}::{name};\n",
                import_path = self.import_path,
                name = name.as_ref(),
            ))
            .unwrap();
    }

    pub(crate) fn new_struct(&mut self, name: impl AsRef<str>, f: impl Fn(&mut Self)) {
        self.add_import(&name);

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
        self.add_import(name.as_ref());

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
        self.add_import(&name);

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
        variable_sized: impl Fn(&mut Self),
    ) {
        self.open_curly(format!("impl {}", name.as_ref()));
        self.open_curly("pub(crate) fn size(&self) -> usize");

        variable_sized(self);

        self.closing_curly();
        self.closing_curly_newline();
    }

    pub(crate) fn impl_world_message(
        &mut self,
        type_name: impl AsRef<str>,
        opcode: u16,
        write_function: impl Fn(&mut Self, ImplType),
        read_function: impl Fn(&mut Self, ImplType),
        sizes: Option<Sizes>,
    ) {
        self.open_curly(format!("impl crate::Message for {}", type_name.as_ref()));
        self.wln(format!("const OPCODE: u32 = {:#06x};", opcode));
        self.newline();

        self.open_curly("fn size_without_header(&self) -> u32");
        if sizes.is_some() && sizes.unwrap().is_constant().is_some() {
            self.wln(format!("{}", sizes.unwrap().maximum()));
        } else {
            self.wln("self.size() as u32");
        }
        self.closing_curly(); // size_without_header
        self.newline();

        self.write_into_vec_trait(&write_function);

        self.open_curly(format!(
            "fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, {}>",
            PARSE_ERROR,
        ));

        read_function(self, ImplType::Std);

        self.closing_curly_newline(); // read_body

        self.closing_curly(); // impl Message
    }

    pub(crate) fn impl_world_server_or_client_message(
        &mut self,
        type_name: impl AsRef<str>,
        container_type: ContainerType,
        version: Version,
    ) {
        let trait_to_impl = match container_type {
            ContainerType::CMsg(_) => CLIENT_MESSAGE_TRAIT_NAME,
            ContainerType::SMsg(_) => SERVER_MESSAGE_TRAIT_NAME,
            _ => unreachable!(),
        };

        self.wln(format!(
            "#[cfg(feature = \"{}\")]",
            version.as_major_world().feature_name()
        ));
        self.wln(format!(
            "impl {}::{} for {} {{}}",
            get_import_path(version),
            trait_to_impl,
            type_name.as_ref(),
        ));
        self.newline();
    }

    fn print_write_decl(&mut self, it: ImplType) {
        self.wln(it.cfg());

        if !it.is_async() {
            self.open_curly(format!("fn {prefix}write<W: {write}>(&self, w: &mut W) -> std::result::Result<(), std::io::Error>",
                                    prefix = it.prefix(),
                                    write = it.write(),
            ));

            return;
        }

        self.wln(format!(
            "fn {prefix}write<'life0, 'life1, 'async_trait, W>(",
            prefix = it.prefix(),
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

    fn print_read_decl(&mut self, it: ImplType) {
        if !it.is_async() {
            self.open_curly(format!(
                "fn {prefix}read<R: {read}>(r: &mut R) -> std::result::Result<Self, {error}>",
                prefix = it.prefix(),
                read = it.read(),
                error = PARSE_ERROR,
            ));

            return;
        }

        self.wln(it.cfg());
        self.wln(format!("fn {}read<'life0, 'async_trait, R>(", it.prefix()));

        self.inc_indent();
        self.wln("r: &'life0 mut R,");
        self.dec_indent();

        self.wln(") -> core::pin::Pin<Box<");

        self.inc_indent();
        self.wln(format!(
            "dyn core::future::Future<Output = std::result::Result<Self, {}>>",
            PARSE_ERROR,
        ));
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

    fn call_as_bytes(&mut self, it: ImplType, sizes: Sizes) {
        let size = if let Some(size) = sizes.is_constant() {
            size.to_string()
        } else {
            "self.size() + 1".to_string()
        };

        self.wln(format!("let mut v = Vec::with_capacity({});", size));
        self.wln("self.write_into_vec(&mut v)?;");
        self.wln(format!("w.write_all(&v){postfix}", postfix = it.postfix()));
    }

    pub(crate) fn write_into_vec_trait(&mut self, write_function: impl Fn(&mut Self, ImplType)) {
        self.open_curly("fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error>");

        write_function(self, ImplType::Std);

        self.wln("Ok(())");

        self.closing_curly();
    }

    pub(crate) fn write_into_vec(
        &mut self,
        type_name: impl AsRef<str>,
        write_function: impl Fn(&mut Self, ImplType),
    ) {
        self.open_curly(format!("impl {}", type_name.as_ref()));

        self.open_curly(
            "pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error>",
        );

        write_function(self, ImplType::Std);

        self.wln("Ok(())");

        self.closing_curly();
        self.closing_curly_newline();
    }

    pub(crate) fn impl_read_write_non_trait(
        &mut self,
        type_name: impl AsRef<str>,
        error_name: impl AsRef<str>,
        read_function: impl Fn(&mut Self, ImplType),
        write_function: impl Fn(&mut Self, ImplType),
        visibility: impl AsRef<str>,
        create_async_reads: bool,
    ) {
        self.write_into_vec(&type_name, write_function);

        self.open_curly(format!("impl {}", type_name.as_ref()));

        for it in ImplType::types() {
            if it.is_async() {
                if !create_async_reads {
                    continue;
                }

                self.wln(it.cfg());
            }
            self.open_curly(format!(
                "{visibility} {func}fn {prefix}read<R: {read}>(r: &mut R) -> std::result::Result<Self, {error}>",
                prefix = it.prefix(),
                read = it.read(),
                error = error_name.as_ref(),
                func = it.func(),
                visibility = visibility.as_ref(),
            ));
            read_function(self, it);
            self.closing_curly_newline();
        }

        self.closing_curly_newline(); // impl
    }

    pub(crate) fn impl_read_write_opcode(
        &mut self,
        type_name: impl AsRef<str>,
        error_name: impl AsRef<str>,
        read_function: impl Fn(&mut Self, ImplType),
        write_function: impl Fn(&mut Self, ImplType),
        create_async_reads: bool,
    ) {
        self.impl_read_write_non_trait(
            type_name,
            error_name,
            read_function,
            write_function,
            "pub",
            create_async_reads,
        )
    }

    pub(crate) fn impl_read_write_struct(
        &mut self,
        type_name: impl AsRef<str>,
        read_function: impl Fn(&mut Self, ImplType),
        write_function: impl Fn(&mut Self, ImplType),
        create_async_reads: bool,
    ) {
        self.impl_read_write_non_trait(
            type_name,
            PARSE_ERROR,
            read_function,
            write_function,
            "pub(crate)",
            create_async_reads,
        )
    }

    pub(crate) fn impl_read_and_writable_login(
        &mut self,
        type_name: impl AsRef<str>,
        opcode: u16,
        trait_to_impl: impl AsRef<str>,
        read_function: impl Fn(&mut Self, ImplType),
        write_function: impl Fn(&mut Self, ImplType),
        sizes: Sizes,
    ) {
        self.write_into_vec(&type_name, write_function);

        self.open_curly(format!(
            "impl {} for {}",
            trait_to_impl.as_ref(),
            type_name.as_ref(),
        ));
        self.wln(format!("const OPCODE: u8 = {:#04x};", opcode));
        self.newline();

        for it in ImplType::types() {
            self.print_read_decl(it);

            read_function(self, it);
            if it.is_async() {
                self.closing_curly_with(")"); // Box::pin
            }
            self.closing_curly_newline();

            self.print_write_decl(it);

            self.call_as_bytes(it, sizes);

            if it.is_async() {
                self.closing_curly_with(")"); // Box::pin
            }

            self.closing_curly_newline(); // Write Function
        }

        self.closing_curly_newline(); // impl
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

    #[allow(unused)]
    pub(crate) fn w_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
    }
    pub(crate) fn w_break_at(&mut self, s: impl AsRef<str>, break_at: usize) {
        if self.get_column() >= break_at {
            self.newline();
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
        self.inner.len() - self.inner.rfind(|a| a == '\n').unwrap()
    }

    pub(crate) fn proper_as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub(crate) fn imports(&self) -> &str {
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
            ImplType::Std => "std::io::Write",
            ImplType::Tokio => "tokio::io::AsyncWriteExt + Unpin + Send",
            ImplType::AsyncStd => "async_std::io::WriteExt + Unpin + Send",
        }
    }

    pub(crate) fn read(&self) -> &str {
        match self {
            ImplType::Std => "std::io::Read",
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
pub enum Version {
    Login(LoginVersion),
    World(WorldVersion),
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Version::Login(l) => match other {
                Version::Login(ol) => l.cmp(ol),
                Version::World(_) => Ordering::Less,
            },
            Version::World(w) => match other {
                Version::Login(_) => Ordering::Greater,
                Version::World(ow) => w.cmp(ow),
            },
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Version {
    pub(crate) fn is_world(&self) -> bool {
        match self {
            Version::Login(_) => false,
            Version::World(_) => true,
        }
    }

    pub(crate) fn as_world(&self) -> WorldVersion {
        match self {
            Version::Login(_) => unreachable!(),
            Version::World(w) => *w,
        }
    }

    pub(crate) fn as_major_world(&self) -> MajorWorldVersion {
        match self.as_world() {
            WorldVersion::Major(m) => match m {
                1 => MajorWorldVersion::Vanilla,
                2 => MajorWorldVersion::BurningCrusade,
                3 => MajorWorldVersion::Wrath,
                _ => unreachable!(),
            },
            WorldVersion::Minor(m, i) => match (m, i) {
                (1, 12) => MajorWorldVersion::Vanilla,
                (2, 4) => MajorWorldVersion::BurningCrusade,
                (3, 3) => MajorWorldVersion::Wrath,
                _ => unreachable!(),
            },
            WorldVersion::Patch(m, i, p) => match (m, i, p) {
                (1, 12, _) => MajorWorldVersion::Vanilla,
                (2, 4, 3) => MajorWorldVersion::BurningCrusade,
                (3, 3, 5) => MajorWorldVersion::Wrath,
                _ => unreachable!(),
            },
            WorldVersion::Exact(m, i, p, e) => match (m, i, p, e) {
                (1, 12, _, _) => MajorWorldVersion::Vanilla,
                (2, 4, 3, 8606) => MajorWorldVersion::BurningCrusade,
                (3, 3, 5, 12340) => MajorWorldVersion::Wrath,
                _ => unreachable!(),
            },
            WorldVersion::All => unreachable!(),
        }
    }

    pub(crate) fn to_module_case(self) -> String {
        match self {
            Version::Login(l) => l.to_module_case(),
            Version::World(l) => l.to_module_case(),
        }
    }
}

impl From<LoginVersion> for Version {
    fn from(l: LoginVersion) -> Self {
        Self::Login(l)
    }
}

impl From<WorldVersion> for Version {
    fn from(l: WorldVersion) -> Self {
        Self::World(l)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MajorWorldVersion {
    Vanilla,
    BurningCrusade,
    Wrath,
}

impl MajorWorldVersion {
    pub(crate) fn encryption_path(&self) -> &'static str {
        match self {
            MajorWorldVersion::Vanilla => "wow_srp::vanilla_header",
            MajorWorldVersion::BurningCrusade => "wow_srp::tbc_header",
            MajorWorldVersion::Wrath => "wow_srp::wrath_header",
        }
    }

    pub(crate) fn module_name(&self) -> &'static str {
        self.feature_name()
    }

    pub(crate) fn feature_name(&self) -> &'static str {
        match self {
            MajorWorldVersion::Vanilla => "vanilla",
            MajorWorldVersion::BurningCrusade => "tbc",
            MajorWorldVersion::Wrath => "wrath",
        }
    }

    pub(crate) fn wrath_or_greater(&self) -> bool {
        match self {
            MajorWorldVersion::Vanilla | MajorWorldVersion::BurningCrusade => false,
            MajorWorldVersion::Wrath => true,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DefinerType {
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

fn print_docc_description_and_comment(
    s: &mut Writer,
    tags: &Tags,
    o: &Objects,
    object_tags: &Tags,
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
    let name = s.to_camel_case();
    if name == "Self" {
        "SelfX".to_string() // Self is a reserved keyword
    } else if name == "Error" {
        "ErrorX".to_string() // Makes it ambiguous with Self::Error in traits
    } else {
        name
    }
}
