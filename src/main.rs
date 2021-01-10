#![allow(unused_variables)]
#![allow(dead_code)]

use manage_projects::import;

fn main() {
    println!("manage-projects: Start");
    /*
    let (a, b) = util::parse::split_1_or_2("util = { path = \"../../utility/util\" }", "=");
    let b = b.unwrap();
    dbg!(a, b);
    dbg!(util::parse::unquote(&b));
    */
    report_projects();
    println!("manage-projects: Done");
}

fn report_projects() {
    let model = import::build_model(false);
    //bg!(&model);
    model.report_summary();
}


