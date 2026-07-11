use std::fs;

use crate::{config_dir::ConfigDir, prelude::*, project::Project};

pub fn load_projects() -> Result<Vec<Project>> {
    let projects_path = ConfigDir::Projects.get_file(Some(""))?;

    serde_yaml::from_str::<Vec<Project>>(
        &fs::read_to_string(&projects_path)
            .map_err(|err| Error::ReadProjectList(err.to_string()))?,
    )
    .map_err(|err| Error::ReadProjectList(err.to_string()))
}
