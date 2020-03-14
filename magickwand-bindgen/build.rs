use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    let wand_config = pkg_config::Config::new()
        .atleast_version("6.9")
        .probe("MagickWand")
        .expect("pkg-config MagickWand should be probed");

    let mut bindgen_builder = bindgen::Builder::default()
        .header("wrapper.h")
        .rustfmt_bindings(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    for include_path in &wand_config.include_paths {
        bindgen_builder = bindgen_builder.clang_arg(format!("-I{}", include_path.display()));
    }

    let bindings = bindgen_builder
        .generate()
        .expect("Unable to generate MagickWand bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
