use std::process::Command;
use std::str;

pub fn main() {
    crate_git_revision::init();

    if let Some(xdr_version) = if let Ok(output) = Command::new("git")
        .arg("submodule")
        .arg("status")
        .arg("--")
        .arg("xdr/curr")
        .output()
    {
        str::from_utf8(&output.stdout)
            .ok()
            .map(str::trim)
            .map(str::to_string)
    } else {
        None
    } {
        println!("cargo:rustc-env=XDR_CURR_VERSION={xdr_version}");
    }

    if let Some(xdr_version) = if let Ok(output) = Command::new("git")
        .arg("submodule")
        .arg("status")
        .arg("--")
        .arg("xdr/next")
        .output()
    {
        str::from_utf8(&output.stdout)
            .ok()
            .map(str::trim)
            .map(str::to_string)
    } else {
        None
    } {
        println!("cargo:rustc-env=XDR_NEXT_VERSION={xdr_version}");
    }
}
