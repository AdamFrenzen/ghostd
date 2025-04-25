use std::path::PathBuf;
use std::process::Command;

fn main() {
    let llama_dir = PathBuf::from("vendor/llama.cpp");
    let build_dir = llama_dir.join("build");

    // Step 1: Configure
    let status = Command::new("cmake")
        .current_dir(&llama_dir)
        .args(["-B", "build"])
        .status()
        .expect("Failed to run cmake config");

    if !status.success() {
        panic!("CMake configuration failed");
    }

    // Step 2: Build llama-server
    let status = Command::new("cmake")
        .current_dir(&llama_dir)
        .args([
            "--build",
            "build",
            "--config",
            "Release",
            "-t",
            "llama-server",
        ])
        .status()
        .expect("Failed to build server");

    if !status.success() {
        panic!("CMake build failed");
    }

    // Step 3: Confirm llama-server exists
    let binary = build_dir.join("bin/llama-server");
    if !binary.exists() {
        panic!("Server binary not found!");
    }
}
