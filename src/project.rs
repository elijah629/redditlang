use std::{
    fs,
    path::{Path, PathBuf},
};

use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::utils::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfiguration {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub path: PathBuf,
    pub config: ProjectConfiguration,
}

impl Project {
    /// None if there is no project at the specified path, or if the configuration is invalid
    pub fn from_path(path: &Path) -> Result<Self> {
        let configuration_path = path.join("walter.yml");
        let config = fs::read_to_string(configuration_path)?;
        let config: ProjectConfiguration = serde_yaml::from_str(&config)?;

        Ok(Project {
            path: path.to_path_buf(),
            config,
        })
    }

    pub fn from_current() -> Result<Self> {
        Self::from_path(std::env::current_dir()?.as_path()).map_err(|_| {
            format!("No {} found in the current directory", "walter.yml".bold()).into()
        })
    }
}
