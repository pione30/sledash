use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

// https://github.com/rust-lang/rust-bindgen/issues/687#issuecomment-450750547
#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    let wand_config = pkg_config::Config::new()
        .atleast_version("6.9")
        .probe("MagickWand")
        .expect("pkg-config MagickWand should be probed");

    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INT_UPWARD".into(),
            "FP_INT_DOWNWARD".into(),
            "FP_INT_TOWARDZERO".into(),
            "FP_INT_TONEARESTFROMZERO".into(),
            "FP_INT_TONEAREST".into(),
            "FP_NAN".into(),
            "FP_INFINITE".into(),
            "FP_ZERO".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "IPPORT_RESERVED".into(),
        ]
        .into_iter()
        .collect(),
    );

    let mut bindgen_builder = bindgen::Builder::default()
        .header("wrapper.h")
        .rustfmt_bindings(true)
        .parse_callbacks(Box::new(ignored_macros));

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
