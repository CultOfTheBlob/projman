use crate::prelude::*;
use directories::ProjectDirs;
use std::{env, path::PathBuf};

pub fn get_config_path() -> Result<PathBuf> {
    if let Ok(override_dir) = env::var("PROJMAN_CONFIG_DIR") {
        return Ok(PathBuf::from(override_dir));
    }

    ProjectDirs::from("", "", "projman")
        .map_or(Err(Error::GetConfigDir), |project_dirs: ProjectDirs| {
            Ok(project_dirs.config_dir().to_path_buf())
        })
}
