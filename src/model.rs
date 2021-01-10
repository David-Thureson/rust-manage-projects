use std::collections::BTreeMap;
use std::path::Path;
use serde::{Deserialize, Serialize};

use util::group::Grouper;

#[derive(Debug, Deserialize, Serialize)]
pub struct Model {
    pub pcs: BTreeMap<String, PC>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PC {
    pub name: String,
    pub rust_root_folder: String,
    pub projects: BTreeMap<String, Project>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub repository: Option<Repository>,
    pub rust_projects: BTreeMap<String, RustProject>,
}

/*
struct IDEProject {
    name: String,
    crates: Vec<Crate>,
}
*/

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub owner: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RustProject {
    pub name: String,
    pub path: String,
    pub dependencies: BTreeMap<String, Dependency>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dependency {
    pub crate_name: String,
    pub version: Option<String>,
    pub is_local: bool,
}

impl Model {
    pub fn new(mut pc_vector: Vec<PC>) -> Self {
        let mut pc_map = BTreeMap::new();
        for pc in pc_vector.drain(..) {
            pc_map.insert(pc.name.to_lowercase(), pc);
        };
        Model {
            pcs: pc_map,
        }
    }

    pub fn report_summary(&self) {
        let mut grouper_pcs = Grouper::new("PCs");
        let mut grouper_repo_owners = Grouper::new("Repository Owners");
        let mut grouper_rust_projects_per_project = Grouper::new("Rust Projects per Project");
        let mut grouper_crates_local = Grouper::new("Local Crates");
        let mut grouper_crates = Grouper::new("Crates");
        for pc in self.pcs.values() {
            grouper_pcs.record_entry_with_count(&pc.name, pc.projects.len());
            for project in pc.projects.values() {
                let owner: String = project.repository.as_ref().map_or("None".to_string(), |repo| repo.owner.clone());
                grouper_repo_owners.record_entry(&owner);
                grouper_rust_projects_per_project.record_entry(&project.rust_projects.len());
                for dependency in project.rust_projects
                    .values()
                    .map(|rust_project| rust_project.dependencies.values())
                    .flatten() {
                    if dependency.is_local {
                        grouper_crates_local.record_entry(&dependency.crate_name);
                    } else {
                        grouper_crates.record_entry(&dependency.crate_name);
                    }
                }
            }
        }
        grouper_pcs.print_by_key(0, None);
        grouper_repo_owners.print_by_count(0, None);
        grouper_rust_projects_per_project.print_by_key(0, None);
        grouper_crates_local.print_by_count(0, None);
        grouper_crates.print_by_count(0, None);

        /*
        for project in self.pcs.values()
            .flat_map(|pc| pc.projects.values())
            .filter(|project| project.rust_projects.len() > 6) {
            dbg!(&project);
        }
        */
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
            repository: None,
            rust_projects: BTreeMap::new(),
        }
    }

    pub fn add_rust_project(&mut self, rust_project: RustProject) {
        let key = rust_project.name.to_lowercase();
        assert!(!self.rust_projects.contains_key(&key));
        self.rust_projects.insert(key, rust_project);
    }
}

impl Repository {
    pub fn new(owner: &str, name: &str, url: &str) -> Self {
        Self {
            owner: owner.to_string(),
            name: name.to_string(),
            url: url.to_string(),
        }
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