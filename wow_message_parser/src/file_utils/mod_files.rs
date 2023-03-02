use crate::file_utils::get_login_logon_version_path;
use crate::parser::types::tags::ObjectTags;
use crate::parser::types::version::{LoginVersion, MajorWorldVersion, Version};
use crate::path_utils::{
    base_directory, get_filepath, get_login_filepath, get_login_version_file_path, login_directory,
    world_directory,
};
use crate::rust_printer::Writer;
use crate::{file_utils, path_utils};
use std::collections::BTreeMap;
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(crate) struct NewModFiles {
    already_existing_files: BTreeMap<PathBuf, bool>,
    login_modules: BTreeMap<LoginVersion, Writer>,
}

impl NewModFiles {
    pub(crate) fn new() -> Self {
        let mut already_existing_files = BTreeMap::new();

        for dir in [
            login_directory(), /* world_directory(), base_directory() */
        ] {
            for file in WalkDir::new(dir).into_iter().filter_map(|a| a.ok()) {
                if !file.file_type().is_file() {
                    continue;
                }

                let filename = file.path().canonicalize().unwrap();

                let value = if let Some(p) = filename.file_name() {
                    p.to_string_lossy() == "opcodes.rs"
                } else {
                    false
                };
                already_existing_files.insert(filename, value);
            }
        }

        Self {
            already_existing_files,
            login_modules: Default::default(),
        }
    }

    fn write_file(&mut self, path: &Path, text: &str) {
        file_utils::create_and_overwrite_if_not_same_contents(text, &path);
        self.already_existing_files
            .insert(path.canonicalize().unwrap(), true);
    }

    pub(crate) fn write_modules_and_remove_unwritten_files(&mut self) {
        self.write_login_modules();

        self.remove_unwritten_files();
    }

    fn remove_unwritten_files(&mut self) {
        for (filename, written) in &self.already_existing_files {
            if !written {
                remove_file(&filename).unwrap();
            }
        }
    }
}

// Login
impl NewModFiles {
    pub(crate) fn add_login_module(
        &mut self,
        name: &str,
        mut versions: impl Iterator<Item = LoginVersion>,
        text: &str,
    ) {
        let module_name = file_utils::get_module_name(name);
        let first_module_text = format!("pub(crate) mod {module_name};\npub use {module_name}::*;");

        // Login crate does not have feature gates, so we can just reexport the first version
        let first_version = versions.next().unwrap();

        let path = get_login_filepath(name, &first_version);
        self.write_file(&path, text);

        self.insert_into_login_module(first_version, &first_module_text);

        for version in versions {
            let version_path = get_login_logon_version_path(&first_version);
            let text = format!("pub use {version_path}::{module_name}::*;");

            self.insert_into_login_module(version, &text);
        }
    }

    fn insert_into_login_module(&mut self, version: LoginVersion, text: &str) {
        if let Some(s) = self.login_modules.get_mut(&version) {
            s.wln(text);
        } else {
            let mut s = Writer::no_import();

            // All does not have opcodes, they are part of other opcodes
            if version != LoginVersion::All {
                s.wln("pub mod opcodes;");
                s.newline();
            }

            s.wln(text);

            self.login_modules.insert(version, s);
        }
    }

    fn write_login_modules(&mut self) {
        let mut logon_module = Writer::no_import();

        for (version, text) in self.login_modules.clone() {
            let path = get_login_version_file_path(&version).join("mod.rs");
            self.write_file(&path, text.inner());

            logon_module.wln(format!("pub mod {};", version.as_module_case()));
        }

        let logon_module_path = login_directory().join("mod.rs");
        self.write_file(&logon_module_path, logon_module.inner());
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum SubmoduleLocation {
    PubUseInternal,
    PubMod,
    PubCrateMod,
    PubUseOnly,
    PubUseAndCfg(String),
    PubModAndCfg(String),
    SpecificLine,
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
    pub(crate) fn new() -> Self {
        let mut already_existing_files = BTreeMap::new();

        for dir in [world_directory(), base_directory()] {
            for file in WalkDir::new(dir).into_iter().filter_map(|a| a.ok()) {
                if !file.file_type().is_file() {
                    continue;
                }

                let filename = file.path().canonicalize().unwrap();

                let value = if let Some(p) = filename.file_name() {
                    p.to_string_lossy() == "opcodes.rs"
                } else {
                    false
                };
                already_existing_files.insert(filename, value);
            }
        }

        Self {
            v: vec![],
            already_existing_files,
        }
    }
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
            use std::fmt::Write;

            let mut s = String::new();
            for (i, location) in &m.submodules {
                match location {
                    SubmoduleLocation::PubUseInternal => {
                        writeln!(s, "pub(crate) mod {i};").unwrap();
                        writeln!(s, "pub use {i}::*;").unwrap();
                    }
                    SubmoduleLocation::PubMod => {
                        if ["vanilla", "wrath", "tbc"].contains(&i.as_str()) {
                            writeln!(s, "#[cfg(feature = \"{i}\")]").unwrap();
                        }
                        writeln!(s, "pub mod {i};").unwrap();
                    }
                    SubmoduleLocation::PubUseOnly => {
                        writeln!(s, "pub use {i}::*;").unwrap();
                    }
                    SubmoduleLocation::PubUseAndCfg(cfg) => {
                        writeln!(s, "#[cfg(feature = \"{cfg}\")]").unwrap();
                        writeln!(s, "pub use {i}::*;").unwrap();
                    }
                    SubmoduleLocation::PubCrateMod => {
                        writeln!(s, "pub(crate) mod {i};").unwrap();
                    }
                    SubmoduleLocation::SpecificLine => {
                        write!(s, "{i}").unwrap();
                    }
                    SubmoduleLocation::PubModAndCfg(cfg) => {
                        writeln!(s, "#[cfg({cfg})]").unwrap();
                        writeln!(s, "pub mod {i};").unwrap();
                    }
                }
            }
            let filename = m.name.clone().join("mod.rs");

            file_utils::create_and_overwrite_if_not_same_contents(&s, &filename);
            self.already_existing_files
                .insert(filename.canonicalize().unwrap(), true);
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

    pub(crate) fn add_world_shared_file(&mut self, shared_module_name: String, tags: &ObjectTags) {
        let base_path = if tags.is_in_base() {
            base_directory()
        } else {
            world_directory()
        };

        let submodule_location = if tags.is_in_base() {
            SubmoduleLocation::PubMod
        } else {
            SubmoduleLocation::PubCrateMod
        };
        self.add_or_append_file(
            base_path.clone(),
            ("shared".to_string(), submodule_location),
        );

        self.add_or_append_file(
            base_path.join("shared"),
            (
                shared_module_name,
                SubmoduleLocation::PubModAndCfg(tags.get_cfg_for_versions()),
            ),
        );
    }

    pub(crate) fn add_everything_but_world_file(
        &mut self,
        name: &str,
        version: &MajorWorldVersion,
        tags: &ObjectTags,
        file_dir: PathBuf,
    ) {
        if tags.is_in_base() {
            self.add_or_append_file(
                base_directory(),
                (
                    file_utils::major_version_to_string(version).to_string(),
                    SubmoduleLocation::PubMod,
                ),
            );

            self.add_or_append_file(
                base_directory().join(file_utils::major_version_to_string(version)),
                (
                    file_utils::get_module_name(name),
                    SubmoduleLocation::PubUseInternal,
                ),
            );

            self.add_or_append_file(
                base_directory().join(file_utils::major_version_to_string(version)),
                (
                    format!(
                        "crate::manual::{}",
                        file_utils::major_version_to_string(version)
                    ),
                    SubmoduleLocation::PubUseOnly,
                ),
            );

            if tags.shared() {
                self.add_or_append_file(
                    base_directory().join("shared"),
                    (
                        "crate::manual::shared".to_string(),
                        SubmoduleLocation::PubUseOnly,
                    ),
                );
            }

            self.add_or_append_file(
                base_directory().join(file_utils::major_version_to_string(version)),
                (
                    format!(
                        "crate::extended::{}",
                        file_utils::major_version_to_string(version)
                    ),
                    SubmoduleLocation::PubUseAndCfg("extended".to_string()),
                ),
            );
        }

        self.add_or_append_file(
            world_directory(),
            (
                file_utils::major_version_to_string(version).to_string(),
                SubmoduleLocation::PubMod,
            ),
        );

        self.add_or_append_file(
            file_dir.clone(),
            ("opcodes".to_string(), SubmoduleLocation::PubMod),
        );

        self.add_or_append_file(
            world_directory().join(file_utils::major_version_to_string(version)),
            (
                format!(
                    "crate::manual::{}",
                    file_utils::major_version_to_string(version)
                ),
                SubmoduleLocation::PubUseOnly,
            ),
        );

        if tags.shared() {
            self.add_or_append_file(
                world_directory().join("shared"),
                (
                    "crate::manual::shared".to_string(),
                    SubmoduleLocation::PubUseOnly,
                ),
            );
        }

        self.add_or_append_file(
            file_dir,
            (
                format!(
                    "crate::helper::{}",
                    file_utils::major_version_to_string(version)
                ),
                SubmoduleLocation::PubUseOnly,
            ),
        );
    }

    pub(crate) fn add_world_file(
        &mut self,
        name: &str,
        version: &MajorWorldVersion,
        tags: &ObjectTags,
    ) {
        let file_dir = world_directory().join(file_utils::major_version_to_string(version));

        self.add_everything_but_world_file(name, version, tags, file_dir.clone());

        self.add_or_append_file(
            file_dir,
            (
                file_utils::get_module_name(name),
                SubmoduleLocation::PubUseInternal,
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

        let e = (
            file_utils::get_module_name(name),
            SubmoduleLocation::PubUseInternal,
        );

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
        tags: &ObjectTags,
        base_s: &str,
    ) {
        let shared_module_name = tags.shared_module_name(name);

        let path = if tags.is_in_base() {
            path_utils::get_base_shared_filepath(&shared_module_name)
        } else {
            path_utils::get_world_shared_filepath(&shared_module_name)
        };

        self.add_world_shared_file(shared_module_name, tags);
        file_utils::create_and_overwrite_if_not_same_contents(base_s, Path::new(&path));

        self.already_existing_files
            .insert(path.canonicalize().unwrap(), true);
    }

    pub(crate) fn write_shared_import_to_file(
        &mut self,
        name: &str,
        tags: &ObjectTags,
        world_s: &str,
        base_s: &str,
        version: &Version,
    ) {
        let version = &version.as_major_world();
        let base_path = path_utils::get_base_filepath(name, version);
        let world_path = path_utils::get_world_filepath(name, version);

        let file_dir = world_directory().join(file_utils::major_version_to_string(version));

        self.add_everything_but_world_file(name, version, tags, file_dir.clone());

        self.write_specific_line_to_file(world_s.to_string(), file_dir);

        file_utils::create_and_overwrite_if_not_same_contents(world_s, &world_path);
        file_utils::create_and_overwrite_if_not_same_contents(base_s, Path::new(&base_path));

        self.already_existing_files
            .insert(base_path.canonicalize().unwrap(), true);
        self.already_existing_files
            .insert(world_path.canonicalize().unwrap(), true);
    }

    pub(crate) fn write_specific_line_to_file(&mut self, line: String, file_dir: PathBuf) {
        self.add_or_append_file(file_dir.clone(), (line, SubmoduleLocation::SpecificLine));
        self.already_existing_files
            .insert(file_dir.canonicalize().unwrap(), true);
    }

    pub(crate) fn write_base_contents_to_file(
        &mut self,
        name: &str,
        tags: &ObjectTags,
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

                file_utils::create_and_overwrite_if_not_same_contents(world_s, &world_path);
                file_utils::create_and_overwrite_if_not_same_contents(
                    base_s,
                    Path::new(&base_path),
                );

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
        tags: &ObjectTags,
        s: &str,
        version: Version,
    ) {
        let path = get_filepath(name, &version);

        match &version {
            Version::Login(v) => {
                self.add_login_file(name, v);

                file_utils::create_and_overwrite_if_not_same_contents(s, Path::new(&path));

                self.already_existing_files
                    .insert(path.canonicalize().unwrap(), true);
            }
            Version::World(version) => {
                self.add_world_file(name, version, tags);

                file_utils::create_and_overwrite_if_not_same_contents(s, &path);

                self.already_existing_files
                    .insert(path.canonicalize().unwrap(), true);
            }
        }
    }
}
