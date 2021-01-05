#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use crate::model::*;
use std::{io, fs};
use std::fs::DirEntry;
use walkdir::WalkDir;
use std::path::Path;

const PC_MONSTER: &str = "Monster PC";
const RUST_ROOT_MONSTER: &str = r"C:\Projects\Rust";

const PC_DAVID: &str = "David PC";
const RUST_ROOT_DAVID: &str = r"T:\Projects\Rust";

pub fn build_model() -> Model {
    let pcs = vec![
        PC::new(PC_MONSTER, RUST_ROOT_MONSTER),
        PC::new(PC_DAVID, RUST_ROOT_DAVID)];
    let mut model = Model::new(pcs);
    for pc in model.pcs.iter_mut() {
        load_pc(pc).unwrap();
    }
    model
}

fn load_pc(pc: &mut PC) -> io::Result<()> {
    for category_path in fs::read_dir(&pc.rust_root_folder)?
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir()){
        //bg!(&category_path);

        for entry in fs::read_dir(category_path)?
            //.map(|entry| entry.unwrap().path())
            .filter(|entry| entry.as_ref().unwrap().path().is_dir()){

            //bg!(&entry?);
            load_project(pc, entry?)

        }
    }
    Ok(())
}

fn load_project(pc: &mut PC, entry: DirEntry) {
    //dbg!(&project_path)
    // let project = Project::new()
    dbg!(entry.file_name());
    let path =  entry.path().to_str().unwrap().to_string();
    let name = entry.file_name().into_string().unwrap();
    assert!(!name.starts_with("."), "\".\" file found in supposed project folder \"{}\".", path);

    let mut project = Project::new(&name, &name);
    for entry_recursive in WalkDir::new(path) {
        //rintln!("{}", entry.unwrap().path().display());
        let entry_recursive = entry_recursive.unwrap();
        if entry_recursive.file_name().eq_ignore_ascii_case("Cargo.toml") {
            load_rust_project(&mut project, entry_recursive.path().parent().unwrap());
        }
    }
    panic!();

    pc.projects.push(project);
}

fn load_rust_project(project: &mut Project, path: &Path) {
    dbg!(path);
}

