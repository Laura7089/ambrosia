use std::{cell::LazyCell, fs::read_dir};

use ambrosia::DEFAULT_GROUPS;

#[test]
fn default_groups_parses() {
    println!("{}", LazyCell::force(&DEFAULT_GROUPS).len());
}

#[test]
// this is a flaky test but it's better than nothing
fn all_groups_parsed() {
    let num_group_files = read_dir("data/groups")
        .expect("failed to open data directory")
        .map(|e| e.unwrap())
        .filter(|e| e.file_type().unwrap().is_file())
        .count();

    assert!(DEFAULT_GROUPS.len() >= num_group_files);
}
