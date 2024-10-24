extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
  println!("OPT LEVEL {}", std::env::var("OPT_LEVEL").unwrap());

  cc::Build::new()
    .file("src/utils.c")
    .flag("-flto=thin")
    .flag("-emit-llvm")
    .compiler("clang")
    .compile("utils");
  
  let bindings = bindgen::Builder::default()
    .header("bindings.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
      .write_to_file(out_path.join("bindings.rs"))
      .expect("Couldn't write bindings!");
}
