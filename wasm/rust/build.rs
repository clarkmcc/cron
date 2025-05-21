use copy_dir::copy_dir;
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Get the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    // Build the Go Extism plugin
    println!("cargo:rerun-if-changed=../go/main.go");
    println!("cargo:rerun-if-changed=../go/go.mod");

    // Run make in the parent directory to build the Extism plugin
    let status = Command::new("make")
        .arg("build-plugin")
        .current_dir("..")
        .status()
        .expect("Failed to build Extism plugin");

    if !status.success() {
        panic!("Failed to build Extism plugin");
    }

    // Copy the WASM plugin to the output directory
    let wasm_build_dir = Path::new("../build");
    let wasm_file = wasm_build_dir.join("cron.wasm");

    // Create the destination directory
    let dest_dir = out_path.join("plugin");
    std::fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");

    // Copy the plugin file
    std::fs::copy(&wasm_file, dest_dir.join("cron.wasm"))
        .expect("Failed to copy WASM plugin");

    // Tell Cargo to tell rustc to link the plugin
    println!("cargo:rustc-env=CRON_PLUGIN_PATH={}", dest_dir.join("cron.wasm").display());
}
