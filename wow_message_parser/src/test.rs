use crate::error_printer::{
    BOTH_LOGIN_AND_WORLD_VERSIONS, COMPLEX_NOT_FOUND, DUPLICATE_DEFINER_VALUES,
    DUPLICATE_FIELD_NAMES, ENUM_HAS_BITWISE_AND, FLAG_HAS_EQUALS, INCORRECT_OPCODE_FOR_MESSAGE,
    INVALID_DEFINER_VALUE, INVALID_INTEGER_TYPE, INVALID_SELF_SIZE, MESSAGE_NOT_IN_INDEX,
    MISSING_ENUMERATOR, MULTIPLE_SELF_VALUE, NON_MATCHING_IF_VARIABLES, NO_VERSIONS,
    OPCODE_HAS_INCORRECT_NAME, OVERLAPPING_VERSIONS, RECURSIVE_TYPE, UNSUPPORTED_UPCAST,
};
use crate::file_utils::write_string_to_file;
use crate::parser::parse_file;
use crate::parser::types::objects::Objects;
use crate::parser::types::version::{MajorWorldVersion, Version};
use crate::path_utils::parser_test_directory;
use crate::rust_printer::{print_enum, print_flag, print_struct, Writer};
use crate::{load_files, print_message_stats, ParsedObjects};
use std::fs::read_to_string;
use std::panic;
use std::path::Path;

const OVERWRITE_ALL_TESTS: bool = false;

fn should_panic<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F, error_code: i32) {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));

    let result = panic::catch_unwind(f);
    panic::set_hook(prev_hook);

    match result {
        Ok(_) => panic!("test should have panicked and gotten caught but did not"),
        Err(e) => {
            assert_eq!(
                e.downcast::<String>().unwrap(),
                Box::new(error_code.to_string())
            );
        }
    };
}

fn must_err_load(file_name: &str) -> Objects {
    let p = parser_test_directory().join("must_err").join(file_name);
    parse_file(&p).into_objects()
}

fn get_all_impl_items() -> Objects {
    let mut o = ParsedObjects::empty();

    load_files(Path::new("tests/impl_levels.wowm"), &mut o);

    o.into_objects()
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

const VERSION: Version = Version::World(MajorWorldVersion::Vanilla);

#[test]
fn simple_enum() {
    let o = get_all_impl_items();

    let d = o.enums().iter().find(|a| a.name() == "SimpleEnum").unwrap();
    let s = print_enum(d, &o, VERSION);

    tcheck(&s, "simple_enum");
}

#[test]
fn simple_flag() {
    let o = get_all_impl_items();

    let d = o.flags().iter().find(|a| a.name() == "SimpleFlag").unwrap();
    let s = print_flag(d, &o, VERSION);

    tcheck(&s, "simple_flag");
}

#[test]
fn struct_with_all_built_in_types() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "StructWithAllBuiltInTypes")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "struct_with_all_built_in_types");
}

#[test]
fn simple_clogin() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleClogin")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "simple_clogin");
}

#[test]
fn simple_slogin() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleSlogin")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "simple_slogin");
}

#[test]
fn simple_smsg() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleSmsg")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "simple_smsg");
}

#[test]
fn simple_cmsg() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleCmsg")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "simple_cmsg");
}

#[test]
fn simple_msg() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleMsg")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "simple_msg");
}

#[test]
fn message_with_complex_types() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "MessageWithComplexTypes")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "message_with_complex_types");
}

#[test]
fn arrays() {
    let o = get_all_impl_items();

    let d = o.all_containers().find(|a| a.name() == "Arrays").unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "arrays");
}

#[test]
fn simple_if_enum() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleIfEnum")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "simple_if_enum");
}

#[test]
fn double_variant_enum() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "DoubleVariantEnum")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "double_variant_enum");
}

#[test]
fn simple_if_flag() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleIfFlag")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "simple_if_flag");
}

#[test]
fn simple_if_enum_else() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleIfEnumElse")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "simple_if_enum_else");
}

#[test]
fn simple_if_enum_if_else() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleIfEnumIfElse")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "simple_if_enum_if_else");
}

#[test]
fn packed_guid_and_guid() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "PackedGuidAndGuid")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "packed_guid_and_guid");
}

#[test]
fn custom_masks() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "CustomMasks")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "custom_masks");
}

#[test]
fn if_edge_cases() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "IfEdgeCases")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "if_edge_cases");
}

#[test]
fn optional() {
    let o = get_all_impl_items();

    let d = o.all_containers().find(|a| a.name() == "Optional").unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "optional");
}

#[test]
fn simple_not_if() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleIfEnumNot")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "simple_not_if");
}

#[test]
fn not_if_else() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SimpleIfNotEnumElse")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "not_if_else");
}

#[test]
fn nested_not_if() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "NestedNotIf")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "nested_not_if");
}

#[test]
fn enum_outside_of_if() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "EnumOutsideOfIf")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "enum_outside_of_if");
}

#[test]
fn cmsg_test_endless_u8() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "CMSG_TEST_ENDLESS_U8")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "cmsg_test_endless_u8");
}

#[test]
fn cmsg_test_optional() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "CMSG_TEST_OPTIONAL")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "cmsg_test_optional");
}

#[test]
fn flag_if_else_if() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "FlagIfElseIf")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "flag_if_else_if");
}

#[test]
fn enum_if_else_if_nested() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "EnumIfElseIfNested")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "enum_if_else_if_nested");
}

#[test]
fn sized_cstring() {
    let o = get_all_impl_items();

    let d = o
        .all_containers()
        .find(|a| a.name() == "SizedCString")
        .unwrap();
    let s = print_struct(d, &o, VERSION);

    tcheck(&s, "sized_cstring");
}

#[test]
fn missing_ty_errors() {
    should_panic(|| must_err_load("missing_type.wowm"), COMPLEX_NOT_FOUND);
}

#[test]
fn missing_enumerator_errors() {
    should_panic(
        || must_err_load("missing_enumerator.wowm"),
        MISSING_ENUMERATOR,
    );
}

#[test]
fn recursive_types_errors() {
    should_panic(|| must_err_load("recursive_types.wowm"), RECURSIVE_TYPE);
}

#[test]
fn missing_versions() {
    should_panic(
        || {
            must_err_load("no_version.wowm");
        },
        NO_VERSIONS,
    );
}

#[test]
fn flag_equals_must_err() {
    should_panic(
        || {
            must_err_load("error_flag.wowm");
        },
        FLAG_HAS_EQUALS,
    );
}

#[test]
fn enum_equals_must_err() {
    should_panic(
        || {
            must_err_load("error_enum.wowm");
        },
        ENUM_HAS_BITWISE_AND,
    );
}

#[test]
fn incorrect_opcode_errors() {
    should_panic(
        || {
            let o = must_err_load("incorrect_opcode.wowm");
            print_message_stats(&o);
        },
        INCORRECT_OPCODE_FOR_MESSAGE,
    );
}

#[test]
fn multi_self_value_errors() {
    should_panic(
        || {
            let o = must_err_load("multi_self_value.wowm");
            print_message_stats(&o);
        },
        MULTIPLE_SELF_VALUE,
    );
}

#[test]
fn invalid_self_size_errors() {
    should_panic(
        || {
            let o = must_err_load("invalid_self_size.wowm");
            print_struct(
                o.all_containers().next().unwrap(),
                &o,
                Version::World(MajorWorldVersion::Vanilla),
            );
        },
        INVALID_SELF_SIZE,
    );
}

#[test]
fn invalid_definer_value() {
    should_panic(
        || {
            let o = must_err_load("invalid_definer_value.wowm");
            print_message_stats(&o);
        },
        INVALID_DEFINER_VALUE,
    );
}

#[test]
fn duplicate_definer_value() {
    should_panic(
        || {
            let o = must_err_load("duplicate_definer_value.wowm");
            print_message_stats(&o);
        },
        DUPLICATE_DEFINER_VALUES,
    );
}

#[test]
fn invalid_integer_type() {
    should_panic(
        || {
            let o = must_err_load("invalid_integer_type.wowm");
            print_message_stats(&o);
        },
        INVALID_INTEGER_TYPE,
    );
}

#[test]
fn non_matching_if_statement_variable() {
    should_panic(
        || {
            let o = must_err_load("different_if_statement_variables.wowm");
            print_message_stats(&o);
        },
        NON_MATCHING_IF_VARIABLES,
    );
}

#[test]
fn unsupported_upcast() {
    should_panic(
        || {
            let o = must_err_load("unsupported_upcast.wowm");
            print_message_stats(&o);
        },
        UNSUPPORTED_UPCAST,
    );
}

#[test]
fn overlapping_versions_errors() {
    should_panic(
        || {
            must_err_load("overlapping_versions.wowm");
        },
        OVERLAPPING_VERSIONS,
    );
}

#[test]
fn both_versions_errors() {
    should_panic(
        || {
            must_err_load("both_versions.wowm");
        },
        BOTH_LOGIN_AND_WORLD_VERSIONS,
    );
}

#[test]
fn duplicate_field_names() {
    should_panic(
        || {
            must_err_load("duplicate_field_names.wowm");
        },
        DUPLICATE_FIELD_NAMES,
    );
}

#[test]
fn not_in_index() {
    should_panic(
        || {
            print_message_stats(&must_err_load("not_in_index.wowm"));
        },
        MESSAGE_NOT_IN_INDEX,
    );
}

#[test]
fn incorrect_name_in_index() {
    should_panic(
        || {
            print_message_stats(&must_err_load("incorrect_name_in_index.wowm"));
        },
        OPCODE_HAS_INCORRECT_NAME,
    );
}
