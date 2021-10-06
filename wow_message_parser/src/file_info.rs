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
}
