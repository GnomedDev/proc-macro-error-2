#[cfg(all(feature = "detect-nightly", not(feature = "nightly")))]
pub fn main() {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());

    let rustc_version = std::process::Command::new(rustc)
        .arg("--version")
        .output()
        .expect("Failed to get rustc version")
        .stdout;

    let rustc_version = String::from_utf8(rustc_version).expect("Failed to parse rustc version");
    if rustc_version.contains("nightly") {
        println!("cargo:rustc-cfg=detected_nightly");
    }
}

#[cfg(not(feature = "detect-nightly"))]
pub fn main() {
    // do nothing if the 'detect-nightly' feature is not enabled
}
