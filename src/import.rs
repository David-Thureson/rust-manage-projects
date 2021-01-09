#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use crate::model::*;
use std::{io, fs};
use std::fs::DirEntry;
use walkdir::WalkDir;
use std::path::Path;
use util::parse;
use std::collections::BTreeMap;

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
    //anic!();

    pc.add_project(project);
}

fn load_rust_project(project: &mut Project, path: &Path) {
    //bg!(path);
    let (name, mut dependencies) = parse_toml(&path);
    let mut rust_project = RustProject::new(&name, path.to_str().unwrap());
    dependencies.drain(..).for_each(|dependency| rust_project.add_dependency(dependency));
    //bg!(&rust_project);
    project.add_rust_project(rust_project);
}

fn parse_toml(path: &Path) -> (String, Vec<Dependency>) {
    let mut dependencies = vec![];
    let sections = parse::read_file_into_sections_by_line(path.join("Cargo.toml").to_str().unwrap(), "[", Some("]"));

    // "package" section.
    //bg!(file_path);
    let rust_project_name = if let Some(section) = sections.get("package") {
        let (name_value_pairs, _extra_lines) = parse::parse_name_value_pairs(section, "=", "#");
        //bg!(&name_value_pairs, &extra_lines);
        name_value_pairs.get("name").unwrap().clone()
    } else {
        // The Cargo.toml file has no "package" section so name the Rust project after the folder.
        path.file_name().unwrap().to_str().unwrap().to_string()
    };
    dbg!(path, &rust_project_name);
    //if rust_project_name.contains("conrod") {
        //anic!();
    //}

    // "dependencies" section.
    if let Some(lines) = sections.get("dependencies") {
        let (name_value_pairs, extra_lines) = parse::parse_name_value_pairs(lines, "=", "#");
        // assert!(extra_lines.is_empty());
        //if !extra_lines.is_empty() {
            //bg!(&extra_lines);
            //anic!();
        //}
        for (name, value) in name_value_pairs.iter() {
            let (version, is_local) = if value.starts_with("{") {
                if value.contains("path") {
                    (None, true)
                } else {
                    panic!("Unexpected [dependencies] value: \"{}\"", value);
                }
            } else {
                (Some(value.to_string()), false)
            };
            dependencies.push(Dependency::new(name, version, is_local));
        }
    }

    (rust_project_name, dependencies)
}

