// build.rs

fn main() {
    // Only build on macOS
    if cfg!(target_os = "macos") {
        // Tell cargo to look for libraries in the specified directory
        println!("cargo:rustc-link-search=native=./objc");

        // Tell cargo to link against the frameworks we need
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AppKit");

        // Build our Objective-C code
        cc::Build::new()
            .file("objc/NSWorkspaceWrapper.m")
            .include("objc")
            .flag("-fobjc-arc") // Use ARC for memory management
            .compile("NSWorkspaceWrapper");

        // Watch our Objective-C files for changes
        println!("cargo:rerun-if-changed=objc/NSWorkspaceWrapper.h");
        println!("cargo:rerun-if-changed=objc/NSWorkspaceWrapper.m");
    } else {
        panic!("This crate is only supported on macOS");
    }
}