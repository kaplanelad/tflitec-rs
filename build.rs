use std::{
    env,
    path::{Path, PathBuf},
};

extern crate bindgen;

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn main() {
    println!("cargo:rustc-link-lib=tensorflowlite_c");
    let out_path = out_dir();
    let tf_src_path = Path::new("./tensorflow");
    let mut builder = bindgen::Builder::default().header(
        tf_src_path
            .join("tensorflow/lite/c/c_api.h")
            .to_str()
            .unwrap(),
    );
    if cfg!(feature = "xnnpack") {
        builder = builder.header(
            tf_src_path
                .join("tensorflow/lite/delegates/xnnpack/xnnpack_delegate.h")
                .to_str()
                .unwrap(),
        );
    }

    let bindings = builder
        .clang_arg(format!("-I{}", tf_src_path.to_str().unwrap()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
