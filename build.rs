fn main() {
    println!("cargo:rustc-link-lib=gstnnmc");
    println!("cargo:rustc-link-search=/workspace/target/debug");
}
