use crate::template::{command::Command, file::File, folder::Folder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct TemplateConfig {
    pub dir_structure: Vec<Folder>,
    pub files: Vec<File>,
    pub build: Vec<Command>,
    pub run: Vec<Command>,
    pub included_paths: Vec<String>,
    pub excluded_paths: Vec<String>,
}

impl TemplateConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dir_structure(self, dir_structure: &[Folder]) -> Self {
        Self {
            dir_structure: dir_structure.into(),
            ..self
        }
    }

    pub fn files(self, files: &[File]) -> Self {
        Self {
            files: files.into(),
            ..self
        }
    }

    pub fn build(self, build: &[Command]) -> Self {
        Self {
            build: build.into(),
            ..self
        }
    }

    pub fn run(self, run: &[Command]) -> Self {
        Self {
            run: run.into(),
            ..self
        }
    }

    pub fn included_paths(self, included_paths: &[&str]) -> Self {
        Self {
            included_paths: included_paths.iter().map(ToString::to_string).collect(),
            ..self
        }
    }

    pub fn excluded_paths(self, excluded_paths: &[&str]) -> Self {
        Self {
            excluded_paths: excluded_paths.iter().map(ToString::to_string).collect(),
            ..self
        }
    }
}
