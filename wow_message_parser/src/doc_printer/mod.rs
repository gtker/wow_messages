pub mod container;
pub mod definer;

use crate::file_utils::create_and_overwrite_if_not_same_contents;
use crate::parser::types::tags::{LoginVersion, Tags, WorldVersion};
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::read_to_string;
use std::path::Path;

pub struct DocWriter {
    name: String,
    inner: String,
    column: usize,
    tags: Tags,
}

impl DocWriter {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }

    pub fn new(name: &str, tags: &Tags) -> Self {
        Self {
            name: name.to_string(),
            inner: String::with_capacity(8000),
            column: 0,
            tags: tags.clone(),
        }
    }

    pub fn w(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
        self.column += s.as_ref().len();
    }

    pub fn w_break_at(&mut self, s: impl AsRef<str>) {
        self.w(s);
        if self.column > 80 {
            self.newline();
        }
    }

    pub fn newline(&mut self) {
        self.w("\n");
        self.column = 0;
    }

    pub fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.newline();
    }

    pub fn wln_no_indent(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
        self.newline();
    }

    pub fn bytes<'a>(&mut self, bytes: impl Iterator<Item = &'a u8>) {
        for b in bytes {
            let text = format!("{}, ", b);
            self.w(&text);
            self.column += text.len();
        }
    }
}

fn create_or_append_hashmap(s: &str, path: String, files: &mut HashMap<String, String>) {
    if let Some(c) = files.get_mut(&path) {
        c.push_str(s);
    } else {
        files.insert(path, s.to_string());
    }
}

pub fn print_docs_summary_and_objects(definers: &[DocWriter], containers: &[DocWriter]) {
    const LOGIN_DEFINER_HEADER: &str = "# Login Definers";
    const LOGIN_CONTAINER_HEADER: &str = "# Login Containers\n";
    const WORLD_DEFINER_HEADER: &str = "# World Definers\n";
    const WORLD_CONTAINER_HEADER: &str = "# Login Containers\n";
    const SUMMARY_PATH: &str = "wowm_language/src/SUMMARY.md";

    let s = read_to_string(SUMMARY_PATH).unwrap();
    let (s, _) = s.split_once(LOGIN_DEFINER_HEADER).unwrap();
    let mut s = s.to_string();

    let mut already_added_files = Vec::new();
    let mut login_definers = Vec::new();
    let mut world_definers = Vec::new();

    let mut files = HashMap::new();

    for definer in definers {
        let path = format!(
            "docs/{lower_name}.md",
            lower_name = definer.name().to_lowercase()
        );

        create_or_append_hashmap(
            definer.inner(),
            "wowm_language/src/".to_string() + &path,
            &mut files,
        );

        if already_added_files.contains(&path) {
            continue;
        }

        let bullet_point = format!("- [{name}]({path})\n", name = definer.name(), path = path,);
        if definer.tags().has_login_version() {
            login_definers.push(bullet_point)
        } else {
            world_definers.push(bullet_point);
        }

        already_added_files.push(path);
    }

    s.push_str(LOGIN_DEFINER_HEADER);
    for i in login_definers {
        s.push_str(&i);
    }
    s.push('\n');

    s.push_str(WORLD_DEFINER_HEADER);
    for i in world_definers {
        s.push_str(&i);
    }
    s.push('\n');

    let mut login_containers = Vec::new();
    let mut world_containers = Vec::new();
    for container in containers {
        let path = format!(
            "docs/{lower_name}.md",
            lower_name = container.name().to_lowercase()
        );

        create_or_append_hashmap(
            container.inner(),
            "wowm_language/src/".to_string() + &path,
            &mut files,
        );

        if already_added_files.contains(&path) {
            continue;
        }

        let bullet_point = format!("- [{name}]({path})\n", name = container.name(), path = path,);
        if container.tags().has_login_version() {
            login_containers.push(bullet_point)
        } else {
            world_containers.push(bullet_point);
        }

        already_added_files.push(path);
    }

    s.push_str(LOGIN_CONTAINER_HEADER);
    for i in login_containers {
        s.push_str(&i);
    }
    s.push('\n');

    s.push_str(WORLD_CONTAINER_HEADER);
    for i in world_containers {
        s.push_str(&i);
    }
    s.push('\n');

    create_and_overwrite_if_not_same_contents(&s, Path::new(SUMMARY_PATH));

    for (path, s) in files {
        create_and_overwrite_if_not_same_contents(&s, Path::new(&path));
    }
}

fn common(s: &mut DocWriter, tags: &Tags) {
    s.wln(format!("# {}", &s.name));
    s.newline();

    print_versions(s, tags.logon_versions(), tags.versions());

    print_metadata(s, tags);
}

fn print_metadata(s: &mut DocWriter, tags: &Tags) {
    if let Some(description) = tags.description() {
        s.wln("### Description");
        s.newline();
        for l in description.as_doc_lines() {
            s.wln(l);
            s.newline();
        }
    }

    if let Some(comment) = tags.comment() {
        s.wln("### Comment");
        s.newline();
        for l in comment.as_doc_lines() {
            s.wln(l);
            s.newline();
        }
    }
}

fn print_versions(
    s: &mut DocWriter,
    login_versions: &[LoginVersion],
    world_versions: &[WorldVersion],
) {
    s.w("## ");

    for (i, l) in login_versions.iter().enumerate() {
        s.w(format!("Protocol Version {}", l));
        if i != login_versions.len() - 1 {
            s.w(", ");
        }
    }

    for (i, l) in world_versions.iter().enumerate() {
        s.w(format!("Client Version {}", l));
        if i != world_versions.len() - 1 {
            s.w(", ");
        }
    }

    s.newline();
    s.newline();
}
