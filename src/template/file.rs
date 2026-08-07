use std::path::Path;

use crate::template::project_context::ProjectContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct File {
    pub path: String,
    pub contents: String,
    pub tracked: bool,
}

impl File {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            contents: String::new(),
            tracked: false,
        }
    }

    pub fn contents(self, content: &str) -> Self {
        Self {
            contents: content.to_string(),
            ..self
        }
    }

    pub fn tracked(self, tracked: bool) -> Self {
        Self { tracked, ..self }
    }

    pub fn resolve(&self, root: &Path, ctx: &ProjectContext) -> Self {
        Self {
            path: root
                .join(ctx.format(&self.path))
                .to_string_lossy()
                .to_string(),
            contents: ctx.format(&self.contents),
            tracked: self.tracked,
        }
    }
}
