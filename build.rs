fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    if os == "windows" && env == "msvc" && arch == "x86_64" {
        println!("cargo:rustc-link-arg-bins=/ENTRY:_start");
        println!("cargo:rustc-link-arg-bins=/SUBSYSTEM:CONSOLE");
    } else {
        println!("{os}-{env}-{arch}")
    }
}
