use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, path::PathBuf};

mod existant;
pub mod info;
mod load_projects;
mod unvalidated;
pub mod valid_project;

pub use load_projects::load_projects;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Project<State = Unvalidated> {
    pub name: String,
    pub path: PathBuf,
    pub repo: String,
    pub license: String,
    pub template_name: String,

    #[serde(skip)]
    pub state: PhantomData<State>,
}

#[derive(Debug, Clone)]
pub struct Existant;

#[derive(Debug, Clone)]
pub struct Nonexistant;

#[derive(Debug, Clone)]
pub struct Unvalidated;

impl<State> Project<State> {
    const PROJECT_FILE_NAME: &str = ".projman.toml";

    pub fn new(config: &Config) -> Self {
        let projects_dir = &config.general.projects_dir;

        let name = "NewProject";

        Self {
            name: String::from(name),
            path: PathBuf::from(projects_dir).join(name),
            repo: String::new(),
            license: String::new(),
            template_name: String::new(),

            state: PhantomData,
        }
    }
}
