use crate::file_info::FileInfo;
use std::fmt::Write;
use std::fs::read_to_string;

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
        self.wln(format!("    {}", reason.as_ref()));

        self.newline();

        if let Ok(contents) = read_to_string(fileinfo.path()) {
            self.print_lines_of_file(&contents, fileinfo);
        } else {
            self.wln("Unable to open file for printing.");
        }

        self.newline();
    }

    fn print_lines_of_file(&mut self, contents: &str, fileinfo: &FileInfo) {
        let mut i = 1_usize; // There is no line 0

        let mut split = contents;
        while let Some((a, b)) = split.split_once('\n') {
            if i > fileinfo.end_line() {
                break;
            }

            if i >= fileinfo.start_line() {
                self.wln(a);
            }

            split = b;
            i += 1;
        }
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
