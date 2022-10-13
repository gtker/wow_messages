use crate::file_info::FileInfo;
use crate::Tags;
use std::process::exit;

const COMPLEX_NOT_FOUND: i32 = 1;

pub(crate) fn complex_not_found(
    struct_name: &str,
    struct_tags: &Tags,
    struct_fileinfo: &FileInfo,
    ty_name: &str,
) -> ! {
    eprintln!("WOWM ERROR: Container has complex type that can not be found.");
    eprintln!();
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

    exit(COMPLEX_NOT_FOUND);
}
