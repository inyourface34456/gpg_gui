use std::process::Command;

fn main() {
    let _ = Command::new("git").args(["add", "-A"]).status();

    let git_hash = Command::new("git")
        .args(["write-tree"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout[0..8]).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
