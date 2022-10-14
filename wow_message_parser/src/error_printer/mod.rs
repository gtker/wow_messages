use crate::file_info::FileInfo;
use crate::{Tags, CONTAINER_SELF_SIZE_FIELD, ENUM_SELF_VALUE_FIELD};
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

pub(crate) fn complex_not_found(
    struct_name: &str,
    struct_tags: &Tags,
    struct_fileinfo: &FileInfo,
    ty_name: &str,
) -> ! {
    let mut s = ErrorWriter::new("Container has complex type that can not be found.");

    s.fileinfo(
        struct_fileinfo,
        format!("Type '{}' needs type '{}'", struct_name, ty_name),
    );

    s.wln(format!("    '{}' needs to cover versions:", ty_name));
    if !struct_tags.logon_versions().collect::<Vec<_>>().is_empty() {
        s.wln("    Login:");

        for t in struct_tags.logon_versions() {
            s.wln(format!("        {}", t));
        }
    }

    if !struct_tags.versions().collect::<Vec<_>>().is_empty() {
        s.wln("    World:");

        for t in struct_tags.versions() {
            s.wln(format!("        {}", t));
        }
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
            "Unable to find enumerator with name '{}' in variable '{}' with type '{}'",
            name, variable_name, ty_name
        ),
    );

    wowm_exit(s, MISSING_ENUMERATOR);
}

pub(crate) fn recursive_type(name: &str, file_info: &FileInfo) -> ! {
    let mut s = ErrorWriter::new("Type contains itself which leads to infinite recursion.");

    s.fileinfo(file_info, format!("{} contains itself.", name));

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
    actual: u16,
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
        "Multiple '{self_value}' defined for enum.",
        self_value = ENUM_SELF_VALUE_FIELD,
    ));

    s.fileinfo(file_info, format!("Type '{ty_name} has multiple enumerators with '{self_value}', first field is '{first_name}', second name is '{second_name}'", self_value = ENUM_SELF_VALUE_FIELD));

    wowm_exit(s, MULTIPLE_SELF_VALUE);
}

pub(crate) fn invalid_self_size_position(
    ty_name: &str,
    file_info: &FileInfo,
    msg: impl AsRef<str>,
) -> ! {
    let mut s = ErrorWriter::new(format!("Invalid usage of '{}'.", CONTAINER_SELF_SIZE_FIELD));

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
