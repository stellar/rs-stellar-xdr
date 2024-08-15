pub fn main() {
    println!("cargo:rustc-check-cfg=cfg(docs)");
    crate_git_revision::init();
}
