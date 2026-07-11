use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Result as FmtResult},
    string::ToString,
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
            args: vec![],
        }
    }

    pub fn args(self, args: &[&str]) -> Self {
        Self {
            args: args.iter().map(ToString::to_string).collect(),
            ..self
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{} {}", self.program, self.args.join(" "))
    }
}
