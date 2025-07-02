use std::{env::var, path::PathBuf};

fn main(){
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    
    let linker_script  = PathBuf::from(&manifest_dir).join("kernel.ld");
    
    let link_args = [
        format!("-T{}", linker_script.display()),
        format!("-e_start"),
        format!("-o{}", PathBuf::from(&manifest_dir)
            .join("../").join("iso").join("boot")
            .join("kernel.elf").display())
    ];
    
    for arg in link_args{
        println!("cargo:rustc-link-arg={arg}");
    }
}