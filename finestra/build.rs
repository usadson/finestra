// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

fn main() {
    #[cfg(all(target_os = "macos", test))]
    configure_xctest();
}

#[cfg(all(target_os = "macos", test))]
fn configure_xctest() {
    println!("cargo:rustc-link-lib=framework=XCTest");
    println!("cargo:rustc-link-search=framework=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/Library/Frameworks/");

    println!("cargo:rustc-env=DYLD_FRAMEWORK_PATH=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/Library/Frameworks/");
}
