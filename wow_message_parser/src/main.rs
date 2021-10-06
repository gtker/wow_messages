use std::path::Path;

use walkdir::WalkDir;

use rust_printer::print_struct;

use crate::container::{Container, ContainerType};
use crate::file_utils::{
    append_string_to_file, get_world_version_file_path, write_string_to_file, ModFiles, LOGIN_DIR,
};
use crate::parser::types::Objects;
use crate::rust_printer::{print_enum, print_flag, print_login_opcodes, print_world_opcodes};

mod container;
pub(crate) mod file_info;
mod file_utils;
pub mod parser;
mod rust_printer;
mod test_case;

const UTILITY_PATH: &str = "crate::util";

const VERSIONS: &str = "versions";
const TEST_STR: &str = "test";
const DISPLAY_STR: &str = "display";
const SKIP_STR: &str = "skip_codegen";
const LOGIN_LOGON_VERSIONS: &str = "login_logon_versions";

// Also used in /utils.rs
const CSTRING_LARGEST_ALLOWED: usize = 256;

// Also used in auth.pest
const ENUM_SELF_VALUE_FIELD: &str = "self.value";

const LOGIN_MESSAGES_GITHUB_REPO: &str = "https://github.com/gtker/wow_messages";

const WORLD_MESSAGES_GITHUB_REPO: &str = "https://github.com/gtker/wow_messages";

fn main() {
    let mut o = Objects::empty();

    load_files(Path::new("wow_message_parser/wowm/login"), &mut o);
    load_files(Path::new("wow_message_parser/wowm/test"), &mut o);
    load_files(Path::new("wow_message_parser/wowm/world"), &mut o);

    let mut m = ModFiles::new();

    for e in o.enums() {
        if e.has_tag(TEST_STR) || e.has_tag(SKIP_STR) {
            continue;
        }
        let s = print_enum(e);
        m.write_contents_to_file(e.name(), e.tags(), &s);
    }

    for e in o.flags() {
        if e.has_tag(TEST_STR) || e.has_tag(SKIP_STR) {
            continue;
        }
        let s = print_flag(e);
        m.write_contents_to_file(e.name(), e.tags(), &s);
    }

    for e in o.structs() {
        if e.has_tag(TEST_STR) || e.has_tag(SKIP_STR) {
            continue;
        }
        let s = print_struct(e, &o);
        m.write_contents_to_file(e.name(), e.tags(), &s);
    }

    for e in o.messages() {
        if e.has_tag(TEST_STR) || e.has_tag(SKIP_STR) {
            continue;
        }
        let s = print_struct(e, &o);
        m.write_contents_to_file(e.name(), e.tags(), &s);
    }

    m.write_mod_files();

    write_login_opcodes(&o);

    write_world_opcodes(&o);

    o.print_stats_for_1_12();
}

fn write_world_opcodes(o: &Objects) {
    for e in o.get_world_versions_with_objects() {
        let mut v = o.get_world_messages_with_versions_and_all(&e);
        v.sort_by_key(|a| a.container_type());
        let cmsg: Vec<&Container> = v
            .clone()
            .into_iter()
            .filter(|a| match a.container_type() {
                ContainerType::CMsg(_) => true,
                ContainerType::Msg(_) => true,
                _ => false,
            })
            .collect();
        if !cmsg.is_empty() {
            let s = print_world_opcodes(&cmsg, &e, ContainerType::CMsg(0));
            let filename = format!("{}/opcodes.rs", get_world_version_file_path(&e));
            write_string_to_file(s.proper_as_str(), Path::new(&filename));
        }

        let smsg: Vec<&Container> = v
            .into_iter()
            .filter(|a| match a.container_type() {
                ContainerType::SMsg(_) => true,
                ContainerType::Msg(_) => true,
                _ => false,
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
            .filter(|a| match a.container_type() {
                ContainerType::SLogin(_) => true,
                _ => false,
            })
            .collect();
        if !slogin.is_empty() {
            let s = print_login_opcodes(&slogin, &e, ContainerType::SLogin(0));
            let filename = format!(
                "{login_dir}/logon/version_{version}/opcodes.rs",
                login_dir = LOGIN_DIR,
                version = e
            );
            write_string_to_file(s.proper_as_str(), Path::new(&filename));
        }

        let clogin: Vec<&Container> = v
            .into_iter()
            .filter(|a| match a.container_type() {
                ContainerType::CLogin(_) => true,
                _ => false,
            })
            .collect();
        if !clogin.is_empty() {
            let s = print_login_opcodes(&clogin, &e, ContainerType::CLogin(0));
            let filename = format!(
                "{login_dir}/logon/version_{version}/opcodes.rs",
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

#[cfg(test)]
mod test {
    use crate::file_utils::write_string_to_file;
    use crate::load_files;
    use crate::parser::types::Objects;
    use crate::rust_printer::{print_enum, print_flag, print_struct, Writer};
    use std::fs::read_to_string;
    use std::path::Path;

    const OVERWRITE_ALL_TESTS: bool = false;

    fn get_all_impl_items() -> Objects {
        let mut o = Objects::empty();

        load_files(Path::new("tests/impl_levels.wowm"), &mut o);
        o.check_values();

        o
    }

    fn tcheck(s: &Writer, name: &str) {
        if OVERWRITE_ALL_TESTS {
            overwrite(s, name);
        } else {
            check(s, name);
        }
    }

    fn check(s: &Writer, name: &str) {
        let expected = read_to_string(Path::new(&format!("tests/{}.txt", name))).unwrap();

        assert_eq!(s.proper_as_str(), expected);
    }

    fn overwrite(s: &Writer, name: &str) {
        write_string_to_file(s.proper_as_str(), Path::new(&format!("tests/{}.txt", name)));
    }

    #[test]
    fn simple_enum() {
        let o = get_all_impl_items();

        let d = o.enums().iter().find(|a| a.name() == "SimpleEnum").unwrap();
        let s = print_enum(d);

        tcheck(&s, "simple_enum");
    }

    #[test]
    fn simple_flag() {
        let o = get_all_impl_items();

        let d = o.flags().iter().find(|a| a.name() == "SimpleFlag").unwrap();
        let s = print_flag(d);

        tcheck(&s, "simple_flag");
    }

    #[test]
    fn struct_with_all_built_in_types() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "StructWithAllBuiltInTypes")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "struct_with_all_built_in_types");
    }

    #[test]
    fn simple_clogin() {
        let o = get_all_impl_items();

        let d = o
            .messages()
            .iter()
            .find(|a| a.name() == "SimpleClogin")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "simple_clogin");
    }

    #[test]
    fn simple_slogin() {
        let o = get_all_impl_items();

        let d = o
            .messages()
            .iter()
            .find(|a| a.name() == "SimpleSlogin")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "simple_slogin");
    }

    #[test]
    fn simple_smsg() {
        let o = get_all_impl_items();

        let d = o
            .messages()
            .iter()
            .find(|a| a.name() == "SimpleSmsg")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "simple_smsg");
    }

    #[test]
    fn simple_cmsg() {
        let o = get_all_impl_items();

        let d = o
            .messages()
            .iter()
            .find(|a| a.name() == "SimpleCmsg")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "simple_cmsg");
    }

    #[test]
    fn simple_msg() {
        let o = get_all_impl_items();

        let d = o
            .messages()
            .iter()
            .find(|a| a.name() == "SimpleMsg")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "simple_msg");
    }

    #[test]
    fn message_with_complex_types() {
        let o = get_all_impl_items();

        let d = o
            .messages()
            .iter()
            .find(|a| a.name() == "MessageWithComplexTypes")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "message_with_complex_types");
    }

    #[test]
    fn arrays() {
        let o = get_all_impl_items();

        let d = o.structs().iter().find(|a| a.name() == "Arrays").unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "arrays");
    }

    #[test]
    fn simple_if_enum() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "SimpleIfEnum")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "simple_if_enum");
    }

    #[test]
    fn double_variant_enum() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "DoubleVariantEnum")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "double_variant_enum");
    }

    #[test]
    fn simple_if_flag() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "SimpleIfFlag")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "simple_if_flag");
    }

    #[test]
    fn simple_if_enum_else() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "SimpleIfEnumElse")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "simple_if_enum_else");
    }

    #[test]
    fn simple_if_enum_if_else() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "SimpleIfEnumIfElse")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "simple_if_enum_if_else");
    }

    #[test]
    fn packed_guid_and_guid() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "PackedGuidAndGuid")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "packed_guid_and_guid");
    }

    #[test]
    fn custom_masks() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "CustomMasks")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "custom_masks");
    }

    #[test]
    fn if_edge_cases() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "IfEdgeCases")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "if_edge_cases");
    }

    #[test]
    fn optional() {
        let o = get_all_impl_items();

        let d = o.structs().iter().find(|a| a.name() == "Optional").unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "optional");
    }

    #[test]
    fn simple_not_if() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "SimpleIfEnumNot")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "simple_not_if");
    }

    #[test]
    fn not_if_else() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "SimpleIfNotEnumElse")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "not_if_else");
    }

    #[test]
    fn nested_not_if() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "NestedNotIf")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "nested_not_if");
    }

    #[test]
    fn enum_outside_of_if() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "EnumOutsideOfIf")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "enum_outside_of_if");
    }

    #[test]
    fn cmsg_test_endless_u8() {
        let o = get_all_impl_items();

        let d = o
            .messages()
            .iter()
            .find(|a| a.name() == "CMSG_TEST_ENDLESS_U8")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "cmsg_test_endless_u8");
    }

    #[test]
    fn cmsg_test_optional() {
        let o = get_all_impl_items();

        let d = o
            .messages()
            .iter()
            .find(|a| a.name() == "CMSG_TEST_OPTIONAL")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "cmsg_test_optional");
    }

    #[test]
    fn flag_if_else_if() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "FlagIfElseIf")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "flag_if_else_if");
    }

    #[test]
    fn enum_if_else_if_nested() {
        let o = get_all_impl_items();

        let d = o
            .structs()
            .iter()
            .find(|a| a.name() == "EnumIfElseIfNested")
            .unwrap();
        let s = print_struct(d, &o);

        tcheck(&s, "enum_if_else_if_nested");
    }
}
