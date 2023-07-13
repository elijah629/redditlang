use std::{fs, path::Path};

use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfiguration {
    pub name: String,
    pub version: Version,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub path: String,
    pub config: ProjectConfiguration,
}

impl Project {
    /// None if there is no project at the specified path, or if the configuration is invalid
    pub fn from_path(path: &Path) -> Option<Self> {
        let configuration_path = path.join("walter.yml");
        let config = fs::read_to_string(configuration_path).ok()?;
        let config: ProjectConfiguration = serde_yaml::from_str(&config).ok()?;

        Some(Project {
            path: path.to_str().unwrap().to_string(),
            config,
        })
    }
}
