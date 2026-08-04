use crate::{
    config_dir::ConfigDir, prelude::*, template::template_config::TemplateConfig,
};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command as StdCommand,
};

mod project_context;

pub mod command;
pub mod file;
pub mod folder;
pub mod template_config;

pub fn load_templates() -> Result<HashMap<String, Template>> {
    let templates_dir_path = ConfigDir::Templates.get_file(None)?;

    let dir_entries = templates_dir_path
        .read_dir()
        .map_err(|err| Error::ReadTemplatesDir(err.to_string()))?;

    let mut templates = HashMap::new();

    for entry in dir_entries {
        let entry_path = entry
            .map_err(|err| Error::ReadTemplatesDir(err.to_string()))?
            .path();

        if !entry_path.is_dir() {
            continue;
        }

        if let Some(template_name) = entry_path.file_name().and_then(OsStr::to_str) {
            let template = Template::load(template_name)?;

            templates.insert(template_name.to_string(), template);
        }
    }

    Ok(templates)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Template {
    pub name: String,
    pub config: TemplateConfig,
    pub icon_path: PathBuf,
}

impl Template {
    pub fn load(name: &str) -> Result<Self> {
        let base_dir = ConfigDir::Template(name.to_string()).get_file(None)?;
        let config_path = base_dir.join("template.yaml");
        let icon_path = base_dir.join("icon.svg");

        let config_str = fs::read_to_string(&config_path)
            .map_err(|err| Error::ReadTemplate(name.to_string(), err.to_string()))?;

        let config = serde_yaml::from_str(&config_str)
            .map_err(|err| Error::ParseTemplate(name.to_string(), err.to_string()))?;

        Ok(Self {
            name: name.to_string(),
            config,
            icon_path,
        })
    }

    pub fn run(&self, project_path: &Path) -> Result<()> {
        for command in &self.config.run {
            StdCommand::new(&command.program)
                .args(&command.args)
                .current_dir(project_path)
                .spawn()
                .map_err(|err| Error::RunCommand(command.to_string(), err.to_string()))?;
        }
        Ok(())
    }

    pub fn included_paths(&self, root: &Path) -> Vec<PathBuf> {
        self.config
            .included_paths
            .iter()
            .map(|p| root.join(p))
            .collect()
    }

    pub fn excluded_paths(&self) -> Vec<&str> {
        self.config
            .excluded_paths
            .iter()
            .map(String::as_str)
            .collect()
    }
}

// #[expect(clippy::unwrap_used)]
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::template::{command::Command, template_config::TemplateConfig};
//     use std::fs;
//
//     fn create_mock_template_config() -> TemplateConfig {
//         TemplateConfig::new()
//             .run(&[Command::new("echo").args(&["hello"])])
//             .included_paths(&["src", "Cargo.toml"])
//             .excluded_paths(&[".git", "target"])
//     }
//
//     #[test]
//     fn test_template_load_success() {
//         let temp_dir = tempfile::tempdir().unwrap();
//         let template_name = "rust-basic";
//
//         temp_env::with_var("PROJMAN_CONFIG_DIR", Some(temp_dir.path()), || {
//             let base_dir = temp_dir.path().join("templates").join(template_name);
//             fs::create_dir_all(&base_dir).unwrap();
//
//             let config = create_mock_template_config();
//             let yaml_str = serde_yaml::to_string(&config).unwrap();
//
//             fs::write(base_dir.join("template.yaml"), yaml_str).unwrap();
//             fs::write(base_dir.join("icon.svg"), "<svg></svg>").unwrap();
//
//             let result = Template::load(template_name);
//             assert!(result.is_ok());
//
//             let template = result.unwrap();
//             assert_eq!(template.name, template_name);
//             assert_eq!(template.icon_path, base_dir.join("icon.svg"));
//             assert_eq!(template.config.excluded_paths, config.excluded_paths);
//         });
//     }
//
//     #[test]
//     fn test_template_load_fails_when_yaml_missing() {
//         let temp_dir = tempfile::tempdir().unwrap();
//
//         temp_env::with_var("PROJMAN_CONFIG_DIR", Some(temp_dir.path()), || {
//             let result = Template::load("non-existent-template");
//             assert!(result.is_err());
//
//             if let Err(Error::ReadTemplate(name, _)) = result {
//                 assert_eq!(name, "non-existent-template");
//             } else {
//                 panic!("Expected ReadTemplate error");
//             }
//         });
//     }
//
//     #[test]
//     fn test_load_templates_finds_and_loads_multiple_directories() {
//         let temp_dir = tempfile::tempdir().unwrap();
//
//         temp_env::with_var("PROJMAN_CONFIG_DIR", Some(temp_dir.path()), || {
//             let templates_root = temp_dir.path().join("templates");
//
//             let dir_a = templates_root.join("template-a");
//             fs::create_dir_all(&dir_a).unwrap();
//             let config_a = create_mock_template_config();
//             fs::write(
//                 dir_a.join("template.yaml"),
//                 serde_yaml::to_string(&config_a).unwrap(),
//             )
//             .unwrap();
//
//             let dir_b = templates_root.join("template-b");
//             fs::create_dir_all(&dir_b).unwrap();
//             let config_b = create_mock_template_config();
//             fs::write(
//                 dir_b.join("template.yaml"),
//                 serde_yaml::to_string(&config_b).unwrap(),
//             )
//             .unwrap();
//
//             fs::write(templates_root.join("stray_file.txt"), "ignore me").unwrap();
//
//             let loaded = load_templates().unwrap();
//             assert_eq!(loaded.len(), 2);
//             assert!(loaded.contains_key("template-a"));
//             assert!(loaded.contains_key("template-b"));
//             assert!(!loaded.contains_key("stray_file.txt"));
//         });
//     }
//
//     #[test]
//     fn test_included_and_excluded_paths_mapping() {
//         let config = create_mock_template_config();
//
//         let template = Template {
//             name: "test".to_string(),
//             config,
//             icon_path: PathBuf::new(),
//         };
//
//         let root_path = Path::new("/workspace/project");
//         let included = template.included_paths(root_path);
//
//         assert_eq!(
//             included,
//             vec![
//                 PathBuf::from("/workspace/project/src"),
//                 PathBuf::from("/workspace/project/Cargo.toml")
//             ]
//         );
//
//         let excluded = template.excluded_paths();
//         assert_eq!(excluded, vec![".git", "target"]);
//     }
//
//     #[test]
//     fn test_template_run_spawns_configured_command() {
//         let temp_dir = tempfile::tempdir().unwrap();
//         let project_path = temp_dir.path().join("my_new_project");
//
//         fs::create_dir_all(&project_path).unwrap();
//
//         let config =
//             TemplateConfig::new().run(&[Command::new("echo").args(&["initializing"])]);
//
//         let template = Template {
//             name: "runner-test".to_string(),
//             config,
//             icon_path: PathBuf::new(),
//         };
//
//         let result = template.run(&project_path);
//         assert!(
//             result.is_ok(),
//             "Failed to execute template run process context"
//         );
//     }
// }
