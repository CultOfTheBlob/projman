use crate::template::project_context::ProjectContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct File {
    pub path: String,
    pub content: String,
    pub tracked: bool,
}

impl File {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            content: String::new(),
            tracked: false,
        }
    }

    pub fn content(self, content: &str) -> Self {
        Self {
            content: content.to_string(),
            ..self
        }
    }

    pub fn tracked(self, tracked: bool) -> Self {
        Self { tracked, ..self }
    }

    pub fn resolve(&self, ctx: &ProjectContext) -> Self {
        Self {
            path: ctx.format(&self.path),
            content: ctx.format(&self.content),
            tracked: self.tracked,
        }
    }
}
