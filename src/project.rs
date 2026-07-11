use crate::{app_state::AppState, config::Config, prelude::*, template::Template};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub repo: String,
    pub license: String,
    pub template_name: String,
}

pub mod info;
mod load_projects;

pub use load_projects::load_projects;

impl Project {
    const PROJMAN_FILE_NAME: &str = ".projman.toml";

    pub fn run(project: &Arc<Self>, app_state: &Arc<AppState>) -> Result<()> {
        app_state
            .get_template(&project.template_name)?
            .run(&project.path)
    }

    pub fn new(config: &Config) -> Self {
        let projects_dir = &config.general.projects_dir;

        let name = "NewProject";

        Self {
            name: String::from(name),
            path: PathBuf::from(projects_dir).join(name),
            repo: String::new(),
            license: String::new(),
            template_name: String::new(),
        }
    }

    pub fn template<'a>(&self, app_state: &'a AppState) -> Result<&'a Template> {
        app_state.get_template(&self.template_name)
    }

    pub fn exists(&self) -> Result<bool> {
        let path = &self.path;

        if !path.is_dir() {
            return Ok(false);
        }

        let project_file_path = path.join(Self::PROJMAN_FILE_NAME);

        if !project_file_path.is_file() {
            return Ok(false);
        }

        let project_file_toml = toml::from_str::<Self>(
            &fs::read_to_string(project_file_path)
                .map_err(|err| Error::ReadProjectFile(err.to_string()))?,
        )
        .map_err(|err| Error::ReadProjectFile(err.to_string()))?;

        if project_file_toml.name != self.name {
            return Ok(false);
        }

        if project_file_toml.template_name != self.template_name {
            return Ok(false);
        }

        if project_file_toml.repo != self.repo {
            return Ok(false);
        }

        if project_file_toml.license != self.license {
            return Ok(false);
        }

        Ok(true)
    }
}
