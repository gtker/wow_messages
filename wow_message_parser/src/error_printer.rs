use crate::file_info::FileInfo;
use crate::Tags;
use std::fmt::Write;
use std::process::exit;

pub(crate) const COMPLEX_NOT_FOUND: i32 = 1;
pub(crate) const RECURSIVE_TYPE: i32 = 2;
pub(crate) const MISSING_ENUMERATOR: i32 = 3;
pub(crate) const ENUM_HAS_BITWISE_AND: i32 = 4;

pub struct ErrorWriter {
    inner: String,
}

impl ErrorWriter {
    pub(crate) fn new(msg: &str) -> Self {
        let mut s = Self {
            inner: String::with_capacity(8000),
        };

        s.wln(format!("WOWM ERROR: {}", msg));
        s.newline();

        s
    }

    pub(crate) fn fileinfo(&mut self, fileinfo: &FileInfo, reason: impl AsRef<str>) {
        self.wln(format!("{}:{}:", fileinfo.name(), fileinfo.start_line()));
        self.wln(reason);
        self.newline();
    }

    pub(crate) fn w(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub(crate) fn newline(&mut self) {
        self.inner.write_char('\n').unwrap();
    }

    pub(crate) fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.newline();
    }

    pub(crate) fn print(&self) {
        eprintln!("{}", self.inner)
    }
}

fn wowm_exit(s: ErrorWriter, code: i32) -> ! {
    #[cfg(not(test))]
    const TEST: bool = false;
    #[cfg(test)]
    const TEST: bool = true;

    if TEST {
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
        format!("    Type '{}' needs type '{}'", struct_name, ty_name),
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

pub(crate) fn recursive_type(name: &str, file_info: &FileInfo) {
    let mut s = ErrorWriter::new("Type contains itself which leads to infinite recursion.");

    s.fileinfo(file_info, format!("    {} contains itself.", name));

    wowm_exit(s, RECURSIVE_TYPE);
}

pub(crate) fn enum_has_bitwise_and(
    ty_name: &str,
    variable_name: &str,
    enum_ty_name: &str,
    file_info: &FileInfo,
) {
    let mut s = ErrorWriter::new("Enum is used with bitwise and (&) in if statement.");

    s.fileinfo(file_info, format!("Enum '{enum_ty_name}' is used in if statement as bitwise and (&) as variable '{variable_name}' in type '{ty_name}'", ));

    wowm_exit(s, ENUM_HAS_BITWISE_AND);
}
