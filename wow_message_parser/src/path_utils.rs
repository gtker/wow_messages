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
