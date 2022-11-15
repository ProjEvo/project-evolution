fn main() {
    #[cfg(not(windows))]
    println!("cargo:rustc-cfg=COMPILING_PLATFORM=\"UNIX\"");

    #[cfg(windows)]
    println!("cargo:rustc-cfg=COMPILING_PLATFORM=\"WINDOWS\"");
}
