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
use walkdir::WalkDir;

use parser::types::objects::Objects;
use rust_printer::print_struct;

use crate::doc_printer::print_docs;
use crate::ember_printer::write_ember_schema;
use crate::file_utils::create_and_overwrite_if_not_same_contents;
use crate::file_utils::mod_files::ModFiles;
use crate::ir_printer::write_intermediate_representation;
use crate::parser::stats::print_message_stats;
use crate::parser::types::objects::Object;
use crate::parser::types::sizes::PACKED_GUID_MAX_SIZE;
use crate::parser::types::version::{AllRustVersions, MajorWorldVersion};
use crate::parser::types::IntegerType;
use crate::path_utils::{get_login_version_file_path, wowm_directory};
use crate::rust_printer::base_structs::{base_struct_read_name, base_struct_write_name};
use crate::rust_printer::writer::Writer;
use crate::rust_printer::{
    print_enum, print_enum_for_base, print_expected, print_flag, print_login_opcodes,
    print_opcode_to_name, print_read_write_base_structs, print_update_mask, print_world_opcodes,
    DefinerType,
};
use parser::types::container::{Container, ContainerType};
use parser::types::parsed::parsed_object::ParsedObjects;
use parser::types::tags::ObjectTags;
use path_utils::get_world_version_file_path;

mod base_printer;
mod doc_printer;
pub(crate) mod file_info;
mod file_utils;
mod ir_printer;
pub mod parser;
mod rust_printer;
mod wireshark_printer;
mod wowm_printer;

mod path_utils;

mod ember_printer;
pub mod error_printer;
#[cfg(test)]
mod test;

const UTILITY_PATH: &str = "crate::util";

const VERSIONS: &str = "versions";
const PASTE_VERSIONS: &str = "paste_versions";
const COMPRESSED: &str = "compressed";
const COMMENT: &str = "comment";
const DISPLAY: &str = "display";
const TEST_STR: &str = "test";
const SKIP_STR: &str = "skip_codegen";
const LOGIN_VERSIONS: &str = "login_versions";
const RUST_BASE_TYPE: &str = "rust_base_type";
const ZERO_IS_ALWAYS_VALID: &str = "zero_is_always_valid";
const FROM_DBC_FILE: &str = "from_dbc_file";
const NON_NETWORK_TYPE: &str = "non_network_type";
const USED_IN_UPDATE_MASK: &str = "used_in_update_mask";
const VALID_RANGE: &str = "valid_range";
const MAXIMUM_LENGTH: &str = "maximum_length";
const UNIMPLEMENTED: &str = "unimplemented";

const MAX_ALLOCATION_SIZE: i128 = 0xFF_FF;
const MAX_ALLOCATION_SIZE_WRATH: i128 = 0x7F_FF_FF;

// Also used in /utils.rs
const CSTRING_SMALLEST_ALLOWED: u8 = 1;
const CSTRING_LARGEST_ALLOWED: u16 = 256; // 256 is a guess

const SIZED_CSTRING_SMALLEST_ALLOWED: u8 = 4 + 1;
const SIZED_CSTRING_LARGEST_ALLOWED: u16 = 4 + 8000; // 8000 is a guess

const STRING_SMALLEST_POSSIBLE: u8 = 1;
const STRING_LARGEST_POSSIBLE: u16 = 257;

const MONSTER_MOVE_SPLINE_SMALLEST_ALLOWED: u8 = 4;
const MONSTER_MOVE_SPLINE_LARGEST_ALLOWED: i128 = 4 + 3 + u32::MAX as i128;

const ENCHANT_MASK_SMALLEST_ALLOWED: u8 = 2;
const ENCHANT_MASK_LARGEST_ALLOWED: u8 = 2 + 16 * 2;

const INSPECT_TALENT_GEAR_MASK_SMALLEST_ALLOWED: u8 = 4;
const INSPECT_TALENT_GEAR_MASK_LARGEST_ALLOWED: i128 =
    4 + 32 * (ENCHANT_MASK_LARGEST_ALLOWED as i128 + 4 + 2 + PACKED_GUID_MAX_SIZE as i128 + 4);

// Also used in auth.pest
const CONTAINER_SELF_SIZE_FIELD: &str = "self.size";

const GITHUB_REPO_URL: &str = "https://github.com/gtker/wow_messages";

fn main() {
    let base = std::thread::spawn(base_printer::print_base);

    load_and_print_wowm_files();

    base.join().unwrap();
}

fn load_and_print_wowm_files() {
    let o = parse_objects_in_directory(&wowm_directory());

    wireshark_printer::print_wireshark(&o);

    let n = print_custom_types(&o);

    print_main_types(&o, n);

    write_login_opcodes(&o);

    write_world_opcodes(&o);

    write_intermediate_representation(&o);

    write_ember_schema(&o);

    print_update_mask();

    print_expected();

    print_read_write_base_structs(&o);

    print_opcode_to_name();

    print_message_stats(&o);
}

enum AuraMaskMember {
    Integer(IntegerType),
    Struct(Container),
}

struct MaskType {
    name: String,
    capacity: u32,
    member: AuraMaskMember,
    access_function_name: String,
}

fn print_custom_types(o: &Objects) -> ModFiles {
    let mut n = ModFiles::new();

    let vanilla_types = [MaskType {
        name: "AuraMask".to_string(),
        capacity: 32,
        member: AuraMaskMember::Integer(IntegerType::U16),
        access_function_name: "auras".to_string(),
    }];
    let tbc_types = [MaskType {
        name: "AuraMask".to_string(),
        capacity: 64,
        member: AuraMaskMember::Struct(
            o.get_world_struct("Aura", MajorWorldVersion::BurningCrusade)
                .clone(),
        ),
        access_function_name: "auras".to_string(),
    }];
    let wrath_types = [
        MaskType {
            name: "AuraMask".to_string(),
            capacity: 64,
            member: AuraMaskMember::Struct(
                o.get_world_struct("Aura", MajorWorldVersion::Wrath).clone(),
            ),
            access_function_name: "auras".to_string(),
        },
        MaskType {
            name: "CacheMask".to_string(),
            capacity: 32,
            member: AuraMaskMember::Integer(IntegerType::U32),
            access_function_name: "data".to_string(),
        },
        MaskType {
            name: "EnchantMask".to_string(),
            capacity: 16,
            member: AuraMaskMember::Integer(IntegerType::U16),
            access_function_name: "enchants".to_string(),
        },
        MaskType {
            name: "InspectTalentGearMask".to_string(),
            capacity: 32,
            member: AuraMaskMember::Struct(
                o.get_world_struct("InspectTalentGear", MajorWorldVersion::Wrath)
                    .clone(),
            ),
            access_function_name: "enchants".to_string(),
        },
    ];

    let all_types = [
        (vanilla_types.as_ref(), MajorWorldVersion::Vanilla),
        (tbc_types.as_ref(), MajorWorldVersion::BurningCrusade),
        (wrath_types.as_ref(), MajorWorldVersion::Wrath),
    ];

    for (types, version) in all_types {
        for t in types {
            let mut s = Writer::new();

            print_custom_type(&mut s, &t, version);
            let versions = &[version];
            n.add_world_module(&t.name, versions, s.inner());
        }
    }

    n
}

fn print_custom_type(s: &mut Writer, t: &MaskType, version: MajorWorldVersion) {
    let name = &t.name;
    let ty_name = match &t.member {
        AuraMaskMember::Integer(i) => i.rust_str().to_string(),
        AuraMaskMember::Struct(c) => {
            format!("crate::{}::{}", version.module_name(), c.name())
        }
    };
    let can_derive_default = t.capacity <= 32;
    let default_text = if !can_derive_default { "" } else { "Default, " };

    s.wln(format!(
        "#[derive(Debug, Hash, {default_text}Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]",
    ));
    s.body(format!("pub struct {name}"), |s| {
        s.wln(format!("inners: [Option<{ty_name}>; Self::MAX_CAPACITY],"));
    });
    s.newline();

    s.body(format!("impl {name}"), |s| {
        s.wln(format!("const MAX_CAPACITY: usize = {};", t.capacity));
        s.newline();

        let bit_pattern_type = format!("u{}", t.capacity);
        s.body(
            "pub(crate) fn read(mut r: impl std::io::Read) -> Result<Self, std::io::Error>",
            |s| {
                s.wln("let mut inners = [None; Self::MAX_CAPACITY];");
                s.wln(format!(
                    "let bit_pattern: {bit_pattern_type} = crate::util::read_{bit_pattern_type}_le(&mut r)?;"
                ));
                s.newline();

                s.body("for (i, inner) in inners.iter_mut().enumerate()", |s| {
                    s.body("if (bit_pattern & (1 << i)) != 0", |s| match &t.member {
                        AuraMaskMember::Integer(i) => {
                            s.wln(format!(
                                "*inner = Some(crate::util::read_{}_le(&mut r)?);",
                                i.rust_str()
                            ));
                        }
                        AuraMaskMember::Struct(c) => {
                            if c.tags().is_in_base() {
                                s.wln(format!("*inner = Some(crate::util::{}(&mut r)?);", base_struct_read_name(c)))
                            } else {
                                s.wln(format!("*inner = Some({ty_name}::read(&mut r)?);"))
                            }
                        }
                    });
                });
                s.newline();

                s.wln("Ok(Self { inners })")
            },
        );
        s.newline();


        let access_function_name = &t.access_function_name;
        s.body("pub(crate) fn write_into_vec(&self, mut v: impl std::io::Write) -> Result<(), std::io::Error>", |s| {
            s.wln(format!("let mut bit_pattern: {bit_pattern_type} = 0;"));
            s.body(format!("for (i, &b) in self.{access_function_name}().iter().enumerate()"), |s| {
                s.body("if b.is_some()", |s| {
                    s.wln("bit_pattern |= 1 << i;")
                });
            });
            s.newline();

            s.wln("std::io::Write::write_all(&mut v, bit_pattern.to_le_bytes().as_slice())?;");
            s.newline();

            s.body(format!("for &i in self.{access_function_name}()"), |s| {
                s.body("if let Some(b) = i", |s| {
                    match &t.member {
                        AuraMaskMember::Integer(_) => {
                            s.wln("std::io::Write::write_all(&mut v, b.to_le_bytes().as_slice())?;")
                        }
                        AuraMaskMember::Struct(c) => {
                            if c.tags().is_in_base() {
                                s.wln(format!("crate::util::{}(&b, &mut v)?;", base_struct_write_name(c)));
                            } else {
                                s.wln("b.write_into_vec(&mut v)?;")

                            }
                        }
                    }
                });
            });
            s.newline();

            s.wln("Ok(())");
        });
        s.newline();

        s.body(format!("pub const fn {access_function_name}(&self) -> &[Option<{ty_name}>]"), |s| {
            s.wln("self.inners.as_slice()")
        });
        s.newline();

        s.body(format!("pub const fn {access_function_name}_mut(&mut self) -> &mut [Option<{ty_name}>]"), |s| {
            s.wln("self.inners.as_mut_slice()")
        });
        s.newline();

        s.body("pub(crate) const fn size(&self) -> usize", |s| {
            s.wln(format!("const MASK_VARIABLE_SIZE: usize = core::mem::size_of::<{bit_pattern_type}>();"));
            let ty_size = match &t.member {
                AuraMaskMember::Integer(i) => i.size().to_string(),
                AuraMaskMember::Struct(c) => if c.is_constant_sized() {
                    c.sizes().maximum().to_string()
                } else {
                    "i.size()".to_string()
                },
            };
            s.wln("let mut auras = 0;");
            s.wln("let mut index = 0;");
            s.body("while index < self.inners.len()", |s| {
                s.body("if let Some(i) = self.inners[index]", |s| {
                    s.wln(format!("auras += {ty_size};"));
                });
                s.wln("index += 1;");
            });
            s.newline();

            s.wln("MASK_VARIABLE_SIZE + auras");
        });
    });

    if !can_derive_default {
        s.newline();
        s.body(format!("impl Default for {}", t.name), |s| {
            s.body("fn default() -> Self", |s| {
                s.body(t.name.as_str(), |s| {
                    s.wln("inners: [None; Self::MAX_CAPACITY],");
                })
            });
        });
    }
}

fn print_main_types(o: &Objects, mut n: ModFiles) {
    for e in o.all_objects() {
        if should_not_write_object(e.tags()) {
            continue;
        }

        match e.tags().all_rust_versions() {
            AllRustVersions::Login(l) => {
                let s = match &e {
                    Object::Container(e) => print_struct(e, o),
                    Object::Enum(e) => print_enum(e, o),
                    Object::Flag(e) => print_flag(e, o),
                };

                n.add_login_module(e.name(), l.iter().cloned(), s.inner())
            }
            AllRustVersions::World(l) => {
                let versions = l.iter().cloned().collect::<Vec<_>>();

                let s = match &e {
                    Object::Container(e) => print_struct(e, o),
                    Object::Enum(e) => print_enum_for_base(e, o),
                    Object::Flag(e) => print_flag(e, o),
                };

                if e.tags().is_in_base() {
                    n.add_base_module(e.name(), &versions, s.inner());
                } else {
                    n.add_world_module(e.name(), &versions, s.inner());
                }
            }
        }
    }

    print_docs(o);

    n.write_modules_and_remove_unwritten_files();
}

fn write_world_opcodes(o: &Objects) {
    for e in o.get_rust_world_versions_with_objects() {
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
            let s = print_world_opcodes(&cmsg, o, &e, ContainerType::CMsg(0));
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
            let s = print_world_opcodes(&smsg, o, &e, ContainerType::SMsg(0));
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
        let clogin: Vec<&Container> = v
            .clone()
            .into_iter()
            .filter(|a| matches!(a.container_type(), ContainerType::CLogin(_)))
            .collect();
        if !clogin.is_empty() {
            let s = print_login_opcodes(&clogin, &e, ContainerType::CLogin(0));
            contents.write_str(s.inner()).unwrap();
        }

        let slogin: Vec<&Container> = v
            .into_iter()
            .filter(|a| matches!(a.container_type(), ContainerType::SLogin(_)))
            .collect();
        if !slogin.is_empty() {
            let s = print_login_opcodes(&slogin, &e, ContainerType::SLogin(0));
            contents.write_str(s.inner()).unwrap();
        }

        let filename = get_login_version_file_path(&e).join("opcodes.rs");
        create_and_overwrite_if_not_same_contents(&contents, &filename);
    }
}

pub(crate) fn parse_objects_in_directory(dir: &Path) -> Objects {
    let mut components = ParsedObjects::empty();

    for file in WalkDir::new(dir).into_iter().filter_map(|a| a.ok()) {
        if !file.file_type().is_file() {
            continue;
        }
        let c = parser::parse_file(file.path());
        components.add_vecs(c);
    }

    components.into_objects()
}

fn should_not_write_object(t: &ObjectTags) -> bool {
    t.test() || t.skip() || !t.has_rust_version()
}

fn should_not_write_object_docs(t: &ObjectTags) -> bool {
    t.test() || t.skip()
}

pub(crate) fn float_format(v: f32) -> String {
    let s = format!("{v}");
    if s.contains('.') {
        s
    } else {
        format!("{s}.0")
    }
}
