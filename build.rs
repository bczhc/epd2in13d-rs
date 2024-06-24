use std::env;
use std::path::Path;

fn main() {
    let root_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:warning={root_dir}");
    println!("cargo:rustc-link-search=native={}", Path::new(&root_dir).join("libs").display());
}