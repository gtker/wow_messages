use crate::file_info::FileInfo;
use crate::parser::types::version::MajorWorldVersion;
use crate::parser::types::IntegerType;
use crate::path_utils::opcodes_file;
use crate::{ObjectTags, CONTAINER_SELF_SIZE_FIELD, ENUM_SELF_VALUE_FIELD};
use std::process::exit;
use writer::ErrorWriter;

mod writer;

pub(crate) const COMPLEX_NOT_FOUND: i32 = 1;
pub(crate) const RECURSIVE_TYPE: i32 = 2;
pub(crate) const MISSING_ENUMERATOR: i32 = 3;
pub(crate) const ENUM_HAS_BITWISE_AND: i32 = 4;
pub(crate) const FLAG_HAS_EQUALS: i32 = 5;
pub(crate) const NO_VERSIONS: i32 = 6;
pub(crate) const INCORRECT_OPCODE_FOR_MESSAGE: i32 = 7;
pub(crate) const MULTIPLE_SELF_VALUE: i32 = 8;
pub(crate) const INVALID_SELF_SIZE: i32 = 9;
pub(crate) const INVALID_DEFINER_VALUE: i32 = 10;
pub(crate) const DUPLICATE_DEFINER_VALUES: i32 = 11;
pub(crate) const INVALID_INTEGER_TYPE: i32 = 12;
pub(crate) const NON_MATCHING_IF_VARIABLES: i32 = 13;
pub(crate) const UNSUPPORTED_UPCAST: i32 = 14;
pub(crate) const OVERLAPPING_VERSIONS: i32 = 15;
pub(crate) const BOTH_LOGIN_AND_WORLD_VERSIONS: i32 = 16;
pub(crate) const DUPLICATE_FIELD_NAMES: i32 = 17;
pub(crate) const MESSAGE_NOT_IN_INDEX: i32 = 18;
pub(crate) const OPCODE_HAS_INCORRECT_NAME: i32 = 19;
pub(crate) const TYPE_IS_UPCAST_TO_SAME: i32 = 20;

fn wowm_exit(s: ErrorWriter, code: i32) -> ! {
    #[cfg(not(test))]
    const TEST: bool = false;
    #[cfg(test)]
    const TEST: bool = true;

    if TEST {
        if std::env::var("WOWM_PRINT_TEST_ERRORS").is_ok() {
            s.print();
        }

        panic!("{}", code);
    } else {
        s.print();

        exit(code);
    }
}

fn print_version_cover(s: &mut ErrorWriter, msg: &str, tags: &ObjectTags) {
    s.wln(msg);
    if tags.logon_versions().next().is_some() {
        s.wln("    Login:");

        for t in tags.logon_versions() {
            s.wln(format!("        {t}"));
        }
    }

    if tags.versions().next().is_some() {
        s.wln("    World:");

        for t in tags.versions() {
            s.wln(format!("        {t}"));
        }
    }
}

pub(crate) fn complex_not_found(
    struct_name: &str,
    struct_tags: &ObjectTags,
    struct_fileinfo: &FileInfo,
    ty_name: &str,
    related: &[(&FileInfo, &ObjectTags)],
) -> ! {
    let mut s = ErrorWriter::new("Container has complex type that can not be found.");

    s.fileinfo(
        struct_fileinfo,
        format!("Type '{struct_name}' needs type '{ty_name}'"),
    );

    print_version_cover(&mut s, &format!("'{ty_name}' needs to cover:"), struct_tags);

    s.wln("Found types with same name:");
    s.newline();

    for r in related {
        s.fileinfo(r.0, "Type at:");
        print_version_cover(&mut s, "Covers:", r.1);
    }

    wowm_exit(s, COMPLEX_NOT_FOUND);
}

pub(crate) fn variable_in_if_not_found(
    variable_name: &str,
    name: &str,
    fileinfo: &FileInfo,
    ty_name: &str,
) -> ! {
    let mut s = ErrorWriter::new("Container uses enumerator in if statement that does not exist.");

    s.fileinfo(
        fileinfo,
        format!(
            "Unable to find enumerator with name '{name}' in variable '{variable_name}' with type '{ty_name}'"
        ),
    );

    wowm_exit(s, MISSING_ENUMERATOR);
}

pub(crate) fn recursive_type(name: &str, file_info: &FileInfo) -> ! {
    let mut s = ErrorWriter::new("Type contains itself which leads to infinite recursion.");

    s.fileinfo(file_info, format!("{name} contains itself."));

    wowm_exit(s, RECURSIVE_TYPE);
}

pub(crate) fn enum_has_bitwise_and(
    ty_name: &str,
    variable_name: &str,
    enum_ty_name: &str,
    file_info: &FileInfo,
) -> ! {
    let mut s = ErrorWriter::new("Enum is used with bitwise and (&) in if statement instead of equals (==) or not equals (!=).");

    s.fileinfo(file_info, format!("Enum '{enum_ty_name}' is used in if statement as bitwise and (&) as variable '{variable_name}' in type '{ty_name}'", ));

    wowm_exit(s, ENUM_HAS_BITWISE_AND);
}

pub(crate) fn flag_used_as_equals_or_not_equals(
    ty_name: &str,
    variable_name: &str,
    enum_ty_name: &str,
    file_info: &FileInfo,
) -> ! {
    let mut s = ErrorWriter::new("Flag is used as either equals (==) or not equals (!=) in if statement instead of bitwise and (&).");

    s.fileinfo(file_info, format!("Flag '{enum_ty_name}' is used in if statement as eqauals (==) or not equals (!=) as variable '{variable_name}' in type '{ty_name}'", ));

    wowm_exit(s, FLAG_HAS_EQUALS);
}

pub(crate) fn object_has_no_versions(ty_name: &str, file_info: &FileInfo) -> ! {
    let mut s = ErrorWriter::new("Object has no versions.");

    s.fileinfo(
        file_info,
        format!("Object '{ty_name}' does not have either a login version or a world version."),
    );

    wowm_exit(s, NO_VERSIONS)
}

pub(crate) fn incorrect_opcode_for_message(
    ty_name: &str,
    file_info: &FileInfo,
    expected_opcode: usize,
    actual: u32,
) -> ! {
    let mut s = ErrorWriter::new("Invalid opcode for message.");

    s.fileinfo(file_info, format!("Message '{ty_name}' is expected to have opcode '{expected_opcode}' but it has '{actual}'", ));

    wowm_exit(s, INCORRECT_OPCODE_FOR_MESSAGE)
}

pub(crate) fn multiple_self_value(
    ty_name: &str,
    file_info: &FileInfo,
    first_name: &str,
    second_name: &str,
) -> ! {
    let mut s = ErrorWriter::new(format!(
        "Multiple '{ENUM_SELF_VALUE_FIELD}' defined for enum.",
    ));

    s.fileinfo(file_info, format!("Type '{ty_name} has multiple enumerators with '{ENUM_SELF_VALUE_FIELD}', first field is '{first_name}', second name is '{second_name}'"));

    wowm_exit(s, MULTIPLE_SELF_VALUE);
}

pub(crate) fn invalid_self_size_position(
    ty_name: &str,
    file_info: &FileInfo,
    msg: impl AsRef<str>,
) -> ! {
    let mut s = ErrorWriter::new(format!("Invalid usage of '{CONTAINER_SELF_SIZE_FIELD}'."));

    s.fileinfo(
        file_info,
        format!(
            "Type '{ty_name}' has invalid usage of '{}': '{}'",
            CONTAINER_SELF_SIZE_FIELD,
            msg.as_ref()
        ),
    );

    wowm_exit(s, INVALID_SELF_SIZE);
}

pub(crate) fn invalid_definer_value(
    ty_name: &str,
    enumerator_name: &str,
    value: &str,
    file_info: &FileInfo,
) -> ! {
    let mut s = ErrorWriter::new("Enumerator has invalid value.");

    s.fileinfo(
        file_info,
        format!("Type '{ty_name}' has enumerator '{enumerator_name}' with invalid value '{value}'"),
    );

    wowm_exit(s, INVALID_DEFINER_VALUE);
}

pub(crate) fn duplicate_definer_value(
    ty_name: &str,
    first_enumerator_name: impl AsRef<str>,
    second_enumerator_name: &str,
    value: u64,
    file_info: &FileInfo,
) -> ! {
    let mut s = ErrorWriter::new("Definer has two fields with the same value.");

    s.fileinfo(file_info,
               format!(
                   "Type '{ty_name}' has enumerator '{first_enumerator_name}' and enumerator '{second_enumerator_name}' with the same value '{value}'",
                   first_enumerator_name = first_enumerator_name.as_ref(),
               ),
    );

    wowm_exit(s, DUPLICATE_DEFINER_VALUES);
}

pub(crate) fn invalid_integer_type(enum_name: &str, int_name: &str, file_info: &FileInfo) -> ! {
    let mut s = ErrorWriter::new("Invalid integer type");

    s.fileinfo(
        file_info,
        format!("Type '{enum_name}' using invalid integer type '{int_name}'"),
    );

    wowm_exit(s, INVALID_INTEGER_TYPE);
}

pub(crate) fn non_matching_if_statement_variables(
    ty_name: &str,
    first_variable_name: &str,
    second_variable_name: &str,
    file_info: &FileInfo,
) -> ! {
    let mut s = ErrorWriter::new("If statement variables are not the same");

    s.fileinfo(file_info, format!("Type '{ty_name}' has if statement with variable '{first_variable_name}' and '{second_variable_name}'. Wowm currently only allows or (||) expressions with the same variable'"));

    wowm_exit(s, NON_MATCHING_IF_VARIABLES);
}

pub(crate) fn unsupported_upcast(
    container_name: &str,
    variable_name: &str,
    ty_name: &str,
    upcast: &str,
    file_info: &FileInfo,
) -> ! {
    let mut s = ErrorWriter::new("Unsupported upcast for type.");
    s.fileinfo(file_info, format!("Type '{container_name}' variable '{variable_name}' with type '{ty_name}' has upcast '{upcast}' which is unsupported."));

    wowm_exit(s, UNSUPPORTED_UPCAST);
}

fn print_version_overlap(
    s: &mut ErrorWriter,
    msg: &str,
    tags: &ObjectTags,
    other_tags: &ObjectTags,
) {
    s.wln(msg);
    if tags.logon_versions().next().is_some() {
        s.wln("    Login:");

        for t in tags.logon_versions() {
            s.w(format!("        {t}"));

            let mut has_overlapped = false;
            for other_t in other_tags.logon_versions() {
                if t.overlaps(&other_t) {
                    if !has_overlapped {
                        has_overlapped = true;
                        s.w("    <-- Overlaps with: ");
                    } else {
                        s.w(", ");
                    }
                    s.w(other_t.to_string());
                }
            }
            s.newline();
        }
    }

    if tags.versions().next().is_some() {
        s.wln("    World:");

        for t in tags.versions() {
            s.w(format!("        {t}"));

            let mut has_overlapped = false;
            for other_t in other_tags.versions() {
                if t.overlaps(&other_t) {
                    if !has_overlapped {
                        has_overlapped = true;
                        s.w("    <-- Overlaps with: ");
                    } else {
                        s.w(", ");
                    }
                    s.w(other_t.to_string());
                }
            }
            s.newline();
        }
    }
}

pub(crate) fn overlapping_versions(
    name: &str,
    first_object_tags: &ObjectTags,
    first_object_file_info: &FileInfo,
    second_object_tags: &ObjectTags,
    second_object_file_info: &FileInfo,
) -> ! {
    let mut s = ErrorWriter::new("Objects with the same name have overlapping versions.");

    s.fileinfo(first_object_file_info, format!("First {name}:"));

    s.fileinfo(second_object_file_info, format!("Second {name}:"));

    print_version_overlap(
        &mut s,
        "First covers:",
        first_object_tags,
        second_object_tags,
    );
    print_version_overlap(
        &mut s,
        "Second covers:",
        second_object_tags,
        first_object_tags,
    );

    wowm_exit(s, OVERLAPPING_VERSIONS);
}

pub(crate) fn object_has_both_versions(ty_name: &str, file_info: &FileInfo) -> ! {
    let mut s = ErrorWriter::new("Object has both login and world versions.");

    s.fileinfo(
        file_info,
        format!("Object '{ty_name}' has both login and world versions."),
    );

    wowm_exit(s, BOTH_LOGIN_AND_WORLD_VERSIONS)
}

pub(crate) fn duplicate_field_names(ty_name: &str, field_name: &str, file_info: &FileInfo) -> ! {
    let mut s = ErrorWriter::new("Object has multiple fields with the same name.");

    s.fileinfo(
        file_info,
        format!("Object '{ty_name}' has multiple fields with the name '{field_name}'."),
    );

    wowm_exit(s, DUPLICATE_FIELD_NAMES)
}

pub(crate) fn opcode_has_incorrect_name(
    ty_name: &str,
    index_name: &str,
    file_info: &FileInfo,
    opcode: usize,
    version: MajorWorldVersion,
) -> ! {
    let mut s = ErrorWriter::new("Opcode has different name than in 'opcodes.rs' index file.");
    let opcodes_file = opcodes_file(version);

    s.fileinfo(
        file_info,
        format!(
            "Opcode '{opcode:#06X?}' found with name {ty_name} while index has '{index_name}' for version '{version}' in index file '{opcodes_file}'",
            opcodes_file = opcodes_file.display(),
            version = version.module_name(),
        ),
    );

    wowm_exit(s, OPCODE_HAS_INCORRECT_NAME)
}

pub(crate) fn message_not_in_index(
    ty_name: &str,
    file_info: &FileInfo,
    opcode: usize,
    version: MajorWorldVersion,
) -> ! {
    let mut s = ErrorWriter::new("Message not in 'opcodes.rs' index file.");
    let opcodes_file = opcodes_file(version);

    s.fileinfo(
        file_info,
        format!(
            "Message '{ty_name}' with opcode '{opcode:#06X?}' for version '{version}' is not in file '{opcodes_file}'",
            opcodes_file = opcodes_file.display(),
            version = version.module_name(),
        ),
    );

    wowm_exit(s, MESSAGE_NOT_IN_INDEX)
}

pub(crate) fn type_is_upcast_to_same(ty_name: &str, file_info: &FileInfo, ty: IntegerType) -> ! {
    let mut s = ErrorWriter::new("Type is upcast to same type.");

    let ty = ty.str();
    s.fileinfo(
        file_info,
        format!("Type '{ty_name}' of integer type '{ty}' is upcast to the same type.",),
    );

    wowm_exit(s, TYPE_IS_UPCAST_TO_SAME)
}
