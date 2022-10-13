use crate::file_info::FileInfo;
use crate::Tags;
use std::process::exit;

const COMPLEX_NOT_FOUND: i32 = 1;

fn wowm_error(msg: &str) {
    eprintln!("WOWM ERROR: {}", msg);
    eprintln!();
}

fn wowm_exit(code: i32) -> ! {
    exit(code);
}

pub(crate) fn complex_not_found(
    struct_name: &str,
    struct_tags: &Tags,
    struct_fileinfo: &FileInfo,
    ty_name: &str,
) -> ! {
    wowm_error("Container has complex type that can not be found.");

    eprintln!(
        "{}:{}:",
        struct_fileinfo.name(),
        struct_fileinfo.start_line()
    );
    eprintln!("    Type '{}' needs type '{}'", struct_name, ty_name);
    eprintln!();
    eprintln!("    '{}' needs to cover versions:", ty_name);
    if !struct_tags.logon_versions().collect::<Vec<_>>().is_empty() {
        eprintln!("    Login:");

        for t in struct_tags.logon_versions() {
            eprintln!("        {}", t);
        }
    }

    if !struct_tags.versions().collect::<Vec<_>>().is_empty() {
        eprintln!("    World:");

        for t in struct_tags.versions() {
            eprintln!("        {}", t);
        }
    }

    wowm_exit(COMPLEX_NOT_FOUND);
}

pub(crate) fn variable_in_if_not_found(variable_name: &str, name: &str, ty_name: &str) -> ! {
    wowm_error("Container uses enumerator in if statement that does not exist.");

    panic!(
        "unable to find enumerator with name '{}' in variable '{}' with type '{}'",
        name, variable_name, ty_name
    )
}
