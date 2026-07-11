use crate::{
    prelude::*,
    project::{self, Project},
    template::{self, Template},
    utils::{self, LogType},
};
use gpui::{App, BorrowAppContext as _, Global};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: HashMap<String, Template>,
    pub projects: Vec<Arc<Project>>,

    pub modal_active: bool,
}

impl AppState {
    pub fn new() -> Self {
        let templates = template::load_templates().unwrap_or_else(|err| {
            utils::log(&err.to_string(), LogType::Error);

            HashMap::new()
        });

        let projects = project::load_projects()
            .unwrap_or_else(|err| {
                utils::log(&err.to_string(), LogType::Error);

                vec![]
            })
            .into_iter()
            .map(Arc::new)
            .collect();

        Self {
            templates,
            projects,

            modal_active: false,
        }
    }

    pub fn set_modal_active(cx: &mut App, modal_active: bool) {
        cx.update_global::<Self, _>(|state: &mut Self, _| {
            state.modal_active = modal_active;
        });
    }

    pub fn get_template(&self, template_name: &str) -> Result<&Template> {
        self.templates
            .get(template_name)
            .ok_or_else(|| Error::GetTemplate(template_name.to_string()))
    }
}

impl Global for AppState {}

#[derive(Debug, Clone)]
pub struct GlobalAppState(pub Arc<AppState>);

impl Global for GlobalAppState {}
