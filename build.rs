use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    // Builds the project in the directory located in `draco`, installing it into $OUT_DIR
    let dst = Config::new("draco")
        .always_configure(false)
        .define("BUILD_FOR_GLTF", "ON")
        .build();

    // TODO!: rerun-if-changed? / rerun-if-env-changed

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    // TODO!!: HACK...
    println!("cargo:rustc-link-search=native=/usr/local/Cellar/llvm/8.0.0/lib");

    println!("cargo:rustc-link-lib=static=dracodec"); // TODO: alternative: draco (includes encoding?)

    // from https://flames-of-code.netlify.com/blog/rust-and-cmake-cplusplus/
    let target  = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else {
        unimplemented!();
    }

    // requires nightly
    // println!("cargo:rustc-link-lib=static-nobundle=stdc++");

    if env::var("DRACO_REGENERATE_BINDINGS") == Ok("1".into()) {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let bindings = bindgen::Builder::default()
            .clang_args(&[
                "-x", "c++",
                "--std", "c++14",
                "-I", out_path.join("include").to_str().unwrap()
            ])
            .header(out_path.join("include/draco/compression/decode.h").to_str().unwrap())
            .whitelist_type("draco::DecoderBuffer")
            .whitelist_type("draco::Decoder")
            .opaque_type("std::.*")
            .disable_name_namespacing()
            .derive_default(true)
            // .trust_clang_mangling(false)
            .raw_line("#![allow(non_upper_case_globals)]")
            .raw_line("#![allow(non_camel_case_types)]")
            .raw_line("#![allow(non_snake_case)]")
            // ValueType is generated twice -> blacklist and write raw...
            .blacklist_type("ValueType")
            .raw_line("pub type ValueType = ::std::os::raw::c_uint;")
            .layout_tests(false) // TODO: some tests actually fail...
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file("src/bindings.rs")
            .expect("Couldn't write bindings!");
    }
}
