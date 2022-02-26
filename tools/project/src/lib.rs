use std::{fs::*, path::{Path, PathBuf}, sync::Arc};

mod cfg;
mod error;

pub use cfg::*;
pub use error::*;
use prelude::{SourceFile, BoltMessage};

pub struct Project {
    path: String,
    cfg: Option<ProjectCfg>,
    source_files: Vec<Arc<SourceFile>>
}

impl Project {
    /// Creates a new project in a directory
    /// 
    /// # Example
    /// 
    /// let project = Project::new("example").unwrap();
    pub fn new(path: String) -> Result<Project, ProjectError> {
        if read_dir(&path).is_err() {
            return Err(ProjectError::DirectoryNotFound(path));
        }

        let proj = Project {
            path,
            cfg: None,
            source_files: vec![]
        };

        return Ok(proj);
    }

    /// Reads the configuration file for a project
    /// 
    /// # Example
    /// 
    /// func read_cfg(project: &mut Project) {
    ///     project.read_cfg();
    /// }
    /// 
    pub fn read_config(&mut self) -> Result<(), ProjectError> {
        let cfg_name = "project.yaml";
        let cfg_path = Path::new(&self.path).join(cfg_name);
        let cfg_path_str = cfg_path.as_os_str().to_str().unwrap().to_string();

        let cfg_file = File::open(cfg_path)
            .map_err(|_| ProjectError::ConfigNotFound(cfg_path_str))?;

        let config: ProjectCfg = serde_yaml::from_reader(cfg_file)
            .map_err(|e| ProjectError::CfgError(e))?;

        self.cfg = Some(config);

        Ok(())
    }

    /// Search the src director for source code files
    /// 
    /// # Example
    /// 
    /// let mut project = Project::new("example");
    /// project.search();
    /// 
    pub fn search(&mut self) -> Result<(), Box<dyn BoltMessage>> {
        let cfg_name = "src";
        let cfg_path = Path::new(&self.path).join(cfg_name);
        let cfg_path_str = cfg_path.as_os_str().to_str().unwrap().to_string();

        if read_dir(cfg_path.as_path()).is_err() {
            return Err(Box::new(ProjectError::DirectoryNotFound(cfg_path_str)));
        }

        let file_paths = search_folder(cfg_path.as_path());

        let mut files = vec![];

        for file_path in file_paths.iter() {
            match SourceFile::open_file(file_path.as_path()) {
                Ok(file) => files.push(file),
                Err(e) => return Err(Box::new(e))
            }
        }

        self.source_files = files;

        Ok(())
    }

    /// Returns the project config
    /// 
    /// This function must be called after read_config
    /// 
    /// # Example
    /// 
    /// func read_cfg(project: &mut Project) {
    ///     project.read_cfg();
    ///     println!("{:?}", project.config());
    /// }
    /// 
    pub fn config(&self) -> &ProjectCfg {
        self.cfg.as_ref().unwrap()
    }

    /// Returns the files in the source directory
    /// 
    /// This function must be called after search
    /// 
    /// # Example
    /// 
    /// let mut project = Project::new("example");
    /// project.search();
    /// let source_files = project.source_files();
    /// 
    pub fn source_files(&self) -> &Vec<Arc<SourceFile>> {
        &self.source_files
    }
}

/// Recursively search for files in a folder
fn search_folder(path: &Path) -> Vec<PathBuf> {
    let mut files = vec![];

    let dir = read_dir(path).unwrap();

    for entry in dir {
        let entry = entry.unwrap();

        if entry.file_type().unwrap().is_dir() {
            let child_path = path.join(entry.file_name());

            files.extend(search_folder(child_path.as_path()));
        } else {
            let file_path = path.join(entry.file_name());

            files.push(file_path);
        }
    }

    files
}