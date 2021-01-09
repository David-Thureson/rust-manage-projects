use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug)]
pub struct Model {
    pub pcs: Vec<PC>,
}

#[derive(Debug)]
pub struct PC {
    pub name: String,
    pub rust_root_folder: String,
    pub projects: BTreeMap<String, Project>,
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub rust_projects: BTreeMap<String, RustProject>,
}

/*
struct IDEProject {
    name: String,
    crates: Vec<Crate>,
}
*/

#[derive(Debug)]
pub struct RustProject {
    pub name: String,
    pub path: String,
    pub dependencies: BTreeMap<String, Dependency>,
}

#[derive(Debug)]
pub struct Dependency {
    pub crate_name: String,
    pub version: Option<String>,
    pub is_local: bool,
}

impl Model {
    pub fn new(pcs: Vec<PC>) -> Self {
        Model {
            pcs,
        }
    }
}

impl PC {
    pub fn new(name: &str, rust_root_folder: &str) -> Self {
        Self {
            name: name.to_string(),
            rust_root_folder: rust_root_folder.to_string(),
            projects: BTreeMap::new(),
        }
    }

    pub fn add_project(&mut self, project: Project) {
        let key = project.name.to_lowercase();
        assert!(!self.projects.contains_key(&key));
        self.projects.insert(key, project);
    }
}

impl Project {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            rust_projects: BTreeMap::new(),
        }
    }

    pub fn add_rust_project(&mut self, rust_project: RustProject) {
        let key = rust_project.name.to_lowercase();
        assert!(!self.rust_projects.contains_key(&key));
        self.rust_projects.insert(key, rust_project);
    }
}

impl RustProject {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            dependencies: Default::default(),
        }
    }

    pub fn add_dependency(&mut self, dependency: Dependency) {
        let key = dependency.crate_name.to_lowercase();
        assert!(!self.dependencies.contains_key(&key));
        self.dependencies.insert(key, dependency);
    }
}

impl Dependency {
    pub fn new(crate_name: &str, version: Option<String>, is_local: bool) -> Self {
        Self {
            crate_name: crate_name.to_string(),
            version,
            is_local,
        }
    }
}