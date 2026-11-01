use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn main() {
    // workflow_run can expose main's GITHUB_SHA while building a develop tag.
    // The technical sheet must identify the checked-out product, not the workflow host.
    let hash = git(&["rev-parse", "HEAD"])
        .filter(|value| value.len() == 40 && value.bytes().all(|c| c.is_ascii_hexdigit()))
        .unwrap_or_else(|| "dev".to_owned());
    println!("cargo:rustc-env=BUILD_HASH={hash}");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=tauri.conf.json");
    if let Some(head) = git(&["rev-parse", "--git-path", "HEAD"]) {
        println!("cargo:rerun-if-changed={head}");
    }
    if let Some(branch) = git(&["symbolic-ref", "-q", "HEAD"]) {
        if let Some(reference) = git(&["rev-parse", "--git-path", &branch]) {
            println!("cargo:rerun-if-changed={reference}");
        }
    }
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    tauri_build::build();
}
