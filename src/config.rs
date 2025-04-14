use derive_more::From;
use manada::NoFilePathError;
use serde::Deserialize;
use std::{collections::{HashMap, HashSet}, fs::read_to_string, path::PathBuf};
use toml::de::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    aliases: Option<HashMap<String, HashSet<String>>>,
}

impl Config {
    pub fn try_new(unit_set: &str) -> Result<Self, NewConfigError> {
        let path = manada::file_path(format!("{unit_set}.toml"))?;
        let file_content = match read_to_string(&path) {
            Ok(f) => f,
            Err(error) => return Err(NewConfigError::FileRead { path, error }),
        };
        toml::from_str(&file_content).map_err(|error| NewConfigError::ParseError { path, error })
    }

    pub fn get_full_unit(self, aliased: &str) -> Option<String> {
        let aliases = self.aliases?;

        aliases.into_iter().find_map(|(key, value)| {
            ((key == aliased) || (value.into_iter().any(|s| s == aliased))).then_some(key)
        })
    }
}

#[derive(From)]
pub enum NewConfigError {
    #[from(NoFilePathError)]
    NoConfig,
    #[from]
    FileRead {
        path: PathBuf,
        error: std::io::Error,
    },
    #[from]
    ParseError { path: PathBuf, error: Error },
}
