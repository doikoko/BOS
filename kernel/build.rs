use std::{env::var, path::PathBuf};

fn main(){
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from("out");

    let linker_script  = PathBuf::from(&manifest_dir).join("kernel.ld");
    let kernel_elf = PathBuf::from(&manifest_dir)
        .parent()
        .unwrap()
        .join(out_dir)
        .join("kernel.elf");

    let link_args = [
        format!("-T{}", linker_script.display()),
        format!("-e_start"),
        format!("-o{}", kernel_elf.display())
    ];
    
    for arg in link_args{
        println!("cargo:rustc-link-arg={arg}");
    }
}