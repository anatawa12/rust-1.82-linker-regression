use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::create_dir_all(&out_dir).unwrap();
    let object = out_dir.join("testlib.o");
    let archive = out_dir.join("testlib.a");

    Command::new("cc")
        .arg("-c")
        .arg("-o")
        .arg(&object)
        .arg("testlib.c")
        .status()
        .expect("running cc");

    Command::new("ar")
        .arg("rcs")
        .arg(&archive)
        .arg(&object)
        .status()
        .expect("running ar");


    println!(
        "cargo:rustc-link-search=native={path}",
        path = out_dir.display()
    );
    println!("cargo:rustc-link-lib=static:+verbatim=testlib.a");
}
