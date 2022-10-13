use crate::path_utils::path_to_fileinfo;
use crate::GITHUB_REPO_URL;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct FileInfo {
    file_name: String,
    path: PathBuf,
    start_position: usize,
    end_position: usize,
}

impl FileInfo {
    pub(crate) fn new(path: PathBuf, start_position: usize, end_position: usize) -> Self {
        let file_name = path_to_fileinfo(&path);

        Self {
            file_name,
            path,
            start_position,
            end_position,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.file_name
    }

    pub(crate) fn start_line(&self) -> usize {
        self.start_position
    }

    pub(crate) fn original_file_github_link(&self) -> String {
        format!(
            "[`{name}:{line}`]({github_repo}/tree/main/{name}#L{line})",
            name = self.name(),
            line = self.start_line(),
            github_repo = GITHUB_REPO_URL,
        )
    }
}
