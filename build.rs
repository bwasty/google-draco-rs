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
}
