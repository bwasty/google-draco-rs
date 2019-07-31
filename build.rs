use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    // Builds the project in the directory located in `draco`, installing it into $OUT_DIR
    let dst = Config::new("draco")
        .define("BUILD_FOR_GLTF", "ON")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    // TODO!!: (optionally) only dracodec? might be smaller...
    // println!("cargo:rustc-link-lib=static=draco");
    println!("cargo:rustc-link-lib=static=dracodec");

    if env::var("REGENERATE_BINDINGS").is_ok() {
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let bindings = bindgen::Builder::default()
            .clang_arg("-x").clang_arg("c++")
            .clang_arg("--std").clang_arg("c++14")
            .clang_arg("-I").clang_arg(out_path.join("include").to_str().unwrap())
            .header(out_path.join("include/draco/core/decoder_buffer.h").to_str().unwrap())
            .header(out_path.join("include/draco/compression/decode.h").to_str().unwrap())
            // .header(out_path.join("include/draco/mesh/mesh.h").to_str().unwrap())
            .whitelist_type("draco::DecoderBuffer")
            .whitelist_type("draco::Decoder")
            // .whitelist_type("draco::Mesh")
            .opaque_type("std::.*")
            .disable_name_namespacing()
            .derive_default(true)
            .trust_clang_mangling(false)
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file("src/bindings.rs")
            .expect("Couldn't write bindings!");
    }
}
