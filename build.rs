use std::process::Command;

fn main() {
    let _ = Command::new("git").args(["add", "-A"]).status();

    // i do have a reason for this, when i correct the issue, it then says that i should use unwrap_or_else
    #[allow(clippy::map_unwrap_or)]
    let git_hash = Command::new("git")
        .args(["write-tree"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout[0..8]).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=GIT_HASH={git_hash}");
}
