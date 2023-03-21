#![forbid(unsafe_code)]
#![warn(
    clippy::approx_constant,
    clippy::bool_to_int_with_if,
    clippy::complexity,
    clippy::correctness,
    clippy::derive_partial_eq_without_eq,
    clippy::doc_markdown,
    clippy::format_in_format_args,
    clippy::uninlined_format_args,
    clippy::enum_variant_names,
    clippy::large_enum_variant,
    clippy::needless_borrow,
    clippy::perf,
    clippy::single_match,
    clippy::style,
    clippy::unseparated_literal_suffix,
    clippy::upper_case_acronyms,
    dead_code,
    non_camel_case_types,
    unused
)]
#![allow(clippy::too_many_arguments)]

use std::fmt::Write;
use std::path::Path;

use doc_printer::container::print_docs_for_container;
use doc_printer::definer::{print_docs_for_enum, print_docs_for_flag};
use walkdir::WalkDir;

use parser::types::objects::Objects;
use rust_printer::print_struct;

use crate::doc_printer::print_docs_summary_and_objects;
use crate::file_utils::create_and_overwrite_if_not_same_contents;
use crate::file_utils::mod_files::ModFiles;
use crate::ir_printer::write_intermediate_representation;
use crate::parser::stats::print_message_stats;
use crate::parser::types::objects::Object;
use crate::parser::types::sizes::PACKED_GUID_MAX_SIZE;
use crate::path_utils::{get_login_version_file_path, wowm_directory};
use crate::rust_printer::{
    print_enum, print_enum_for_base, print_flag, print_login_opcodes, print_opcode_to_name,
    print_update_mask, print_world_opcodes, DefinerType,
};
use parser::types::container::{Container, ContainerType};
use parser::types::parsed::parsed_object::ParsedObjects;
use parser::types::tags::ObjectTags;
use path_utils::get_world_version_file_path;

mod base_printer;
mod doc_printer;
pub(crate) mod file_info;
mod file_utils;
pub mod impl_features;
mod ir_printer;
pub mod parser;
mod rust_printer;
mod wireshark_printer;
mod wowm_printer;

mod path_utils;

pub mod error_printer;
#[cfg(test)]
mod test;

const UTILITY_PATH: &str = "crate::util";

const VERSIONS: &str = "versions";
const PASTE_VERSIONS: &str = "paste_versions";
const DESCRIPTION: &str = "description";
const COMPRESSED: &str = "compressed";
const COMMENT: &str = "comment";
const DISPLAY: &str = "display";
const TEST_STR: &str = "test";
const SKIP_STR: &str = "skip_codegen";
const LOGIN_VERSIONS: &str = "login_versions";
const RUST_BASE_TYPE: &str = "rust_base_type";
const ZERO_IS_ALWAYS_VALID: &str = "zero_is_always_valid";
const UNIMPLEMENTED: &str = "unimplemented";

// Also used in /utils.rs
const CSTRING_SMALLEST_ALLOWED: usize = 1;
const CSTRING_LARGEST_ALLOWED: usize = 256; // 256 is a guess

const SIZED_CSTRING_SMALLEST_ALLOWED: usize = 4 + 1;
const SIZED_CSTRING_LARGEST_ALLOWED: usize = 4 + 8000; // 8000 is a guess

const STRING_SMALLEST_POSSIBLE: usize = 1;
const STRING_LARGEST_POSSIBLE: usize = 257;

const MONSTER_MOVE_SPLINE_SMALLEST_ALLOWED: usize = 4;
const MONSTER_MOVE_SPLINE_LARGEST_ALLOWED: usize = 4 + 3 + u32::MAX as usize;

const ENCHANT_MASK_SMALLEST_ALLOWED: usize = 2;
const ENCHANT_MASK_LARGEST_ALLOWED: usize = 2 + 16 * 2;

const INSPECT_TALENT_GEAR_MASK_SMALLEST_ALLOWED: usize = 4;
const INSPECT_TALENT_GEAR_MASK_LARGEST_ALLOWED: usize =
    4 + 32 * (ENCHANT_MASK_LARGEST_ALLOWED + 4 + 2 + PACKED_GUID_MAX_SIZE as usize + 4);

// Also used in auth.pest
const ENUM_SELF_VALUE_FIELD: &str = "self.value";
const CONTAINER_SELF_SIZE_FIELD: &str = "self.size";

const GITHUB_REPO_URL: &str = "https://github.com/gtker/wow_messages";

fn main() {
    let base = std::thread::spawn(base_printer::print_base);

    let mut o = ParsedObjects::empty();

    load_files(&wowm_directory("login"), &mut o);
    load_files(&wowm_directory("world"), &mut o);
    load_files(&wowm_directory("unimplemented"), &mut o);

    let o = o.into_objects();

    wireshark_printer::print_wireshark(&o);

    print_main_types(&o);

    write_login_opcodes(&o);

    write_world_opcodes(&o);

    write_intermediate_representation(&o);

    print_update_mask();

    print_opcode_to_name();

    print_message_stats(&o);

    base.join().unwrap();
}

fn print_main_types(o: &Objects) {
    let mut n = ModFiles::new();

    let mut definer_docs = Vec::new();
    let mut object_docs = Vec::new();

    for e in o.all_objects() {
        if should_not_write_object(e.tags()) {
            continue;
        }

        let s = match &e {
            Object::Container(e) => print_struct(e, o),
            Object::Enum(e) => print_enum(e, o),
            Object::Flag(e) => print_flag(e, o),
        };

        if e.tags().has_login_version() {
            let versions = e.tags().main_versions().map(|a| a.as_login());
            n.add_login_module(e.name(), versions, s.inner())
        } else {
            let versions = e
                .tags()
                .main_versions()
                .map(|a| a.as_major_world())
                .collect::<Vec<_>>();

            let s = match &e {
                Object::Enum(e) => print_enum_for_base(e, o),
                Object::Container(e) => print_struct(e, o),
                Object::Flag(e) => print_flag(e, o),
            };

            if e.tags().is_in_base() {
                n.add_base_module(e.name(), &versions, s.inner());
            } else {
                n.add_world_module(e.name(), &versions, s.inner());
            }
        }

        match &e {
            Object::Container(e) => object_docs.push(print_docs_for_container(e, o)),
            Object::Enum(e) => definer_docs.push(print_docs_for_enum(e)),
            Object::Flag(e) => definer_docs.push(print_docs_for_flag(e)),
        }
    }

    definer_docs.sort_by(|a, b| a.name().cmp(b.name()));
    object_docs.sort_by(|a, b| a.name().cmp(b.name()));

    print_docs_summary_and_objects(&definer_docs, &object_docs);

    n.write_modules_and_remove_unwritten_files();
}

fn write_world_opcodes(o: &Objects) {
    for e in o.get_main_world_versions_with_objects() {
        let mut contents = String::with_capacity(16000);

        let mut v = o.get_world_messages_with_versions_and_all(&e);
        v.sort_by_key(|a| a.container_type());
        let cmsg: Vec<&Container> = v
            .clone()
            .into_iter()
            .filter(|a| {
                matches!(
                    a.container_type(),
                    ContainerType::Msg(_) | ContainerType::CMsg(_)
                )
            })
            .collect();
        if !cmsg.is_empty() {
            let s = print_world_opcodes(&cmsg, &e, ContainerType::CMsg(0));
            contents.write_str(s.inner()).unwrap();
        }

        let smsg: Vec<&Container> = v
            .into_iter()
            .filter(|a| {
                matches!(
                    a.container_type(),
                    ContainerType::SMsg(_) | ContainerType::Msg(_)
                )
            })
            .collect();
        if !smsg.is_empty() {
            let s = print_world_opcodes(&smsg, &e, ContainerType::SMsg(0));
            contents.write_str(s.inner()).unwrap();
        }

        let filename = get_world_version_file_path(&e).join("opcodes.rs");
        create_and_overwrite_if_not_same_contents(&contents, &filename);
    }
}

fn write_login_opcodes(o: &Objects) {
    for e in o.get_login_versions_with_objects() {
        let mut contents = String::with_capacity(16000);

        let mut v: Vec<&Container> = o.get_login_messages_with_versions_and_all(&e);
        v.sort_by_key(|a| a.container_type());
        let slogin: Vec<&Container> = v
            .clone()
            .into_iter()
            .filter(|a| matches!(a.container_type(), ContainerType::SLogin(_)))
            .collect();
        if !slogin.is_empty() {
            let s = print_login_opcodes(&slogin, &e, ContainerType::SLogin(0));
            contents.write_str(s.inner()).unwrap();
        }

        let clogin: Vec<&Container> = v
            .into_iter()
            .filter(|a| matches!(a.container_type(), ContainerType::CLogin(_)))
            .collect();
        if !clogin.is_empty() {
            let s = print_login_opcodes(&clogin, &e, ContainerType::CLogin(0));
            contents.write_str(s.inner()).unwrap();
        }

        let filename = get_login_version_file_path(&e).join("opcodes.rs");
        create_and_overwrite_if_not_same_contents(&contents, &filename);
    }
}

fn load_files(dir: &Path, components: &mut ParsedObjects) {
    for file in WalkDir::new(dir).into_iter().filter_map(|a| a.ok()) {
        if !file.file_type().is_file() {
            continue;
        }
        let c = parser::parse_file(file.path());
        components.add_vecs(c);
    }
}

fn should_not_write_object(t: &ObjectTags) -> bool {
    t.test() || t.skip() || !t.is_main_version()
}
