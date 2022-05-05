use crate::GITHUB_REPO_URL;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FileInfo {
    file_name: String,
    start_position: (usize, usize),
}

impl FileInfo {
    pub fn new(file_name: &str, start_position: (usize, usize)) -> Self {
        Self {
            file_name: file_name.to_string(),
            start_position,
        }
    }

    pub fn name(&self) -> &str {
        &self.file_name
    }

    pub fn start_line(&self) -> usize {
        self.start_position.0
    }

    pub fn original_file_github_link(&self) -> String {
        format!(
            "[`{name}:{line}`]({github_repo}/tree/main/{name}#L{line})",
            name = self.name(),
            line = self.start_line(),
            github_repo = GITHUB_REPO_URL,
        )
    }
}
