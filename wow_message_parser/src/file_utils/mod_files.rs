use crate::file_utils;
use crate::file_utils::{get_login_logon_version_path, get_module_name, get_shared_module_name};
use crate::parser::types::version::{LoginVersion, MajorWorldVersion};
use crate::path_utils::{
    base_directory, get_base_filepath, get_base_shared_filepath, get_login_filepath,
    get_login_version_file_path, get_world_filepath, get_world_shared_filepath, login_directory,
    world_directory,
};
use crate::rust_printer::Writer;
use std::collections::BTreeMap;
use std::fs::remove_file;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(crate) struct ModFiles {
    already_existing_files: BTreeMap<PathBuf, bool>,
    login_modules: BTreeMap<LoginVersion, Writer>,
    base_modules: BTreeMap<MajorWorldVersion, Writer>,
    world_modules: BTreeMap<MajorWorldVersion, Writer>,
    shared_base_modules: Writer,
    shared_world_modules: Writer,
}

impl ModFiles {
    pub(crate) fn new() -> Self {
        let mut already_existing_files = BTreeMap::new();

        for dir in [login_directory(), world_directory(), base_directory()] {
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

        let mut shared_base_modules = Writer::new();
        shared_base_modules.wln("pub use crate::manual::shared::*;");

        let mut shared_world_modules = Writer::new();
        shared_world_modules.wln("pub use crate::manual::shared::*;");

        Self {
            already_existing_files,
            login_modules: Default::default(),
            base_modules: Default::default(),
            world_modules: Default::default(),
            shared_base_modules,
            shared_world_modules,
        }
    }

    fn write_file(&mut self, path: &Path, text: &str) {
        file_utils::create_and_overwrite_if_not_same_contents(text, path);
        self.already_existing_files
            .insert(path.canonicalize().unwrap(), true);
    }

    pub(crate) fn write_modules_and_remove_unwritten_files(&mut self) {
        self.write_login_modules();
        self.write_base_modules();
        self.write_world_modules();

        self.remove_unwritten_files();
    }

    fn remove_unwritten_files(&mut self) {
        for (filename, written) in &self.already_existing_files {
            if !written {
                remove_file(filename).unwrap();
            }
        }
    }
}

impl ModFiles {
    pub(crate) fn add_base_module(
        &mut self,
        name: &str,
        versions: &[MajorWorldVersion],
        text: &str,
    ) {
        match versions {
            [] => panic!(),
            [version] => {
                let path = get_base_filepath(name, version);
                self.write_file(&path, text);

                let module_name = get_module_name(name);
                let module_text =
                    format!("pub(crate) mod {module_name};\npub use {module_name}::*;");
                self.insert_into_base_module(*version, &module_text);
            }
            versions => {
                let module_name = get_shared_module_name(name, versions);
                let path = get_base_shared_filepath(&module_name);
                self.write_file(&path, text);

                let module_text = format!("pub mod {module_name};");
                self.insert_into_shared_base_module(versions, &module_text);

                for version in versions {
                    let s = format!("pub use crate::shared::{module_name}::*;");
                    self.insert_into_base_module(*version, &s);
                }

                for version in versions {
                    let s = format!("pub use wow_world_base::shared::{module_name}::*;");
                    self.insert_into_world_module(*version, &s);
                }
            }
        }
    }

    fn insert_into_base_module(&mut self, version: MajorWorldVersion, text: &str) {
        if let Some(s) = self.base_modules.get_mut(&version) {
            s.wln(text);
        } else {
            let mut s = Writer::new();

            let module = version.module_name();
            s.wln(format!("pub use crate::manual::{module}::*;"));
            s.wln("#[cfg(feature = \"extended\")]");
            s.wln(format!("pub use crate::extended::{module}::*;"));
            s.wln("pub use crate::manual::shared::datetime_vanilla_tbc_wrath::*;");
            s.wln("pub use crate::manual::shared::gold_vanilla_tbc_wrath::*;");
            s.wln("pub use crate::manual::shared::guid_vanilla_tbc_wrath::*;");
            s.wln("pub use crate::manual::shared::level_vanilla_tbc_wrath::*;");
            s.wln("pub use crate::manual::shared::player_gender_vanilla_tbc_wrath::*;");
            match version {
                MajorWorldVersion::Vanilla => {}
                MajorWorldVersion::BurningCrusade | MajorWorldVersion::Wrath => {
                    s.wln("pub use crate::manual::shared::player_race_tbc_wrath::*;");
                }
            }
            s.wln("pub use crate::manual::shared::skill_category_vanilla_tbc_wrath::*;");
            s.newline();

            s.wln(text);

            self.base_modules.insert(version, s);
        }
    }

    fn insert_into_shared_base_module(&mut self, versions: &[MajorWorldVersion], text: &str) {
        use std::fmt::Write;

        let mut cfgs = "#[cfg(any(".to_string();
        for (i, version) in versions.iter().enumerate() {
            if i != 0 {
                write!(cfgs, ", ").unwrap();
            }

            write!(cfgs, "feature = \"{}\"", version.module_name()).unwrap();
        }
        write!(cfgs, "))]").unwrap();

        self.shared_base_modules.wln(cfgs);
        self.shared_base_modules.wln(text);
    }

    fn write_base_modules(&mut self) {
        let mut base_module = Writer::new();
        base_module.wln("pub mod shared;");

        for (version, text) in self.base_modules.clone() {
            let path = base_directory().join(version.module_name()).join("mod.rs");
            self.write_file(&path, text.inner());

            let module = version.module_name();
            base_module.wln(format!("#[cfg(feature = \"{module}\")]"));
            base_module.wln(format!("pub mod {module};"));
        }

        let base_module_path = base_directory().join("mod.rs");
        self.write_file(&base_module_path, base_module.inner());

        let base_shared_module_path = base_directory().join("shared").join("mod.rs");
        let s = self.shared_base_modules.clone();
        self.write_file(&base_shared_module_path, s.inner());
    }
}

impl ModFiles {
    pub(crate) fn add_world_module(
        &mut self,
        name: &str,
        versions: &[MajorWorldVersion],
        text: &str,
    ) {
        match versions {
            [] => panic!(),
            [version] => {
                let path = get_world_filepath(name, version);
                self.write_file(&path, text);

                let module_name = get_module_name(name);
                let module_text =
                    format!("pub(crate) mod {module_name};\npub use {module_name}::*;");
                self.insert_into_world_module(*version, &module_text);
            }
            versions => {
                let module_name = get_shared_module_name(name, versions);
                let path = get_world_shared_filepath(&module_name);
                self.write_file(&path, text);

                let module_text = format!("pub mod {module_name};");
                self.insert_into_shared_world_module(versions, &module_text);

                for version in versions {
                    let s = format!("pub use crate::shared::{module_name}::*;");
                    self.insert_into_world_module(*version, &s);
                }
            }
        }
    }

    fn insert_into_shared_world_module(&mut self, versions: &[MajorWorldVersion], text: &str) {
        use std::fmt::Write;

        let mut cfgs = "#[cfg(any(".to_string();
        for (i, version) in versions.iter().enumerate() {
            if i != 0 {
                write!(cfgs, ", ").unwrap();
            }

            write!(cfgs, "feature = \"{}\"", version.module_name()).unwrap();
        }
        write!(cfgs, "))]").unwrap();

        self.shared_world_modules.wln(cfgs);
        self.shared_world_modules.wln(text);
    }

    fn insert_into_world_module(&mut self, version: MajorWorldVersion, text: &str) {
        if let Some(s) = self.world_modules.get_mut(&version) {
            s.wln(text);
        } else {
            let mut s = Writer::new();

            let module = version.module_name();
            s.wln("pub mod opcodes;");
            s.wln(format!("pub use wow_world_base::{module}::*;"));
            s.wln(format!("pub use crate::manual::{module}::*;"));
            s.wln(format!("pub use crate::helper::{module}::*;"));
            s.wln(format!("pub use crate::traits::{module}::*;"));
            s.newline();

            s.wln(text);

            self.world_modules.insert(version, s);
        }
    }

    fn write_world_modules(&mut self) {
        let mut world_module = Writer::new();
        world_module.wln("pub mod shared;");

        for (version, text) in self.world_modules.clone() {
            let path = world_directory().join(version.module_name()).join("mod.rs");
            self.write_file(&path, text.inner());

            let module = version.module_name();
            world_module.wln(format!("#[cfg(feature = \"{module}\")]"));
            world_module.wln(format!("pub mod {module};"));
        }

        let world_module_path = world_directory().join("mod.rs");
        self.write_file(&world_module_path, world_module.inner());

        let world_shared_module_path = world_directory().join("shared").join("mod.rs");
        let s = self.shared_world_modules.clone();
        self.write_file(&world_shared_module_path, s.inner());
    }
}

// Login
impl ModFiles {
    pub(crate) fn add_login_module(
        &mut self,
        name: &str,
        mut versions: impl Iterator<Item = LoginVersion>,
        text: &str,
    ) {
        let module_name = get_module_name(name);
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
            let mut s = Writer::new();

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
        let mut logon_module = Writer::new();

        for (version, text) in self.login_modules.clone() {
            let path = get_login_version_file_path(&version).join("mod.rs");
            self.write_file(&path, text.inner());

            logon_module.wln(format!("pub mod {};", version.as_module_case()));
        }

        let logon_module_path = login_directory().join("mod.rs");
        self.write_file(&logon_module_path, logon_module.inner());
    }
}
