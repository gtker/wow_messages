use std::path::Path;

use doc_printer::container::print_docs_for_container;
use doc_printer::definer::{print_docs_for_enum, print_docs_for_flag};
use walkdir::WalkDir;

use parser::types::objects::Objects;
use rust_printer::print_struct;

use crate::container::{Container, ContainerType};
use crate::doc_printer::print_docs_summary_and_objects;
use crate::file_utils::{
    append_string_to_file, get_world_version_file_path, write_string_to_file, ModFiles, LOGIN_DIR,
};
use crate::ir_printer::write_intermediate_representation;
use crate::rust_printer::{
    print_enum, print_flag, print_login_opcodes, print_update_mask, print_world_opcodes,
};
use parser::types::tags::Tags;

mod container;
mod doc_printer;
pub(crate) mod file_info;
mod file_utils;
pub mod impl_features;
mod ir_printer;
pub mod parser;
mod rust_printer;
mod test_case;
mod wowm_printer;

#[cfg(test)]
mod test;

const UTILITY_PATH: &str = "crate::util";

const VERSIONS: &str = "versions";
const DESCRIPTION: &str = "description";
const COMMENT: &str = "comment";
const DISPLAY: &str = "display";
const TEST_STR: &str = "test";
const DISPLAY_STR: &str = "display";
const SKIP_STR: &str = "skip_codegen";
const LOGIN_LOGON_VERSIONS: &str = "login_logon_versions";

// Also used in /utils.rs
const CSTRING_SMALLEST_ALLOWED: usize = 1;
const CSTRING_LARGEST_ALLOWED: usize = 256;

// Also used in auth.pest
const ENUM_SELF_VALUE_FIELD: &str = "self.value";
const CONTAINER_SELF_SIZE_FIELD: &str = "self.size";

const GITHUB_REPO_URL: &str = "https://github.com/gtker/wow_messages";

fn main() {
    let mut o = Objects::empty();

    load_files(Path::new("wow_message_parser/wowm/login"), &mut o);
    load_files(Path::new("wow_message_parser/wowm/world"), &mut o);
    //load_files(Path::new("wow_message_parser/wowm/test"), &mut o);

    let mut m = ModFiles::new();

    let mut definer_docs = Vec::new();
    for e in o.enums() {
        if should_not_write_object(e.tags()) {
            continue;
        }
        let s = print_enum(e);
        m.write_contents_to_file(e.name(), e.tags(), &s);

        definer_docs.push(print_docs_for_enum(e));
    }

    for e in o.flags() {
        if should_not_write_object(e.tags()) {
            continue;
        }
        let s = print_flag(e);
        m.write_contents_to_file(e.name(), e.tags(), &s);

        definer_docs.push(print_docs_for_flag(e));
    }

    let mut object_docs = Vec::new();
    for e in o.all_containers() {
        if should_not_write_object(e.tags()) {
            continue;
        }
        let s = print_struct(e, &o);
        m.write_contents_to_file(e.name(), e.tags(), &s);
        object_docs.push(print_docs_for_container(e, &o));
    }

    definer_docs.sort_by(|a, b| a.name().cmp(b.name()));
    object_docs.sort_by(|a, b| a.name().cmp(b.name()));

    print_docs_summary_and_objects(&definer_docs, &object_docs);

    m.write_mod_files();

    write_login_opcodes(&o);

    write_world_opcodes(&o);

    write_intermediate_representation(&o);

    print_update_mask();

    o.print_stats_for_1_12();
}

fn write_world_opcodes(o: &Objects) {
    for e in o.get_world_versions_with_objects() {
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
            let filename = format!("{}/opcodes.rs", get_world_version_file_path(&e));
            write_string_to_file(s.proper_as_str(), Path::new(&filename));
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
            let filename = format!("{}/opcodes.rs", get_world_version_file_path(&e));
            append_string_to_file(s.proper_as_str(), Path::new(&filename));
        }
    }
}

fn write_login_opcodes(o: &Objects) {
    for e in o.get_login_versions_with_objects() {
        let mut v: Vec<&Container> = o.get_login_messages_with_versions_and_all(&e);
        v.sort_by_key(|a| a.container_type());
        let slogin: Vec<&Container> = v
            .clone()
            .into_iter()
            .filter(|a| matches!(a.container_type(), ContainerType::SLogin(_)))
            .collect();
        if !slogin.is_empty() {
            let s = print_login_opcodes(&slogin, &e, ContainerType::SLogin(0));
            let filename = format!(
                "{login_dir}/version_{version}/opcodes.rs",
                login_dir = LOGIN_DIR,
                version = e
            );
            write_string_to_file(s.proper_as_str(), Path::new(&filename));
        }

        let clogin: Vec<&Container> = v
            .into_iter()
            .filter(|a| matches!(a.container_type(), ContainerType::CLogin(_)))
            .collect();
        if !clogin.is_empty() {
            let s = print_login_opcodes(&clogin, &e, ContainerType::CLogin(0));
            let filename = format!(
                "{login_dir}/version_{version}/opcodes.rs",
                login_dir = LOGIN_DIR,
                version = e
            );
            append_string_to_file(s.proper_as_str(), Path::new(&filename));
        }
    }
}

fn load_files(dir: &Path, components: &mut Objects) {
    for file in WalkDir::new(dir).into_iter().filter_map(|a| a.ok()) {
        if !file.file_type().is_file() {
            continue;
        }
        let c = parser::parse_file(file.path());
        components.add_vecs(c);
    }
    components.check_values();
}

fn should_not_write_object(t: &Tags) -> bool {
    t.contains(TEST_STR) || t.contains(SKIP_STR)
}
