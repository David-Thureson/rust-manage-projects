pub struct Model {
    pub pcs: Vec<PC>,
}

pub struct PC {
    pub name: String,
    pub rust_root_folder: String,
    pub projects: Vec<Project>,
}

pub struct Project {
    pub name: String,
    pub path: String,
    pub rust_projects: Vec<RustProject>,
}

/*
struct IDEProject {
    name: String,
    crates: Vec<Crate>,
}
*/

pub struct RustProject {
    pub name: String,
    pub path: String,
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
            projects: vec![],
        }

    }
}

impl Project {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            rust_projects: vec![]
        }
    }
}
