#![allow(unused_variables)]
#![allow(dead_code)]

use manage_projects::import;

fn main() {
    println!("manage-projects: Start");
    report_projects();
    println!("manage-projects: Done");
}

fn report_projects() {
    let model = import::build_model();
    dbg!(&model);
}


