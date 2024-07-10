extern crate cc;

fn main() {
    cc::Build::new()
        .file("/home/xxl/Rust/2024G-RT/RayTracer/src/stb_image_wrapper.c")
        .compile("stb_image_wrapper");
    // println!("cargo:rustc-link-lib=stb");
    println!("1");
}
