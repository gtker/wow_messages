use crate::parser::enumerator::Definer;
use crate::rust_printer::Writer;
use crate::{ENUM_SELF_VALUE_FIELD, LOGIN_MESSAGES_GITHUB_REPO, WORLD_MESSAGES_GITHUB_REPO};
use std::fmt::Write;

struct WowmWriter {
    inner: String,
    prefix: String,
    indentation: u8,
}

impl WowmWriter {
    pub fn new(prefix: &str) -> Self {
        Self {
            inner: String::with_capacity(4000),
            prefix: prefix.to_string(),
            indentation: 0,
        }
    }

    pub fn w(&mut self, s: impl AsRef<str>) {
        for _ in 0..self.indentation {
            self.inner.write_str(Writer::INDENTATION);
        }

        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub fn newline(&mut self) {
        self.w("\n");
    }

    pub fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.newline();
    }

    pub fn inc(&mut self) {
        assert_ne!(self.indentation, 0xFF);

        self.indentation += 1;
    }

    pub fn dec(&mut self) {
        assert_ne!(self.indentation, 0);

        self.indentation -= 1;
    }
}

pub fn get_definer_wowm_definition(kind: &str, e: &Definer, prefix: &str) -> String {
    let mut s = WowmWriter::new(prefix);
    s.wln(&format!(
        "{kind} {name} : {ty} {{",
        kind = kind,
        name = e.name(),
        ty = e.ty().str(),
    ));

    s.inc();
    for field in e.fields() {
        s.wln(format!(
            "{name} = {val};",
            name = field.name(),
            val = field.value().original()
        ));
    }

    if let Some(f) = e.self_value() {
        s.wln(format!(
            "{name} = {self_value}",
            name = f.name(),
            self_value = ENUM_SELF_VALUE_FIELD
        ));
    }
    s.dec();
    s.wln("}");

    s.inner
}
