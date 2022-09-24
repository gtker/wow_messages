use crate::file_utils;
use crate::parser::types::tags::{LoginVersion, WorldVersion};
use crate::rust_printer::MajorWorldVersion;
use std::path::{Path, PathBuf};

fn workspace_directory() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(p.pop());
    p
}

pub fn wowm_directory(directory: &str) -> PathBuf {
    workspace_directory()
        .join("wow_message_parser")
        .join("wowm")
        .join(directory)
}

pub fn login_directory() -> PathBuf {
    workspace_directory()
        .join("wow_login_messages")
        .join("src")
        .join("logon")
}

pub fn world_directory() -> PathBuf {
    workspace_directory()
        .join("wow_world_messages")
        .join("src")
        .join("world")
}

pub fn base_directory() -> PathBuf {
    workspace_directory()
        .join("wow_world_base")
        .join("src")
        .join("inner")
}

fn update_mask_location(version: MajorWorldVersion) -> PathBuf {
    workspace_directory()
        .join("wow_world_messages")
        .join("src")
        .join("helper")
        .join(version.module_name())
        .join("update_mask")
        .join("impls.rs")
}

pub fn vanilla_update_mask_location() -> PathBuf {
    update_mask_location(MajorWorldVersion::Vanilla)
}

pub fn tbc_update_mask_location() -> PathBuf {
    update_mask_location(MajorWorldVersion::BurningCrusade)
}

pub fn wrath_update_mask_location() -> PathBuf {
    update_mask_location(MajorWorldVersion::Wrath)
}

pub fn path_to_fileinfo(path: &Path) -> String {
    let ws = workspace_directory().canonicalize().unwrap();
    let path = path.canonicalize().unwrap();

    path.strip_prefix(ws).unwrap().to_str().unwrap().into()
}

pub fn get_world_version_file_path(version: &WorldVersion) -> PathBuf {
    world_directory().join(file_utils::major_version_to_string(version))
}

pub fn get_login_version_file_path(version: &LoginVersion) -> PathBuf {
    let p = login_directory();
    match version {
        LoginVersion::Specific(v) => p.join(format!("version_{}", v)),
        LoginVersion::All => p.join("all"),
    }
}

pub fn get_base_filepath(object_name: &str, version: &WorldVersion) -> PathBuf {
    base_directory()
        .join(file_utils::major_version_to_string(version))
        .join(format!("{}.rs", file_utils::get_module_name(object_name)))
}

pub fn get_base_shared_filepath(object_name: &str, versions: &[WorldVersion]) -> PathBuf {
    base_directory().join("shared").join(format!(
        "{}.rs",
        file_utils::get_shared_module_name(object_name, versions)
    ))
}

pub fn get_world_shared_filepath(object_name: &str, versions: &[WorldVersion]) -> PathBuf {
    world_directory().join("shared").join(format!(
        "{}.rs",
        file_utils::get_shared_module_name(object_name, versions)
    ))
}

pub fn get_world_filepath(object_name: &str, version: &WorldVersion) -> PathBuf {
    get_world_version_file_path(version)
        .join(format!("{}.rs", file_utils::get_module_name(object_name)))
}

pub fn get_login_filepath(object_name: &str, version: &LoginVersion) -> PathBuf {
    get_login_version_file_path(version)
        .join(format!("{}.rs", file_utils::get_module_name(object_name)))
}
