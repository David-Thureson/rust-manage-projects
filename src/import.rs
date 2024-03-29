#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use crate::model::*;
use std::{io, fs};
use std::fs::DirEntry;
use walkdir::WalkDir;
use std::path::Path;
use util::{parse, date_time};
use std::time::Instant;
use itertools::Itertools;

const FILE_MODEL_SERIALIZED: &str = "Model.json";

const PC_MONSTER: &str = "Monster PC";
const RUST_ROOT_MONSTER: &str = r"C:\Projects\Rust";

// const PC_DAVID: &str = "David PC";
// const RUST_ROOT_DAVID: &str = r"T:\Projects\Rust";

pub fn build_model(force_rebuild: bool) -> Model {
    let path_model = Path::new(FILE_MODEL_SERIALIZED);
    let model = if force_rebuild || !path_model.exists() {
        let pcs = vec![
            PC::new(PC_MONSTER, RUST_ROOT_MONSTER)];
            // PC::new(PC_DAVID, RUST_ROOT_DAVID)];
        let mut model = Model::new(pcs);
        for pc in model.pcs.values_mut() {
            load_pc(pc).unwrap();
        }
        let json = serde_json::to_string(&model).unwrap();
        fs::write(path_model, json).ok();
        model
    } else {
        let json = fs::read_to_string(path_model).unwrap();
        serde_json::from_str(&json).unwrap()
    };
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
    //bg!(entry.file_name());
    let path =  entry.path().to_str().unwrap().to_string();
    let name = entry.file_name().into_string().unwrap();
    assert!(!name.starts_with("."), "\".\" file found in supposed project folder \"{}\".", path);

    let mut project = Project::new(&name, &path);

    load_repository(&mut project);

    // Find all of the Rust projects by looking for "Cargo.toml".
    for entry_recursive in WalkDir::new(&path)
        .into_iter()
        .filter_entry(|entry| !entry.file_name().eq("target"))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().eq_ignore_ascii_case("Cargo.toml"))
    {
        load_rust_project(&mut project, entry_recursive.path().parent().unwrap());
    }
    //anic!();

    // Find the first and last dates for Rust source files.
    let (it1, it2) = WalkDir::new(&path)
        .into_iter()
        .filter_entry(|entry| !entry.file_name().eq("target"))
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let file_name = entry.file_name().to_str().unwrap();
            //rintln!("{}", file_name);
            file_name.ends_with(".rs") || file_name.ends_with(".toml")
        })
        .filter_map(|entry| entry.metadata().ok())
        .tee();
    //let first_time = it1.map(|entry| std::cmp::min(entry.created().unwrap(), entry.modified().unwrap()))
    //bg!(&path);
    let first_time = it1.map(|entry| entry.modified().unwrap())
        .min()
        .unwrap();
    // let last_time = it2.map(|entry| std::cmp::max(entry.created().unwrap(), entry.modified().unwrap()))
    let last_time = it2.map(|entry| entry.modified().unwrap())
        .max()
        .unwrap();
    project.first_systime = Some(first_time);
    project.last_systime = Some(last_time);

    pc.add_project(project);
}

fn load_repository(project: &mut Project) {
    //bg!(&project);
    let path_git = Path::new(&project.path).join(".git");
    //bg!(&path_git);
    if path_git.exists() {
        let path_config = path_git.join("config");
        //bg!(&path_config);
        if path_config.exists() {
            let sections = parse::read_file_into_sections_by_line(path_config.to_str().unwrap(), "[", Some("]"));
            //bg!(&sections);
            if let Some(lines) = sections.get("remote \"origin\"") {
                let (name_value_pairs, _extra_lines) = parse::parse_name_value_pairs(lines, "=", None);
                let url = name_value_pairs.get("url").unwrap();
                let splits = url.split("/").collect::<Vec<_>>();
                let owner = splits[3];
                let name = parse::before(splits[4], ".git");
                //bg!(&url, &splits, &owner, &name);
                //if (project.name.eq_ignore_ascii_case("util")) {
                    //anic!();
                //}
                assert!(project.repository.is_none());
                project.repository = Some(Repository::new(owner, name, url));
            }
        }
    }
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
        let (name_value_pairs, _extra_lines) = parse::parse_name_value_pairs(section, "=", Some("#"));
        //bg!(&name_value_pairs, &extra_lines);
        name_value_pairs.get("name").unwrap().clone()
    } else {
        // The Cargo.toml file has no "package" section so name the Rust project after the folder.
        path.file_name().unwrap().to_str().unwrap().to_string()
    };
    //bg!(path, &rust_project_name);
    //if rust_project_name.contains("conrod") {
        //anic!();
    //}

    // "dependencies" section.
    //bg!(&path);
    if let Some(lines) = sections.get("dependencies") {
        let (name_value_pairs, extra_lines) = parse::parse_name_value_pairs(lines, "=", Some("#"));
        // assert!(extra_lines.is_empty());
        //if !extra_lines.is_empty() {
            //bg!(&extra_lines);
            //anic!();
        //}
        for (name, value) in name_value_pairs.iter() {
            // if value.contains("utility") {
                //bg!(&name, &value);
                //anic!();
            //}
            let (version, is_local) = if value.starts_with("{") {
                if value.contains("path") {
                    (None, true)
                } else {
                    let inner = parse::remove_delimiters(value, "{", "}");
                    let entries = inner
                        .split(",")
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>();
                    let (inner_name_value_pairs, _extra_lines) = parse::parse_name_value_pairs(&entries, "=", None);
                    if let Some(version) = inner_name_value_pairs.get("version") {
                        (Some(version.to_string()), false)
                    } else {
                        (None, false)
                    }
                    //anic!("Unexpected [dependencies] value: \"{}\"", value);
                }
            } else {
                (Some(value.to_string()), false)
            };
            dependencies.push(Dependency::new(name, version, is_local));
        }
    }

    (rust_project_name, dependencies)
}

pub fn test_file_search_time() {
    let path = r"C:\Projects\Rust\console";

    // Find all of the Rust projects by looking for "Cargo.toml" using WalkDir.
    let mut found1 = vec![];
    let start_time = Instant::now();
    for entry_recursive in WalkDir::new(path) {
        //rintln!("{}", entry.unwrap().path().display());
        let entry_recursive = entry_recursive.unwrap();
        if entry_recursive.file_name().eq_ignore_ascii_case("Cargo.toml") {
            found1.push(entry_recursive.path().to_str().unwrap().to_string());
        }
    }
    dbg!(Instant::now() - start_time);
    dbg!(&found1);

    let mut found2 = vec![];
    let start_time = Instant::now();
    for entry_recursive in WalkDir::new(path)
        .into_iter()
        .filter_entry(|entry| !entry.file_name().eq("target")) {

        //rintln!("{}", entry.unwrap().path().display());
        let entry_recursive = entry_recursive.unwrap();
        if entry_recursive.file_name().eq_ignore_ascii_case("Cargo.toml") {
            found2.push(entry_recursive.path().to_str().unwrap().to_string());
        }
    }
    dbg!(Instant::now() - start_time);
    dbg!(&found2);

    let mut found3 = vec![];
    let start_time = Instant::now();
    for entry_recursive in WalkDir::new(path)
        .into_iter()
        .filter_entry(|entry| !entry.file_name().eq("target"))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().eq_ignore_ascii_case("Cargo.toml")) {

        found3.push(entry_recursive.path().to_str().unwrap().to_string());
    }
    dbg!(Instant::now() - start_time);
    dbg!(&found3);

    // glob("**/*.png")

    // let mut found2 = vec![];
    // test_file_search_time_one(&mut found2, Path::new(path));

}

/*
fn test_file_search_time_one(found: &mut Vec<String>, path: &Path) {
    let path_cargo = path.join("Cargo.toml");
    if path_cargo.exists() {
        found.push(path_cargo.to_str().unwrap().to_string());
    } else {
        path.
    }
}
*/

pub fn test_file_datetimes() {
    let path = r"C:\Projects\Rust\console\adder";
    for entry in WalkDir::new(&path)
        .into_iter()
        .filter_entry(|entry| !entry.file_name().eq("target"))
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_name().to_str().unwrap().ends_with(".rs")) {

        let metadata = entry.metadata().unwrap();
        println!("{}: c = {}; m = {}",
                 entry.file_name().to_str().unwrap(),
                 date_time::systemtime_as_date(&metadata.created().unwrap()),
                 date_time::systemtime_as_date(&metadata.modified().unwrap()));
    }
}