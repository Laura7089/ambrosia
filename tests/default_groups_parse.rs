use std::fs::read_dir;

use ambrosia::default_groups;

#[test]
fn default_groups_parses() {
    println!("{}", default_groups().len());
}

#[test]
// this is a flaky test but it's better than nothing
fn all_groups_parsed() {
    let num_group_files = read_dir("data/groups")
        .expect("failed to open data directory")
        .map(|e| e.unwrap())
        .filter(|e| e.file_type().unwrap().is_file())
        .count();

    assert!(default_groups().len() >= num_group_files);
}
