use std::{marker::PhantomData, sync::Arc};

use crate::{
    project::{Existant, Nonexistant, Project, Unvalidated},
    utils::{self, LogType},
};

#[derive(Debug, Clone)]
pub enum ValidProject {
    Existant(Arc<Project<Existant>>),
    Nonexistant(Arc<Project<Nonexistant>>),
}

impl Project<Unvalidated> {
    pub(super) fn validate(self) -> ValidProject {
        let project_exists = self.exists().unwrap_or_else(|err| {
            utils::log(&err.to_string(), LogType::Error);

            false
        });

        if project_exists {
            ValidProject::Existant(Arc::new(Project {
                name: self.name,
                path: self.path,
                repo: self.repo,
                license: self.license,
                template_name: self.template_name,

                state: PhantomData,
            }))
        } else {
            ValidProject::Nonexistant(Arc::new(Project {
                name: self.name,
                path: self.path,
                repo: self.repo,
                license: self.license,
                template_name: self.template_name,

                state: PhantomData,
            }))
        }
    }
}
