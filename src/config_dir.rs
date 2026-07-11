use crate::{prelude::*, utils};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    fs,
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub enum ConfigDir {
    ConfigFile,
    Template(String),
    Templates,
    Projects,
}

impl ConfigDir {
    pub fn get_file(self, create_if_missing: Option<&str>) -> Result<PathBuf> {
        let config_path = utils::get_config_path()?;

        fs::create_dir_all(&config_path).map_err(|err| {
            Error::CreateDir(config_path.to_string_lossy().into_owned(), err.to_string())
        })?;

        let path = config_path.join(self.to_string());

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                Error::CreateDir(parent.to_string_lossy().into_owned(), err.to_string())
            })?;
        }

        if path.exists() {
            return Ok(path);
        }

        if let Some(content) = create_if_missing {
            fs::write(&path, content.as_bytes())
                .map_err(|err| Error::CreateDir(self.to_string(), err.to_string()))?;
        }

        Ok(path)
    }
}

impl Display for ConfigDir {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let string = match self {
            Self::ConfigFile => "config.toml",
            Self::Template(name) => &format!("templates/{name}"),
            Self::Templates => "templates",
            Self::Projects => "projects.yaml",
        };

        write!(f, "{string}")
    }
}

#[expect(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_file_creates_file_if_missing() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_content = "theme = \"dark\"\n";

        temp_env::with_var("PROJMAN_CONFIG_DIR", Some(temp_dir.path()), || {
            let result = ConfigDir::ConfigFile.get_file(Some(test_content));
            assert!(result.is_ok());

            let path = result.unwrap();
            assert!(path.exists());
            assert_eq!(path, temp_dir.path().join("config.toml"));

            let file_content = fs::read_to_string(&path).unwrap();
            assert_eq!(file_content, test_content);
        });
    }

    #[test]
    fn test_get_file_returns_existing_file_without_overwriting() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_content = "original = true";

        temp_env::with_var("PROJMAN_CONFIG_DIR", Some(temp_dir.path()), || {
            let file_path = temp_dir.path().join("config.toml");
            fs::write(&file_path, test_content).unwrap();

            let new_content = "original = false";

            let result = ConfigDir::ConfigFile.get_file(Some(new_content));
            assert!(result.is_ok());

            let path = result.unwrap();
            let current_content = fs::read_to_string(&path).unwrap();
            assert_eq!(current_content, test_content);
        });
    }

    #[test]
    fn test_get_file_no_create_if_missing_returns_path_anyway() {
        let temp_dir = tempfile::tempdir().unwrap();

        temp_env::with_var("PROJMAN_CONFIG_DIR", Some(temp_dir.path()), || {
            let result = ConfigDir::ConfigFile.get_file(None);
            assert!(result.is_ok());

            let path = result.unwrap();
            assert_eq!(path, temp_dir.path().join("config.toml"));
            assert!(!path.exists());
        });
    }

    #[test]
    fn test_get_file_handles_nested_dirs_for_templates() {
        let temp_dir = tempfile::tempdir().unwrap();
        let template_content = "{\"name\": \"rust\"}".to_string();

        temp_env::with_var("PROJMAN_CONFIG_DIR", Some(temp_dir.path()), || {
            let result = ConfigDir::Template("rust/template.yaml".to_string())
                .get_file(Some(&template_content));
            assert!(result.is_ok());

            let path = result.unwrap();
            assert_eq!(path, temp_dir.path().join("templates/rust/template.yaml"));
            assert!(path.exists());

            let current_content = fs::read_to_string(&path).unwrap();
            assert_eq!(current_content, template_content);
        });
    }
}
