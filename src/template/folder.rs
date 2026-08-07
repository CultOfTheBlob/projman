use crate::template::project_context::ProjectContext;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Folder {
    pub name: String,
    pub sub_dirs: Vec<Self>,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sub_dirs: vec![],
        }
    }

    pub fn sub_dirs(self, sub_dirs: &[Self]) -> Self {
        Self {
            sub_dirs: sub_dirs.into(),
            ..self
        }
    }

    pub fn resolve(&self, root: &Path, ctx: &ProjectContext) -> Vec<PathBuf> {
        let formatted_name = ctx.format(&self.name);
        let current_dir = root.join(formatted_name);

        let mut dirs = vec![current_dir];
        for sub in &self.sub_dirs {
            let current_dir = &dirs[0];

            dirs.extend(sub.resolve(current_dir, ctx));
        }

        dirs
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::template::project_context::ProjectContext;
//
//     fn create_mock_context<'a>() -> ProjectContext<'a> {
//         ProjectContext {
//             name: "project_name",
//             repo: "git@project_name.com",
//             license: "LICENSE",
//         }
//     }
//
//     #[test]
//     fn test_resolve_flat_folder_with_context_formatting() {
//         let ctx = create_mock_context();
//         let root = Path::new("/workspace");
//
//         let folder = Folder::new("#{name}").sub_dirs(&[]);
//
//         let resolved = folder.resolve(root, &ctx);
//
//         assert_eq!(resolved, vec![PathBuf::from("/workspace/project_name")]);
//     }
//
//     #[test]
//     fn test_resolve_nested_subdirectories_recursively() {
//         let ctx = create_mock_context();
//         let root = Path::new("/workspace");
//
//         let folder =
//             Folder::new("#{name}")
//                 .sub_dirs(&[Folder::new("src")
//                     .sub_dirs(&[Folder::new("controllers").sub_dirs(&[])])]);
//
//         let resolved = folder.resolve(root, &ctx);
//
//         assert_eq!(
//             resolved,
//             vec![
//                 PathBuf::from("/workspace/project_name"),
//                 PathBuf::from("/workspace/project_name/src"),
//                 PathBuf::from("/workspace/project_name/src/controllers"),
//             ]
//         );
//     }
// }
