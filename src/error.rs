use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MazeError {
    #[error("The project name `{0}` already exists in the current directory")]
    ProjectExists(PathBuf),

    #[error("The CMakeLists.txt file does not exist in the project directory")]
    CMakeFileNotFound,
    #[error("The class `{0}` already exists in the project")]
    ClassExists(String),
}
