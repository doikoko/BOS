use std::{env::var, path::PathBuf};

fn main(){
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(&manifest_dir)
        .parent()
        .unwrap()
        .join("out");

    let loader_elf = PathBuf::from(&out_dir).join("loader.elf");
    let loader_ld = PathBuf::from(&manifest_dir).join("loader.ld");
    
    let link_args = [
        format!("-o{}", loader_elf.display()),
        format!("-T{}", loader_ld.display())
    ];

    for arg in link_args{
        println!("cargo:rustc-link-arg={arg}");
    }
}