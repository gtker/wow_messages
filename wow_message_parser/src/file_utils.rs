use std::collections::BTreeMap;
use std::fmt::Write as wfmt;
use std::fs::{read_to_string, remove_file};
use std::io::Write;
use std::path::{Path, PathBuf};

use heck::SnakeCase;
use walkdir::WalkDir;

use crate::parser::types::tags::Tags;
use crate::parser::types::version::Version;
use crate::parser::types::version::{LoginVersion, WorldVersion};
use crate::path_utils;
use crate::path_utils::{base_directory, login_directory, world_directory};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum SubmoduleLocation {
    PubUseInternal,
    PubMod,
    PubCrateMod,
    PubUseOnly,
}

#[derive(Debug)]
pub(crate) struct ModFile {
    pub name: PathBuf,
    pub submodules: Vec<(String, SubmoduleLocation)>,
}

#[derive(Debug)]
pub(crate) struct ModFiles {
    v: Vec<ModFile>,
    already_existing_files: BTreeMap<PathBuf, bool>,
}

impl ModFiles {
    pub(crate) fn remove_unwritten_files(&self) {
        for (filename, written) in self.already_existing_files.iter() {
            if !written {
                remove_file(Path::new(filename)).unwrap();
            }
        }
    }

    pub(crate) fn write_mod_files(&mut self) {
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
                    SubmoduleLocation::PubUseOnly => {
                        writeln!(s, "pub use {}::*;", i).unwrap();
                    }
                    SubmoduleLocation::PubCrateMod => {
                        writeln!(s, "pub(crate) mod {};", i).unwrap();
                    }
                }
            }
            let filename = m.name.clone().join("mod.rs");

            create_and_overwrite_if_not_same_contents(&s, &filename);
            self.already_existing_files
                .insert(filename.canonicalize().unwrap(), true);
        }
    }

    pub(crate) fn new() -> Self {
        let mut already_existing_files = BTreeMap::new();

        for dir in [login_directory(), world_directory(), base_directory()] {
            for file in WalkDir::new(dir).into_iter().filter_map(|a| a.ok()) {
                if !file.file_type().is_file() {
                    continue;
                }

                let filename = file.path().canonicalize().unwrap();

                already_existing_files.insert(filename, false);
            }
        }

        Self {
            v: vec![],
            already_existing_files,
        }
    }

    fn add_or_append_file(&mut self, file_dir: PathBuf, e: (String, SubmoduleLocation)) {
        if let Some(v) = self.v.iter_mut().find(|a| a.name == file_dir) {
            v.submodules.push(e);
        } else {
            self.v.push(ModFile {
                name: file_dir,
                submodules: vec![e],
            })
        }
    }

    pub(crate) fn add_world_shared_file(
        &mut self,
        name: &str,
        versions: &[WorldVersion],
        tags: &Tags,
    ) {
        let base_path = if tags.is_in_base() {
            base_directory()
        } else {
            world_directory()
        };

        self.add_or_append_file(
            base_path.clone(),
            ("shared".to_string(), SubmoduleLocation::PubCrateMod),
        );

        self.add_or_append_file(
            base_path.join("shared"),
            (
                get_shared_module_name(name, versions),
                SubmoduleLocation::PubMod,
            ),
        );
    }

    pub(crate) fn add_world_file(&mut self, name: &str, version: &WorldVersion, tags: &Tags) {
        assert!(version.is_main_version());

        if tags.is_in_base() {
            self.add_or_append_file(
                base_directory(),
                (
                    major_version_to_string(version).to_string(),
                    SubmoduleLocation::PubMod,
                ),
            );

            self.add_or_append_file(
                base_directory().join(major_version_to_string(version)),
                (get_module_name(name), SubmoduleLocation::PubUseInternal),
            );
        }

        self.add_or_append_file(
            world_directory(),
            (
                major_version_to_string(version).to_string(),
                SubmoduleLocation::PubMod,
            ),
        );

        let file_dir = world_directory().join(major_version_to_string(version));
        self.add_or_append_file(
            file_dir.clone(),
            (get_module_name(name), SubmoduleLocation::PubUseInternal),
        );
        self.add_or_append_file(
            file_dir.clone(),
            ("opcodes".to_string(), SubmoduleLocation::PubMod),
        );

        self.add_or_append_file(
            file_dir,
            (
                format!("crate::helper::{}", major_version_to_string(version)),
                SubmoduleLocation::PubUseOnly,
            ),
        );
    }

    pub(crate) fn add_login_file(&mut self, name: &str, version: &LoginVersion) {
        let e = (version.as_module_case(), SubmoduleLocation::PubMod);

        let top_level_dir = login_directory();

        if let Some(v) = self.v.iter_mut().find(|a| a.name == top_level_dir) {
            v.submodules.push(e);
        } else {
            self.v.push(ModFile {
                name: top_level_dir,
                submodules: vec![e],
            })
        }

        let module_level_dir = path_utils::get_login_version_file_path(version);

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

    pub(crate) fn write_shared_contents_to_file(
        &mut self,
        name: &str,
        tags: &Tags,
        base_s: &str,
        versions: &[Version],
    ) {
        let versions: Vec<WorldVersion> = versions.iter().map(|a| a.as_world()).collect();

        let path = if tags.is_in_base() {
            path_utils::get_base_shared_filepath(name, &versions)
        } else {
            path_utils::get_world_shared_filepath(name, &versions)
        };

        self.add_world_shared_file(name, &versions, tags);
        create_and_overwrite_if_not_same_contents(base_s, Path::new(&path));

        self.already_existing_files
            .insert(path.canonicalize().unwrap(), true);
    }

    pub(crate) fn write_shared_import_to_file(
        &mut self,
        name: &str,
        tags: &Tags,
        world_s: &str,
        base_s: &str,
        version: &Version,
    ) {
        let version = &version.as_world();
        let base_path = path_utils::get_base_filepath(name, version);
        let world_path = path_utils::get_world_filepath(name, version);

        self.add_world_file(name, version, tags);
        create_and_overwrite_if_not_same_contents(world_s, &world_path);
        create_and_overwrite_if_not_same_contents(base_s, Path::new(&base_path));

        self.already_existing_files
            .insert(base_path.canonicalize().unwrap(), true);
        self.already_existing_files
            .insert(world_path.canonicalize().unwrap(), true);
    }

    pub(crate) fn write_base_contents_to_file(
        &mut self,
        name: &str,
        tags: &Tags,
        base_s: &str,
        world_s: &str,
        version: Version,
    ) {
        match &version {
            Version::Login(_) => unimplemented!(),
            Version::World(version) => {
                let world_path = path_utils::get_world_filepath(name, version);
                let base_path = path_utils::get_base_filepath(name, version);

                self.add_world_file(name, version, tags);

                create_and_overwrite_if_not_same_contents(world_s, &world_path);
                create_and_overwrite_if_not_same_contents(base_s, Path::new(&base_path));

                self.already_existing_files
                    .insert(world_path.canonicalize().unwrap(), true);
                self.already_existing_files
                    .insert(base_path.canonicalize().unwrap(), true);
            }
        }
    }

    pub(crate) fn write_contents_to_file(
        &mut self,
        name: &str,
        tags: &Tags,
        s: &str,
        version: Version,
    ) {
        match &version {
            Version::Login(v) => {
                let path = path_utils::get_login_filepath(name, v);

                self.add_login_file(name, v);

                create_and_overwrite_if_not_same_contents(s, Path::new(&path));

                self.already_existing_files
                    .insert(path.canonicalize().unwrap(), true);
            }
            Version::World(version) => {
                let path = path_utils::get_world_filepath(name, version);

                self.add_world_file(name, version, tags);

                create_and_overwrite_if_not_same_contents(s, &path);

                self.already_existing_files
                    .insert(path.canonicalize().unwrap(), true);
            }
        }
    }
}

pub(crate) fn major_version_to_string(v: &WorldVersion) -> &'static str {
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

pub(crate) fn get_world_version_path(version: &WorldVersion) -> String {
    format!("crate::world::{}", major_version_to_string(version))
}

pub(crate) fn get_login_logon_version_path(version: &LoginVersion) -> String {
    format!("crate::logon::{}", version.as_module_case())
}

pub(crate) fn get_world_shared_path(ty_name: &str, versions: &[WorldVersion]) -> String {
    format!(
        "crate::world::shared::{}",
        get_shared_module_name(ty_name, versions)
    )
}

pub(crate) fn get_import_path(version: Version) -> String {
    match &version {
        Version::Login(f) => get_login_logon_version_path(f),
        Version::World(f) => get_world_version_path(f),
    }
}

pub(crate) fn append_string_to_file(s: &str, filename: &Path) {
    let f = std::fs::OpenOptions::new().append(true).open(filename);
    let mut f = match f {
        Ok(f) => f,
        Err(e) => panic!(
            "Unable to append string to file: '{:?}' with error '{:?}'",
            filename, e
        ),
    };

    f.write_all(s.as_bytes()).unwrap();
}

pub(crate) fn write_string_to_file(s: &str, filename: &Path) {
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

pub(crate) fn overwrite_if_not_same_contents(s: &str, filename: &Path) {
    let f = read_to_string(filename).unwrap();
    if f != s {
        write_string_to_file(s, filename);
    }
}

pub(crate) fn create_and_overwrite_if_not_same_contents(s: &str, filename: &Path) {
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

pub(crate) fn get_module_name(e: &str) -> String {
    e.to_snake_case()
}

pub(crate) fn get_shared_module_name(e: &str, versions: &[WorldVersion]) -> String {
    let mut s = e.to_snake_case();

    let mut versions = versions.to_vec();
    versions.sort();

    for v in versions {
        s += &v.as_module_case();
    }

    s
}
