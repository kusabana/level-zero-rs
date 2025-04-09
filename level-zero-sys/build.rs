use std::path::PathBuf;
use std::{env, fs};

use bindgen::Builder;
use bindgen::callbacks::ParseCallbacks;
use cmake::Config;

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
    fn process_comment(&self, comment: &str) -> Option<String> {
        // This doesn't translate level-zero's doxygen comments perfectly, but it's better than nothing.
        match doxygen_bindgen::transform(comment) {
            Ok(res) => {
                // doxygen_bindgen doesn't handle the @details key
                let comment = res.replace("@details", "\n");
                Some(comment)
            }
            Err(err) => {
                println!("cargo:warning=Problem processing doxygen comment: {comment}\n{err}");
                None
            }
        }
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let level_zero_dir = manifest_dir.join("..").join("level-zero");

    let bindings = Builder::default()
        .header(manifest_dir.join("wrapper.h").display().to_string())
        .clang_arg(format!("-I{}", level_zero_dir.join("include").display()))
        .parse_callbacks(Box::new(Callbacks))
        .rustified_enum(".*")
        .generate()
        .expect("Failed to generate bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::write(out_dir.join("bindings.rs"), bindings.to_string()).expect("Failed to write bindings");

    let dest = Config::new(level_zero_dir)
        .define("BUILD_STATIC", "1")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dest.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=ze_loader");
    if cfg!(not(windows)) {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
