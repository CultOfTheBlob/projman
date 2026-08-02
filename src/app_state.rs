use crate::{
    config_dir::ConfigDir,
    prelude::*,
    project::{self, valid_project::ValidProject},
    template::{self, Template},
    utils::{self, LogType},
};
use gpui::{App, BorrowAppContext as _, Global};
use serde::{Serializer, ser::SerializeSeq};
use std::{collections::HashMap, fs::File, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: HashMap<String, Template>,
    pub projects: Vec<ValidProject>,

    pub modal_active: bool,
}

impl AppState {
    pub fn new() -> Self {
        let templates = template::load_templates().unwrap_or_else(|err| {
            utils::log(&err.to_string(), LogType::Error);

            HashMap::new()
        });

        let projects = project::load_projects().unwrap_or_else(|err| {
            utils::log(&err.to_string(), LogType::Error);

            vec![]
        });

        Self {
            templates,
            projects,

            modal_active: false,
        }
    }

    pub fn set_modal_active(cx: &mut App, modal_active: bool) {
        cx.update_global::<GlobalAppState, ()>(|app_state: &mut GlobalAppState, _| {
            let app_state = Arc::make_mut(&mut app_state.0);
            app_state.modal_active = modal_active;
        });
    }

    pub fn get_template(&self, template_name: &str) -> Result<&Template> {
        self.templates
            .get(template_name)
            .ok_or_else(|| Error::GetTemplate(template_name.to_string()))
    }

    pub fn get_filtered_projects(&self, filter: &str) -> Vec<&ValidProject> {
        let filter = filter.to_lowercase();

        self.projects
            .iter()
            .filter(|project| {
                let name = match &project {
                    ValidProject::Existant(p) => &p.name,
                    ValidProject::Nonexistant(p) => &p.name,
                };

                name.to_lowercase().contains(&filter)
            })
            .collect()
    }

    pub fn add_project(&mut self, project: ValidProject) -> Result<()> {
        self.projects.push(project);

        let projects_path = ConfigDir::Projects.get_file(Some(""))?;
        let writer = File::create(projects_path)
            .map_err(|err| Error::AddProjectToProjects(err.to_string()))?;

        let mut serializer = serde_yaml::Serializer::new(writer);
        let mut seq = serializer
            .serialize_seq(Some(self.projects.len()))
            .map_err(|err| Error::AddProjectToProjects(err.to_string()))?;

        for project in &self.projects {
            match project {
                ValidProject::Existant(p) => seq
                    .serialize_element(p.as_ref())
                    .map_err(|err| Error::AddProjectToProjects(err.to_string()))?,
                ValidProject::Nonexistant(p) => seq
                    .serialize_element(p.as_ref())
                    .map_err(|err| Error::AddProjectToProjects(err.to_string()))?,
            }
        }

        seq.end()
            .map_err(|err| Error::AddProjectToProjects(err.to_string()))?;

        Ok(())
    }

    pub fn remove_project(&mut self, project_index: usize) {
        let project = self.projects[project_index].clone();

        match project {
            ValidProject::Existant(project) => todo!(),
            ValidProject::Nonexistant(_) => {
                self.projects.remove(project_index);
            }
        }
    }
}

impl Global for AppState {}

#[derive(Debug, Clone)]
pub struct GlobalAppState(pub Arc<AppState>);

impl Global for GlobalAppState {}
