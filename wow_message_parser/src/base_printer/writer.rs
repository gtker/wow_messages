use std::fmt::Write;

pub(crate) struct Writer {
    inner: String,
    indentation: u8,
}

impl Writer {
    const INDENTATION: &'static str = "    ";

    pub fn new() -> Self {
        Self {
            inner: String::with_capacity(8000),
            indentation: 0,
        }
    }

    pub fn inc_indent(&mut self) {
        if self.indentation == 0xff {
            panic!("attempting to overflow indentation")
        }

        self.indentation += 1;
    }

    pub fn dec_indent(&mut self) {
        if self.indentation == 0 {
            panic!("attempting to underflow indentation");
        }

        self.indentation -= 1;
    }

    pub fn w_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub fn w(&mut self, s: impl AsRef<str>) {
        for _ in 0..self.indentation {
            self.inner.write_str(Self::INDENTATION).unwrap();
        }

        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub fn newline(&mut self) {
        self.inner.write_char('\n').unwrap();
    }

    pub fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.newline();
    }

    pub fn wln_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
        self.newline();
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }
}
