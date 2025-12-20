fn main() {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        // Get a path of Swift Runtime Library
        let path = String::from_utf8(
            Command::new("xcode-select")
                .args(["--print-path"])
                .output()
                .expect("failed to run xcode-select")
                .stdout,
        )
        .unwrap()
        .trim()
        .to_string();

        let status = Command::new("swift")
            .args(["build", "-c", "release"])
            .current_dir("../ztloop_ext/ZTLoopAppkit")
            .status()
            .expect("failed to build ZTLoopAppkit");

        if !status.success() {
            panic!("ztloop: Failed to build XCode project");
        }

        println!(
            "cargo:rustc-link-search=platform/ztloop_ext/ZTLoopAppkit/.build/arm64-apple-macosx/release/"
        );
        println!(
            "cargo:rustc-link-search={}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx",
            &path
        );
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx",
            &path
        );
        /*println!(
            "cargo:rustc-link-search={}/Toolchains/XcodeDefault.xctoolchain/usr/
         lib/swift-5.5/macosx",
            &path
        );
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift-5.5/macosx", &path);*/
        println!("cargo:rustc-link-search={}", "/usr/lib/swift");
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", "/usr/lib/swift");
        println!("cargo:rustc-link-lib=static=ZTLoopAppkit");
    }
}
