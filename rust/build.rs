fn main() {
    #[cfg(windows)]
    println!("cargo:rustc-link-lib=ws2_32");
}
