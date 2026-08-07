use crate::{config_dir::ConfigDir, prelude::*, theme::ThemeType};
use gpui::Global;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct General {
    pub projects_dir: String,
    pub delete_project_folder: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Theme {
    pub theme: ThemeType,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub general: General,
    pub theme: Theme,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config = Self::read_config_file()?;
        config.validate()?;

        Ok(config)
    }

    fn read_config_file() -> Result<Self> {
        let default_config_contents = toml::to_string_pretty(&Self::default())
            .map_err(|err| Error::ParseConfig(err.to_string()))?;

        let config_path =
            ConfigDir::ConfigFile.get_file(Some(&default_config_contents))?;

        let config_string = fs::read_to_string(&config_path)
            .map_err(|err| Error::ParseConfig(err.to_string()))?;

        toml::from_str::<Self>(&config_string)
            .map_err(|err| Error::ParseConfig(err.to_string()))
    }

    fn validate(&self) -> Result<()> {
        if self.general.projects_dir.is_empty() {
            return Err(Error::ValidateConfig(String::from(
                "project_dir field is empty!",
            )));
        }

        if !PathBuf::from(&self.general.projects_dir).is_dir() {
            return Err(Error::ValidateConfig(String::from(
                "project_dir is not a directory!",
            )));
        }

        Ok(())
    }
}

impl Global for Config {}

// #[expect(clippy::unwrap_used)]
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;
//
//     fn create_mock_config(projects_path: &str) -> Config {
//         Config {
//             general: General {
//                 projects_dir: projects_path.to_string(),
//                 ..Default::default()
//             },
//             ..Default::default()
//         }
//     }
//
//     #[test]
//     fn test_config_load_creates_and_parses_default_file() {
//         let temp_dir = tempfile::tempdir().unwrap();
//
//         temp_env::with_var("PROJMAN_CONFIG_DIR", Some(temp_dir.path()), || {
//             let result = Config::load();
//
//             assert!(result.is_err());
//             if let Err(Error::ValidateConfig(msg)) = result {
//                 assert!(msg.contains("project_dir field is empty"));
//             } else {
//                 panic!("Expected ValidateConfig error due to empty default projects_dir");
//             }
//         });
//     }
//
//     #[test]
//     fn test_validation_passes_for_valid_directory() {
//         let temp_dir = tempfile::tempdir().unwrap();
//         let valid_dir = temp_dir.path().join("my_projects");
//
//         fs::create_dir_all(&valid_dir).unwrap();
//
//         let config = create_mock_config(valid_dir.to_str().unwrap());
//         config.validate().unwrap();
//     }
//
//     #[test]
//     fn test_validation_fails_when_projects_dir_empty() {
//         let config = create_mock_config("");
//
//         let result = config.validate();
//         assert!(result.is_err());
//
//         if let Err(Error::ValidateConfig(msg)) = result {
//             assert!(msg.contains("project_dir field is empty"));
//         } else {
//             panic!("Expected ValidateConfig error");
//         }
//     }
//
//     #[test]
//     fn test_validation_fails_when_path_is_not_a_directory() {
//         let temp_dir = tempfile::tempdir().unwrap();
//
//         let file_path = temp_dir.path().join("not_a_directory");
//         fs::write(&file_path, "hello").unwrap();
//
//         let config = create_mock_config(file_path.to_str().unwrap());
//
//         let result = config.validate();
//         assert!(result.is_err());
//
//         if let Err(Error::ValidateConfig(msg)) = result {
//             assert!(msg.contains("project_dir is not a directory"));
//         } else {
//             panic!("Expected ValidateConfig error");
//         }
//     }
// }
