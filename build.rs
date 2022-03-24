use std::{
    env,
    path::{Path, PathBuf},
    // process::Command,
};

extern crate bindgen;

fn out_dir() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").unwrap())
}

fn build_tflite_c<P: AsRef<Path>>(tf_src_path: P) -> PathBuf {
    cmake::Config::new(tf_src_path)
        .define("TFLITE_C_BUILD_SHARED_LIBS", "OFF")
        // .define("CMAKE_OSX_ARCHITECTURES", "arm64")
        .build()
}

fn link_libs_c<P: AsRef<Path>>(target_dir: P) {
    let build_dir = target_dir.as_ref().join("build");
    let search_paths = vec![
        "ruy-build",
        "fft2d-build",
        "xnnpack-build",
        "farmhash-build",
        "flatbuffers-build",
    ];

    for p in search_paths {
        println!(
            "cargo:rustc-link-search=native={}",
            build_dir.join("_deps").join(p).display()
        );
    }

    let search_paths = vec!["pthreadpool", "cpuinfo", "tensorflow-lite", "clog"];
    for p in search_paths {
        println!(
            "cargo:rustc-link-search=native={}",
            build_dir.join(p).display()
        );
    }
}

fn main() {
    println!("cargo:rustc-link-lib=static=farmhash");
    println!("cargo:rustc-link-lib=static=ruy");
    println!("cargo:rustc-link-lib=static=cpuinfo");
    println!("cargo:rustc-link-lib=static=XNNPACK");
    println!("cargo:rustc-link-lib=static=tensorflow-lite");
    println!("cargo:rustc-link-lib=static=pthreadpool");
    println!("cargo:rustc-link-lib=static=fft2d_fftsg");
    println!("cargo:rustc-link-lib=static=flatbuffers");
    println!("cargo:rustc-link-lib=static=clog");
    println!("cargo:rustc-link-lib=static=fft2d_fftsg2d"); // println!("cargo:rustc-link-lib=tensorflowlite_c");
    
    if cfg!(target_os = "linux") {
      println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    // println!("cargo:rustc-link-lib=static=tensorflowlite_c");
    link_libs_c(build_tflite_c("tensorflow/tensorflow/lite/c"));
    // panic!();
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
