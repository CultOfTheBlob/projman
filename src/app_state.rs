use crate::{
    prelude::*,
    project::{self, Project, valid_project::ValidProject},
    template::{self, Template},
    utils::{self, LogType},
};
use gpui::{App, BorrowAppContext as _, Global};
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use update_projects_file::update_projects_file;

mod update_projects_file;

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: HashMap<String, Template>,
    pub projects: Vec<ValidProject>,

    pub selected_project_index: Option<usize>,

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

            selected_project_index: None,

            modal_active: false,
        }
    }

    pub fn set_modal_active(cx: &mut App, modal_active: bool) {
        cx.update_global::<GlobalAppState, ()>(|app_state: &mut GlobalAppState, _| {
            let app_state = Arc::make_mut(&mut app_state.0);
            app_state.modal_active = modal_active;
        });
    }

    pub fn set_selected_project_index(
        cx: &mut App,
        selected_project_index: Option<usize>,
    ) {
        cx.update_global::<GlobalAppState, ()>(|app_state: &mut GlobalAppState, _| {
            let app_state = Arc::make_mut(&mut app_state.0);
            app_state.selected_project_index = selected_project_index;
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

    pub fn get_selected_project(&self) -> Option<ValidProject> {
        self.selected_project_index
            .map(|index| self.projects[index].clone())
    }

    pub fn add_project(&mut self, project: ValidProject) -> Result<()> {
        self.projects.push(project);

        update_projects_file(&self.projects)?;

        Ok(())
    }

    pub fn remove_project(
        &mut self,
        project_index: usize,
        remove_project_folder: bool,
    ) -> Result<()> {
        let project = self.projects[project_index].clone();

        self.projects.remove(project_index);

        self.selected_project_index = None;

        if let ValidProject::Existant(project) = project {
            let project_path = PathBuf::from(&project.name);

            let project_file_path = project_path.join(Project::<()>::PROJECT_FILE_NAME);

            fs::remove_file(project_file_path)
                .map_err(|err| Error::RemoveProject(err.to_string()))?;

            if remove_project_folder {
                fs::remove_dir_all(project_path)
                    .map_err(|err| Error::RemoveProject(err.to_string()))?;
            }
        }

        update_projects_file(&self.projects)?;

        Ok(())
    }
}

impl Global for AppState {}

#[derive(Debug, Clone)]
pub struct GlobalAppState(pub Arc<AppState>);

impl Global for GlobalAppState {}
