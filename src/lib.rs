use derive_more::Display;
use std::{env, path::{Path, PathBuf}};

pub fn file_path<P: AsRef<Path>>(file_name: P) -> Result<PathBuf, NoFilePathError> {
    let config_dir_file = dirs::config_dir().map(|dir| dir.join("manada").join(&file_name));
    let etc_file = env::var("MANADA_CONFIG")
        .map_or(PathBuf::from("/etc/manada"), PathBuf::from)
        .join(&file_name);

    match (config_dir_file, etc_file) {
        (Some(file), _) if file.exists() => Ok(file),
        (_, file) if file.exists() => Ok(file),
        (Some(home_file), etc_file) => Err(NoFilePathError::NeitherHomeNorEtc(home_file, etc_file)),
        // Very unlikly but it can happen, if there is no home config dir
        (None, etc_file) => Err(NoFilePathError::NoEtc(etc_file)),
    }
}

#[derive(Display)]
pub enum NoFilePathError {
    #[display("Neither {} nor {} exist", _0.display(), _1.display())]
    NeitherHomeNorEtc(PathBuf, PathBuf),
    #[display("{} doesn't exist", _0.display())]
    NoEtc(PathBuf),
}
