use heck::ToPascalCase;

pub(crate) use enums::{print_enum, print_enum_for_base};
pub(crate) use flags::print_flag;
pub(crate) use opcodes::print_login_opcodes;
pub(crate) use opcodes::print_world_opcodes;
pub(crate) use structs::print_struct;

mod base_structs;
mod enums;
mod expected;
mod flags;
mod opcode_to_name;
mod opcodes;
pub mod rust_view;
mod structs;
mod update_mask;
pub(crate) mod writer;

use crate::parser::types::tags::MemberTags;
use crate::rust_printer::writer::Writer;
use crate::{ObjectTags, Objects};

pub(crate) use base_structs::print_read_write_base_structs;
pub(crate) use expected::print_expected;
pub use opcode_to_name::print_opcode_to_name;
pub use update_mask::*;

pub(crate) const EXPECTED_OPCODE_ERROR: &str = "crate::errors::ExpectedOpcodeError";
pub(crate) const PARSE_ERROR_KIND: &str = "crate::errors::ParseErrorKind";
pub(crate) const PARSE_ERROR: &str = "crate::errors::ParseError";

pub(crate) const CLIENT_MESSAGE_TRAIT_NAME: &str = "ClientMessage";
pub(crate) const SERVER_MESSAGE_TRAIT_NAME: &str = "ServerMessage";

pub(crate) const LOGIN_CLIENT_MESSAGE_ENUM_NAME: &str = "ClientOpcodeMessage";
pub(crate) const LOGIN_SERVER_MESSAGE_ENUM_NAME: &str = "ServerOpcodeMessage";
pub(crate) const WORLD_CLIENT_MESSAGE_ENUM_NAME: &str = "ClientOpcodeMessage";
pub(crate) const WORLD_SERVER_MESSAGE_ENUM_NAME: &str = "ServerOpcodeMessage";

pub(crate) const SYNC_IMPORT: &str = "use std::io::{Read, Write};";

const CFG_SYNC_AND_ENCRYPTION: &str = "#[cfg(all(feature = \"sync\", feature = \"encryption\"))]";
const CFG_ASYNC_TOKIO_AND_ENCRYPTION: &str =
    "#[cfg(all(feature = \"tokio\", feature = \"encryption\"))]";
const CFG_ASYNC_ASYNC_STD_AND_ENCRYPTION: &str =
    "#[cfg(all(feature = \"async-std\", feature = \"encryption\"))]";

const CFG_TESTCASE: &str = "#[cfg(feature = \"print-testcase\")]";

const CFG_SYNC: &str = "#[cfg(feature = \"sync\")]";
const CFG_ASYNC_TOKIO: &str = "#[cfg(feature = \"tokio\")]";
const CFG_ASYNC_ASYNC_STD: &str = "#[cfg(feature = \"async-std\")]";

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

    pub(crate) fn cfg_name(&self) -> &str {
        match self {
            ImplType::Std => "sync",
            ImplType::Tokio => "tokio",
            ImplType::AsyncStd => "async-std",
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

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum MessageSide {
    Server,
    Client,
}

impl MessageSide {
    pub(crate) fn non_wrath_world_header_size(&self) -> i128 {
        match self {
            MessageSide::Server => 4,
            MessageSide::Client => 6,
        }
    }

    pub(crate) fn world_opcode_size(&self) -> i128 {
        match self {
            MessageSide::Server => 2,
            MessageSide::Client => 4,
        }
    }

    pub(crate) fn is_server(&self) -> bool {
        matches!(self, Self::Server)
    }
    pub(crate) fn values() -> &'static [Self] {
        &[Self::Server, Self::Client]
    }

    pub(crate) fn world_str(&self) -> &'static str {
        match self {
            MessageSide::Server => "server",
            MessageSide::Client => "client",
        }
    }

    pub(crate) fn title_world_str(&self) -> &'static str {
        match self {
            MessageSide::Server => "Server",
            MessageSide::Client => "Client",
        }
    }

    pub(crate) fn opposite_title_world_str(&self) -> &'static str {
        match self {
            MessageSide::Server => "Client",
            MessageSide::Client => "Server",
        }
    }
}

fn print_member_docc_description_and_comment(
    s: &mut Writer,
    tags: &MemberTags,
    o: &Objects,
    object_tags: &ObjectTags,
) {
    if let Some(comment) = tags.comment() {
        for line in comment.as_rust_doc_lines(o, object_tags) {
            s.docc(line);
        }
    }
}

pub(crate) fn print_serde_derive(s: &mut Writer, should_derive: bool, transparent: bool) {
    if should_derive {
        s.wln("#[cfg_attr(feature = \"serde\", derive(serde::Deserialize, serde::Serialize))]");

        if transparent {
            s.wln("#[cfg_attr(feature = \"serde\", serde(transparent))]");
        }
    }
}

fn print_docc_description_and_comment(
    s: &mut Writer,
    tags: &ObjectTags,
    o: &Objects,
    object_tags: &ObjectTags,
) {
    if let Some(comment) = tags.comment() {
        for line in comment.as_rust_doc_lines(o, object_tags) {
            s.docc(line);
        }

        if tags.non_network_type() {
            s.docc_newline();
        }
    }

    if tags.non_network_type() {
        s.docc("This type is not sent over the network, but is used in the game in another way.");
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
