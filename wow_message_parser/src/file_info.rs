use crate::GITHUB_REPO_URL;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct FileInfo {
    file_name: String,
    start_position: (usize, usize),
}

impl FileInfo {
    pub(crate) fn new(file_name: &str, start_position: (usize, usize)) -> Self {
        Self {
            file_name: file_name.to_string(),
            start_position,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.file_name
    }

    pub(crate) fn start_line(&self) -> usize {
        self.start_position.0
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
