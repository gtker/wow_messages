use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

use heck::SnakeCase;

use crate::parser::types::tags::{LoginVersion, Tags, WorldVersion};
use crate::rust_printer::Writer;

pub const LOGIN_DIR: &str = "wow_login_messages/src/logon";
pub const WORLD_DIR: &str = "wow_vanilla_messages/src/world";

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SubmoduleLocation {
    PubUseInternal,
    PubMod,
}

#[derive(Debug)]
pub struct ModFile {
    pub name: String,
    pub submodules: Vec<(String, SubmoduleLocation)>,
}

#[derive(Debug)]
pub struct ModFiles {
    v: Vec<ModFile>,
}

impl ModFiles {
    pub fn write_mod_files(&mut self) {
        for m in &mut self.v {
            m.submodules.sort();
            m.submodules.dedup();
        }

        for m in &self.v {
            let mut s = String::new();
            for (i, location) in &m.submodules {
                match location {
                    SubmoduleLocation::PubUseInternal => {
                        s.push_str(&format!("mod {};\n", i));
                        s.push_str(&format!("pub use {}::*;\n", i));
                    }
                    SubmoduleLocation::PubMod => {
                        s.push_str(&format!("pub mod {};\n", i));
                    }
                }
            }
            let filename = m.name.to_string() + "mod.rs";
            write_string_to_file(&s, Path::new(&filename));
        }
    }

    pub fn new() -> Self {
        Self { v: vec![] }
    }

    fn add_or_append_file(&mut self, file_dir: String, e: (String, SubmoduleLocation)) {
        if let Some(v) = self.v.iter_mut().find(|a| a.name == file_dir) {
            v.submodules.push(e);
        } else {
            self.v.push(ModFile {
                name: file_dir,
                submodules: vec![e],
            })
        }
    }

    pub fn add_world_file(&mut self, name: &str, version: &WorldVersion) {
        let (m, i) = match version {
            WorldVersion::Minor(m, i) => (m, i),
            WorldVersion::Major(_)
            | WorldVersion::Patch(_, _, _)
            | WorldVersion::Exact(_, _, _, _)
            | WorldVersion::All => unimplemented!(),
        };

        self.add_or_append_file(
            format!("{}/", WORLD_DIR),
            (format!("v{}", m), SubmoduleLocation::PubMod),
        );

        self.add_or_append_file(
            format!("{}/v{}/", WORLD_DIR, m),
            (format!("v{}", i), SubmoduleLocation::PubMod),
        );

        let file_dir = format!("{}/v{}/v{}/", WORLD_DIR, m, i);
        self.add_or_append_file(
            file_dir.clone(),
            (get_module_name(name), SubmoduleLocation::PubUseInternal),
        );
        self.add_or_append_file(file_dir, ("opcodes".to_string(), SubmoduleLocation::PubMod));
    }

    pub fn add_login_file(&mut self, name: &str, version: &LoginVersion) {
        let e = match version {
            LoginVersion::Specific(v) => {
                format!("version_{}", v)
            }
            LoginVersion::All => "all".to_string(),
        };
        let e = (e, SubmoduleLocation::PubMod);

        let top_level_dir = format!("{}/", LOGIN_DIR);

        if let Some(v) = self.v.iter_mut().find(|a| a.name == top_level_dir) {
            v.submodules.push(e);
        } else {
            self.v.push(ModFile {
                name: top_level_dir,
                submodules: vec![e],
            })
        }

        let module_level_dir = get_login_version_file_path(version);

        let e = (get_module_name(name), SubmoduleLocation::PubUseInternal);

        if let Some(v) = self.v.iter_mut().find(|a| a.name == module_level_dir) {
            v.submodules.push(e);
        } else {
            self.v.push(ModFile {
                name: module_level_dir,
                submodules: match version {
                    LoginVersion::Specific(_) => {
                        vec![e, ("opcodes".to_string(), SubmoduleLocation::PubMod)]
                    }
                    LoginVersion::All => {
                        vec![e]
                    }
                },
            })
        }
    }

    pub fn write_contents_to_file(&mut self, name: &str, tags: &Tags, s: &Writer) {
        for (i, version) in tags.logon_versions().iter().enumerate() {
            let path = get_login_filepath(name, version);
            self.add_login_file(name, version);
            let s = if i == 0 {
                s.proper_as_str()
            } else {
                s.imports()
            };
            write_string_to_file(s, Path::new(&path));
        }

        for (i, version) in tags.versions().iter().enumerate() {
            let path = get_world_filepath(name, version);
            self.add_world_file(name, version);
            let s = if i == 0 {
                s.proper_as_str()
            } else {
                s.imports()
            };
            write_string_to_file(s, Path::new(&path));
        }
    }
}

pub fn get_world_version_path(version: &WorldVersion) -> String {
    match version {
        WorldVersion::Major(m) => format!("crate::world::v{}", m),
        WorldVersion::Minor(m, i) => format!("crate::world::v{}::v{}", m, i),
        WorldVersion::Patch(m, i, p) => format!("crate::world::v{}::v{}::v{}", m, i, p),
        WorldVersion::Exact(m, i, p, b) => format!("crate::world::v{}::v{}::v{}::v{}", m, i, p, b),
        WorldVersion::All => "crate::world".to_string(),
    }
}

pub fn get_world_version_file_path(version: &WorldVersion) -> String {
    match version {
        WorldVersion::Major(m) => format!("{}/v{}/", WORLD_DIR, m),
        WorldVersion::Minor(m, i) => format!("{}/v{}/v{}/", WORLD_DIR, m, i),
        WorldVersion::Patch(m, i, p) => format!("{}/v{}/v{}/v{}/", WORLD_DIR, m, i, p),
        WorldVersion::Exact(m, i, p, b) => {
            format!("{}/v{}/v{}/v{}/v{}/", WORLD_DIR, m, i, p, b)
        }
        WorldVersion::All => format!("{}/", WORLD_DIR),
    }
}

pub fn get_login_logon_version_path(version: &LoginVersion) -> String {
    match version {
        LoginVersion::Specific(v) => {
            format!("crate::logon::version_{}", v)
        }
        LoginVersion::All => "crate::logon::all".to_string(),
    }
}

pub fn get_login_version_file_path(version: &LoginVersion) -> String {
    match version {
        LoginVersion::Specific(v) => {
            format!(
                "{login_dir}/version_{version}/",
                login_dir = LOGIN_DIR,
                version = v
            )
        }
        LoginVersion::All => {
            format!("{login_dir}/all/", login_dir = LOGIN_DIR,)
        }
    }
}

pub fn get_import_path(tags: &Tags) -> String {
    if let Some(f) = tags.logon_versions().first() {
        get_login_logon_version_path(f)
    } else if let Some(f) = tags.versions().first() {
        get_world_version_path(f)
    } else {
        panic!(
            "get_import_path does not have logon or reconnect version: {:#?}",
            tags
        )
    }
}

fn get_world_filepath(object_name: &str, version: &WorldVersion) -> String {
    let s = get_world_version_file_path(version);
    s + &get_module_name(object_name) + ".rs"
}

fn get_login_filepath(object_name: &str, version: &LoginVersion) -> String {
    let s = get_login_version_file_path(version);
    s + &get_module_name(object_name) + ".rs"
}

pub fn append_string_to_file(s: &str, filename: &Path) {
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open(filename)
        .unwrap();
    f.write_all(s.as_bytes()).unwrap();
}

pub fn write_string_to_file(s: &str, filename: &Path) {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename);
    let mut f = match f {
        Ok(f) => f,
        Err(_) => {
            let dir = filename.parent().unwrap();
            std::fs::create_dir_all(dir).unwrap();
            std::fs::File::create(filename)
                .unwrap_or_else(|_| panic!("unable to open file: '{}'", filename.to_str().unwrap()))
        }
    };

    f.write_all(s.as_bytes()).unwrap();
}

pub fn create_or_append(s: &str, filename: &Path) {
    let f = std::fs::OpenOptions::new().append(true).open(filename);
    if let Ok(mut f) = f {
        f.write_all(s.as_bytes()).unwrap();
    } else {
        write_string_to_file(s, filename);
    }
}

pub fn overwrite_if_not_same_contents(s: &str, filename: &Path) {
    let f = read_to_string(filename).unwrap();
    if f != s {
        write_string_to_file(s, filename);
    }
}

pub fn get_module_name(e: &str) -> String {
    e.to_snake_case()
}
