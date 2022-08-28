use std::collections::BTreeMap;
use std::fmt::Write as wfmt;
use std::fs::{read_to_string, remove_file};
use std::io::Write;
use std::path::Path;

use heck::SnakeCase;
use walkdir::WalkDir;

use crate::parser::types::tags::{LoginVersion, Tags, WorldVersion};
use crate::rust_printer::Writer;

pub const LOGIN_DIR: &str = "wow_login_messages/src/logon";
pub const WORLD_DIR: &str = "wow_world_messages/src/world";
pub const UPDATE_MASK_LOCATION: &str = "wow_world_messages/src/helper/update_mask/impls.rs";
pub const BASE_DIR: &str = "wow_world_base/src/inner";

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
    already_existing_files: BTreeMap<String, bool>,
}

impl ModFiles {
    pub fn remove_unwritten_files(&self) {
        for (filename, written) in self.already_existing_files.iter() {
            if !written {
                remove_file(Path::new(filename)).unwrap();
            }
        }
    }

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
                        writeln!(s, "pub(crate) mod {};", i).unwrap();
                        writeln!(s, "pub use {}::*;", i).unwrap();
                    }
                    SubmoduleLocation::PubMod => {
                        if ["vanilla", "wrath", "tbc"].contains(&i.as_str()) {
                            writeln!(s, "#[cfg(feature = \"{}\")]", i).unwrap();
                        }
                        writeln!(s, "pub mod {};", i).unwrap();
                    }
                }
            }
            let filename = m.name.to_string() + "mod.rs";

            self.already_existing_files.insert(filename.clone(), true);

            create_and_overwrite_if_not_same_contents(&s, Path::new(&filename));
        }
    }

    pub fn new() -> Self {
        let mut already_existing_files = BTreeMap::new();

        for dir in [LOGIN_DIR, WORLD_DIR, BASE_DIR] {
            for file in WalkDir::new(dir).into_iter().filter_map(|a| a.ok()) {
                if !file.file_type().is_file() {
                    continue;
                }

                already_existing_files.insert(file.path().to_str().unwrap().to_string(), false);
            }
        }

        Self {
            v: vec![],
            already_existing_files,
        }
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

    pub fn add_world_file(&mut self, name: &str, version: &WorldVersion, tags: &Tags) {
        assert!(version.is_main_version());

        if tags.is_in_common() {
            self.add_or_append_file(
                format!("{}/", BASE_DIR),
                (
                    major_version_to_string(version).to_string(),
                    SubmoduleLocation::PubMod,
                ),
            );

            self.add_or_append_file(
                format!("{}/{}/", BASE_DIR, major_version_to_string(version)),
                (get_module_name(name), SubmoduleLocation::PubUseInternal),
            );
        }

        self.add_or_append_file(
            format!("{}/", WORLD_DIR),
            (
                major_version_to_string(version).to_string(),
                SubmoduleLocation::PubMod,
            ),
        );

        let file_dir = format!("{}/{}/", WORLD_DIR, major_version_to_string(version));
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

    pub fn write_common_contents_to_file(
        &mut self,
        name: &str,
        tags: &Tags,
        common_s: &Writer,
        world_s: &Writer,
    ) {
        for version in tags.main_versions() {
            let world_path = get_world_filepath(name, version);
            let common_path = get_common_filepath(name, version);

            self.add_world_file(name, version, tags);

            self.already_existing_files.insert(world_path.clone(), true);
            self.already_existing_files
                .insert(common_path.clone(), true);

            create_and_overwrite_if_not_same_contents(world_s.inner(), Path::new(&world_path));
            create_and_overwrite_if_not_same_contents(common_s.inner(), Path::new(&common_path));
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

            self.already_existing_files.insert(path.clone(), true);

            create_and_overwrite_if_not_same_contents(s, Path::new(&path));
        }

        for version in tags.main_versions() {
            let path = get_world_filepath(name, version);

            self.add_world_file(name, version, tags);

            self.already_existing_files.insert(path.clone(), true);

            create_and_overwrite_if_not_same_contents(s.proper_as_str(), Path::new(&path));
        }
    }
}

fn major_version_to_string(v: &WorldVersion) -> &'static str {
    fn version(m: u8) -> &'static str {
        if m == 1 {
            "vanilla"
        } else if m == 2 {
            "tbc"
        } else if m == 3 {
            "wrath"
        } else {
            unreachable!()
        }
    }

    match *v {
        WorldVersion::Major(m) => {
            assert!((1..=3).contains(&m));

            version(m)
        }
        WorldVersion::Minor(m, i) => {
            match (m, i) {
                (1, 12) | (2, 4) | (3, 3) => {}
                _ => unreachable!(),
            }

            version(m)
        }
        WorldVersion::Patch(m, i, p) => {
            match (m, i, p) {
                (2, 4, 3) | (3, 3, 5) => {}
                _ => unreachable!(),
            }

            version(m)
        }
        WorldVersion::Exact(_, _, _, _) | WorldVersion::All => unimplemented!(),
    }
}

pub fn get_world_version_path(version: &WorldVersion) -> String {
    format!("crate::world::{}", major_version_to_string(version))
}

pub fn get_world_version_file_path(version: &WorldVersion) -> String {
    format!("{}/{}/", WORLD_DIR, major_version_to_string(version))
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
    } else if let Some(f) = tags.first_major_version() {
        get_world_version_path(f)
    } else {
        panic!(
            "get_import_path does not have logon or reconnect version: {:#?}",
            tags
        )
    }
}

fn get_common_filepath(object_name: &str, version: &WorldVersion) -> String {
    let s = format!("{}/{}/", BASE_DIR, major_version_to_string(version));
    s + &get_module_name(object_name) + ".rs"
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

pub fn overwrite_if_not_same_contents(s: &str, filename: &Path) {
    let f = read_to_string(filename).unwrap();
    if f != s {
        write_string_to_file(s, filename);
    }
}

pub fn create_and_overwrite_if_not_same_contents(s: &str, filename: &Path) {
    let f = std::fs::OpenOptions::new().open(filename);
    if f.is_ok() {
        let f = read_to_string(filename).unwrap();
        if f != s {
            write_string_to_file(s, filename);
        }
    } else {
        write_string_to_file(s, filename);
    }
}

pub fn get_module_name(e: &str) -> String {
    e.to_snake_case()
}
