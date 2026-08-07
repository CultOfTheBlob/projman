use crate::{
    app_state::AppState,
    prelude::*,
    project::{Existant, Nonexistant, Project},
    template::{command::Command, file::File, folder::Folder},
};
use askalono::{Store, TextData};
use futures::AsyncBufReadExt;
use futures::StreamExt;
use futures::io::BufReader;
use git2::{IndexAddOption, Repository};
use smol::process::{Command as AsyncCommand, Stdio};
use std::{
    fs::{self},
    iter,
    marker::PhantomData,
};

impl Project<Nonexistant> {
    pub async fn create<F>(
        mut self,
        app_state: &AppState,
        on_log: F,
    ) -> Result<Project<Existant>>
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        let template = self.get_template(app_state)?;

        create_project_dir(&self)?;

        clone_project_repo(&self)?;

        load_project_license(&mut self)?;

        create_project_file(&self)?;

        create_project_dir_structure(&self, &template.config.dir_structure)?;

        create_project_files(&self, &template.config.files)?;

        execute_build_commands(&self, &template.config.build, on_log).await?;

        commit_project_init(&self)?;

        Ok(Project::<Existant> {
            name: self.name,
            path: self.path,
            repo: self.repo,
            license: self.license,
            template_name: self.template_name,
            state: PhantomData,
        })
    }
}

fn create_project_dir(project: &Project<Nonexistant>) -> Result<()> {
    fs::create_dir_all(&project.path)
        .map_err(|err| Error::CreateProjectDir(err.to_string()))
}

fn clone_project_repo(project: &Project<Nonexistant>) -> Result<()> {
    project.clone_repo()
}

fn load_project_license(project: &mut Project<Nonexistant>) -> Result<()> {
    let cache = &include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/cache/license.cache.zstd"
    ))[..];

    let store = Store::from_cache(cache)
        .map_err(|err| Error::LoadProjectLicense(err.to_string()))?;

    let license_path = project.path.join("LICENSE");

    let license_contents = fs::read_to_string(&license_path)
        .map_err(|err| Error::LoadProjectLicense(err.to_string()))?;

    let license = store
        .analyze(&TextData::from(license_contents.as_str()))
        .name
        .to_owned();

    project.license = license;

    Ok(())
}

fn create_project_file(project: &Project<Nonexistant>) -> Result<()> {
    let project_file_contents = toml::to_string_pretty(&project)
        .map_err(|err| Error::CreateProjectFile(err.to_string()))?;

    fs::write(project.get_project_file_path(), project_file_contents)
        .map_err(|err| Error::CreateProjectFile(err.to_string()))?;

    Ok(())
}

fn create_project_dir_structure(
    project: &Project<Nonexistant>,
    dir_structure: &[Folder],
) -> Result<()> {
    for dir in dir_structure {
        let dirs = dir.resolve(&project.path, &project.into());

        for dir in dirs {
            fs::create_dir_all(&dir)
                .map_err(|err| Error::CreateProjectDirStructure(err.to_string()))?;
        }
    }

    Ok(())
}

fn create_project_files(project: &Project<Nonexistant>, files: &[File]) -> Result<()> {
    for file in files {
        let file = file.resolve(&project.path, &project.into());

        fs::write(file.path, file.contents)
            .map_err(|err| Error::CreateProjectFiles(err.to_string()))?;
    }

    Ok(())
}

pub async fn execute_build_commands<F>(
    project: &Project<Nonexistant>,
    commands: &[Command],
    on_log: F,
) -> Result<()>
where
    F: Fn(String) + Send + Sync + 'static,
{
    let on_log = std::sync::Arc::new(on_log);

    for command in commands {
        on_log(format!("> {} {}", command.program, command.args.join(" ")));

        let mut child = AsyncCommand::new(&command.program)
            .args(&command.args)
            .current_dir(&project.path)
            .env("FORCE_COLOR", "1")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| Error::ExecuteProjectCommands(err.to_string()))?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let stdout_task = smol::spawn({
            let on_log = on_log.clone();
            async move {
                if let Some(stdout) = stdout {
                    let mut lines = BufReader::new(stdout).lines();
                    while let Some(Ok(line)) = lines.next().await {
                        on_log(line);
                    }
                }
            }
        });

        let stderr_task = smol::spawn({
            let on_log = on_log.clone();
            async move {
                if let Some(stderr) = stderr {
                    let mut lines = BufReader::new(stderr).lines();
                    while let Some(Ok(line)) = lines.next().await {
                        on_log(line);
                    }
                }
            }
        });

        stdout_task.await;
        stderr_task.await;

        let status = child
            .status()
            .await
            .map_err(|err| Error::ExecuteProjectCommands(err.to_string()))?;

        if !status.success() {
            return Err(Error::ExecuteProjectCommands(format!(
                "Command failed with exit code: {:?}",
                status.code()
            )));
        }
    }

    Ok(())
}

fn commit_project_init(project: &Project<Nonexistant>) -> Result<()> {
    let project_repo = Repository::open(&project.path)
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    let mut index = project_repo
        .index()
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    index
        .add_all(iter::once(&"*"), IndexAddOption::DEFAULT, None)
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    index
        .write()
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    let signature = project_repo
        .signature()
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    let tree = project_repo
        .find_tree(
            index
                .write_tree()
                .map_err(|err| Error::CommitProjectInit(err.to_string()))?,
        )
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    let parent_commit = project_repo
        .head()
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?
        .peel_to_commit()
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    project_repo
        .commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initialized ProjMan project",
            &tree,
            &[&parent_commit],
        )
        .map_err(|err| Error::CommitProjectInit(err.to_string()))?;

    Ok(())
}
