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
    // import::test_file_search_time();
    // import::test_file_datetimes();
    println!("manage-projects: Done");
}

fn report_projects() {
    let model = import::build_model(true);
    //bg!(&model);
    model.report_summary();
    model.report_file_times();
}


