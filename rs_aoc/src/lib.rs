pub mod days;

use std::path::PathBuf;

/// Resolve the path to an input file for a given day.
/// Looks for files in the `input/` directory relative to the project root.
pub fn input_path(day: u32, sample: bool) -> PathBuf {
    let filename = if sample {
        format!("day{:02}_sample.txt", day)
    } else {
        format!("day{:02}.txt", day)
    };
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("input")
        .join(filename)
}

/// Read the full input for a given day.
pub fn read_input(day: u32) -> String {
    let path = input_path(day, false);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Could not read input file: {}", path.display()))
}

/// Read the sample input for a given day.
pub fn read_sample_input(day: u32) -> String {
    let path = input_path(day, true);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Could not read sample input file: {}", path.display()))
}
