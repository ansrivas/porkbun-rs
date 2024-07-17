use std::{io::Result, process::Command};

use chrono::{DateTime, SecondsFormat, Utc};

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    // build information
    let output = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .output()
        .unwrap();
    let git_tag = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_VERSION={git_tag}");

    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_commit = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_COMMIT_HASH={git_commit}");

    let now: DateTime<Utc> = Utc::now();
    let build_date = now.to_rfc3339_opts(SecondsFormat::Secs, true);
    println!("cargo:rustc-env=GIT_BUILD_DATE={build_date}");

    Ok(())
}