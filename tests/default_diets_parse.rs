use std::cell::LazyCell;
use std::fs::read_dir;

use ambrosia::DEFAULT_DIETS;

#[test]
fn default_diets_parses() {
    println!("{}", LazyCell::force(&DEFAULT_DIETS).len());
}

#[test]
// this is a flaky test but it's better than nothing
fn all_diets_parsed() {
    let num_diet_files = read_dir("data/diets")
        .expect("failed to open data directory")
        .map(|e| e.unwrap())
        .filter(|e| e.file_type().unwrap().is_file())
        .count();

    assert!(DEFAULT_DIETS.len() >= num_diet_files);
}
