use std::path::{Path, PathBuf};
use std::{env, fs};

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}

fn main() {
    let target_dir = get_output_path();
    let src = Path::join(
        &env::current_dir().unwrap(),
        "scrolling_log_text_lines_sample.txt",
    );
    let dest = Path::join(
        Path::new(&target_dir),
        Path::new("scrolling_log_text_lines_sample.txt"),
    );
    fs::copy(src, dest).unwrap();
}
