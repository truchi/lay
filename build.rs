fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning=Should appear only once");
}
