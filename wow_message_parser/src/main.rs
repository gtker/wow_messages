#![allow(clippy::too_many_arguments)]

use std::path::Path;

use doc_printer::container::print_docs_for_container;
use doc_printer::definer::{print_docs_for_enum, print_docs_for_flag};
use walkdir::WalkDir;

use parser::types::objects::Objects;
use rust_printer::print_struct;

use crate::container::{Container, ContainerType};
use crate::doc_printer::print_docs_summary_and_objects;
use crate::file_utils::{
    append_string_to_file, create_and_overwrite_if_not_same_contents, get_world_version_file_path,
    write_string_to_file, ModFiles, LOGIN_DIR,
};
use crate::ir_printer::write_intermediate_representation;
use crate::parser::types::objects::Object;
use crate::rust_printer::{
    get_import_from_base, get_import_from_shared, print_enum, print_enum_for_base, print_flag,
    print_login_opcodes, print_update_mask, print_world_opcodes, DefinerType, Version,
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
const LOGIN_VERSIONS: &str = "login_versions";
const RUST_BASE_TYPE: &str = "rust_base_type";

// Also used in /utils.rs
const CSTRING_SMALLEST_ALLOWED: usize = 1;
const CSTRING_LARGEST_ALLOWED: usize = 256; // 256 is a guess

const SIZED_CSTRING_SMALLEST_ALLOWED: usize = 4 + 1;
const SIZED_CSTRING_LARGEST_ALLOWED: usize = 4 + 8000; // 8000 is a guess

// Also used in auth.pest
const ENUM_SELF_VALUE_FIELD: &str = "self.value";
const CONTAINER_SELF_SIZE_FIELD: &str = "self.size";

const GITHUB_REPO_URL: &str = "https://github.com/gtker/wow_messages";

fn main() {
    let mut o = Objects::empty();

    load_files(Path::new("wow_message_parser/wowm/login"), &mut o);
    load_files(Path::new("wow_message_parser/wowm/world"), &mut o);
    load_files(Path::new("wow_message_parser/wowm/unimplemented"), &mut o);
    //load_files(Path::new("wow_message_parser/wowm/test"), &mut o);

    let mut m = ModFiles::new();

    let mut definer_docs = Vec::new();
    let mut object_docs = Vec::new();

    for e in o.all_objects() {
        if should_not_write_object(e.tags()) {
            continue;
        }

        let (first, mut versions) = e.tags().first_and_main_versions();

        if !e.tags().is_in_base() {
            let s = match &e {
                Object::Container(e) => print_struct(e, &o, first),
                Object::Enum(e) => print_enum(e, &o, first),
                Object::Flag(e) => print_flag(e, &o, first),
            };

            if versions.is_empty() {
                m.write_contents_to_file(e.name(), e.tags(), s.proper_as_str(), first);
            } else if !first.is_world() {
                m.write_contents_to_file(e.name(), e.tags(), s.proper_as_str(), first);

                for v in versions {
                    m.write_contents_to_file(e.name(), e.tags(), s.imports(), v);
                }
            } else {
                versions.push(first);

                m.write_shared_contents_to_file(e.name(), e.tags(), s.inner(), &versions);

                for v in versions.clone() {
                    let s = get_import_from_shared(e.name(), &versions);
                    m.write_contents_to_file(e.name(), e.tags(), &s, v);
                }
            }
        } else {
            let (base_s, world_s) = match &e {
                Object::Enum(e) => {
                    let base_s = print_enum_for_base(e, &o, first);
                    let world_s = get_import_from_base(e.name(), first);

                    (base_s, world_s)
                }
                _ => unimplemented!(),
            };

            if versions.is_empty() {
                m.write_base_contents_to_file(e.name(), e.tags(), base_s.inner(), &world_s, first);
            } else {
                versions.push(first);

                m.write_shared_contents_to_file(e.name(), e.tags(), base_s.inner(), &versions);

                for v in versions.clone() {
                    let (world_s, base_s) = match &e {
                        Object::Enum(e) => {
                            let base_s = get_import_from_shared(e.name(), &versions);
                            let world_s = get_import_from_base(e.name(), v);

                            (world_s, base_s)
                        }
                        _ => unimplemented!(),
                    };

                    m.write_shared_import_to_file(e.name(), e.tags(), &world_s, &base_s, &v);
                }
            }
        }

        if !e.tags().unimplemented() {
            match &e {
                Object::Container(e) => object_docs.push(print_docs_for_container(e, &o)),
                Object::Enum(e) => definer_docs.push(print_docs_for_enum(e)),
                Object::Flag(e) => definer_docs.push(print_docs_for_flag(e)),
            }
        }
    }

    definer_docs.sort_by(|a, b| a.name().cmp(b.name()));
    object_docs.sort_by(|a, b| a.name().cmp(b.name()));

    print_docs_summary_and_objects(&definer_docs, &object_docs);

    m.write_mod_files();
    m.remove_unwritten_files();

    write_login_opcodes(&o);

    write_world_opcodes(&o);

    write_intermediate_representation(&o);

    print_update_mask();

    o.print_stats_for_1_12();
}

fn write_world_opcodes(o: &Objects) {
    for e in o.get_main_world_versions_with_objects() {
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
            create_and_overwrite_if_not_same_contents(s.proper_as_str(), Path::new(&filename));
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
    t.contains(TEST_STR) || t.contains(SKIP_STR) || !t.is_main_version()
}
