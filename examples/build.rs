fn main() {
    println!("cargo:rustc-link-lib=gstnnstreamermulticam");
    println!("cargo:rustc-link-search=/workspace/target/debug");
}
