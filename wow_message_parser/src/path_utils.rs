use crate::file_utils::{BASE_DIR, WORLD_DIR};
use crate::parser::types::tags::{LoginVersion, WorldVersion};
use crate::{file_utils, LOGIN_DIR};
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

pub fn path_to_fileinfo(path: &Path) -> String {
    use path_slash::PathExt;

    path.canonicalize()
        .unwrap()
        .to_slash()
        .unwrap()
        .rsplit_once("wow_messages/")
        .unwrap()
        .1
        .to_string()
}

pub fn get_world_version_file_path(version: &WorldVersion) -> String {
    format!(
        "{}/{}/",
        WORLD_DIR,
        file_utils::major_version_to_string(version)
    )
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

pub fn get_base_filepath(object_name: &str, version: &WorldVersion) -> String {
    let s = format!(
        "{}/{}/",
        BASE_DIR,
        file_utils::major_version_to_string(version)
    );
    s + &file_utils::get_module_name(object_name) + ".rs"
}

pub fn get_base_shared_filepath(object_name: &str, versions: &[WorldVersion]) -> String {
    let s = format!("{}/shared/", BASE_DIR);
    s + &file_utils::get_shared_module_name(object_name, versions) + ".rs"
}

pub fn get_world_shared_filepath(object_name: &str, versions: &[WorldVersion]) -> String {
    let s = format!("{}/shared/", WORLD_DIR);
    s + &file_utils::get_shared_module_name(object_name, versions) + ".rs"
}

pub fn get_world_filepath(object_name: &str, version: &WorldVersion) -> String {
    let s = get_world_version_file_path(version);
    s + &file_utils::get_module_name(object_name) + ".rs"
}

pub fn get_login_filepath(object_name: &str, version: &LoginVersion) -> String {
    let s = get_login_version_file_path(version);
    s + &file_utils::get_module_name(object_name) + ".rs"
}
