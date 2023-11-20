use std::path::Path;
use fs_extra::dir::{self, CopyOptions};

fn main() {
    let source_path = Path::new("resources");
    let dest_path = Path::new("target/debug");

    let mut options = CopyOptions::new();
    options.overwrite = true;

    if source_path.exists() {
        dir::copy(source_path, dest_path, &options).expect("Failed to copy resources");
    } else {
        panic!("Source path does not exist");
    }
}
